これで調査が完了しました。計画を立てます。

## 調査結果サマリー

### docs/init.md について
`docs/init.md` は存在しない。タスク指示書の内容をソース・オブ・トゥルースとして使用する（指示書が事実上 init.md の内容を包含している）。`docs/init.md` は実装の一部として作成する。

### 現状確認

| 項目 | 状態 | 根拠 |
|------|------|------|
| `Cargo.toml`（workspace root） | 未存在 | git show HEAD 確認 |
| `crates/` | 未存在 | ls 確認 |
| `packages/cli*` | 未存在 | packages/ には eslint-config/typescript-config/ui のみ |
| `.github/workflows/` | 未存在 | ls 確認 |
| `.changeset/` | 未存在 | ls 確認 |
| `pnpm-workspace.yaml` の `packages/*` | 既存 | pnpm-workspace.yaml:2 — `packages/*` glob は `packages/cli*` を既にカバー |
| `mise.toml` の Rust ツールチェーン | 未設定 | `[tools]` に `node = "24.14.0"` のみ |

---

## 設計判断

### 1. TypeScript ビルドツール選定

**決定: `tsup`**

理由:
- `bin` フィールドが `./dist/index.cjs` を指している（CJS 拡張子が必須）
- `tsc --module CommonJS` の出力拡張子は `.js`（`.cjs` にならない）
- `tsup --format cjs` は `.cjs` ファイルをネイティブ出力する
- 既存パッケージにビルドツールの先例がないため、CJS 出力に最適な `tsup` を採用

### 2. テストフレームワーク選定

**決定: `vitest`**

理由:
- TypeScript ファースト（ts-jest 不要）
- pnpm モノレポと相性が良い
- 現プロジェクトの Node.js 24 + TypeScript 5.9 環境に最適

### 3. `platform.ts` の関数設計

`resolvePackageName(platform: string, arch: string, isGlibc: boolean): string` の純粋関数として設計。

- `process.platform`/`process.arch` へのアクセスを `index.ts` に集約
- テスト時にモックなしでパラメータを直接渡せる
- `detect-libc` の呼び出しも `index.ts` に置く

### 4. `BINARY_NAME` 定数の管理

TypeScript: `index.ts` で `const BINARY_NAME = 'sample-cli'` として定義（platform.ts はバイナリパス構築に関与しないため `index.ts` が適切）

Rust: `crates/cli/Cargo.toml` の `name = "sample-cli"` が唯一の定義

### 5. `pnpm-workspace.yaml` の変更要否

**変更不要** — `packages/*` glob が `packages/cli`、`packages/cli-darwin-arm64` 等を既にカバーしている（`pnpm-workspace.yaml:2`）

### 6. `turbo.json` の更新

`outputs` に `"dist/**"` を追加（`packages/cli` のビルド出力）。`test` タスクを新規追加。

### 7. `mise.toml` の Rust 設定

`[tools]` に `rust = "latest"` を追加。

---

## 実装計画

### 作成・変更ファイル一覧

#### 高優先度

**Rust クレート（新規作成）**

```
Cargo.toml                              # workspace 定義
crates/core/Cargo.toml                  # library crate: sample-cli-core
crates/core/src/lib.rs                  # pub fn greet() スタブ
crates/cli/Cargo.toml                   # binary crate: sample-cli, core依存
crates/cli/src/main.rs                  # greet()呼び出し + process::exit
```

**npm プラットフォームパッケージ（5パッケージ × 2ファイル = 10ファイル、新規作成）**

```
packages/cli-darwin-arm64/package.json
packages/cli-darwin-arm64/vendor/aarch64-apple-darwin/sample-cli/.gitkeep
packages/cli-darwin-x64/package.json
packages/cli-darwin-x64/vendor/x86_64-apple-darwin/sample-cli/.gitkeep
packages/cli-win32-x64/package.json
packages/cli-win32-x64/vendor/x86_64-pc-windows-msvc/sample-cli/.gitkeep
packages/cli-linux-x64-gnu/package.json
packages/cli-linux-x64-gnu/vendor/x86_64-unknown-linux-gnu/sample-cli/.gitkeep
packages/cli-linux-x64-musl/package.json
packages/cli-linux-x64-musl/vendor/x86_64-unknown-linux-musl/sample-cli/.gitkeep
```

**TypeScript ランチャ（新規作成）**

```
packages/cli/package.json
packages/cli/tsconfig.json
packages/cli/src/index.ts
packages/cli/src/platform.ts
```

#### 中優先度

**ルート設定変更**

```
mise.toml                               # [tools] に rust = "latest" 追加
turbo.json                              # outputs に "dist/**" 追加、test タスク追加
```

#### 低優先度

```
.github/workflows/release.yml           # 5ターゲット build matrix + publish
.changeset/config.json                  # linked versions 設定
```

#### ドキュメント

```
docs/init.md                            # このアーキテクチャの仕様書（タスク指示書の内容を構造化）
```

---

## 各ファイルの実装仕様

### `Cargo.toml` (workspace root)

```toml
[workspace]
members = ["crates/core", "crates/cli"]
resolver = "2"
```

### `crates/core/Cargo.toml`

```toml
[package]
name = "sample-cli-core"
version = "0.1.0"
edition = "2021"
```

### `crates/core/src/lib.rs`

```rust
pub fn greet() -> String {
    "Hello from sample-cli-core!".to_string()
}
```

### `crates/cli/Cargo.toml`

```toml
[package]
name = "sample-cli"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "sample-cli"
path = "src/main.rs"

[dependencies]
sample-cli-core = { path = "../core" }
```

### `crates/cli/src/main.rs`

```rust
use sample_cli_core::greet;
use std::process;

fn main() {
    println!("{}", greet());
    process::exit(0);
}
```

---

### プラットフォームパッケージ `package.json` パターン（5パッケージ共通）

`@taiga-tech/cli-darwin-arm64` の例:
```json
{
    "name": "@taiga-tech/cli-darwin-arm64",
    "version": "0.1.0",
    "os": ["darwin"],
    "cpu": ["arm64"],
    "files": ["vendor/**"],
    "publishConfig": {
        "access": "public"
    }
}
```

| パッケージ名 | os | cpu | target triple |
|---|---|---|---|
| `@taiga-tech/cli-darwin-arm64` | `["darwin"]` | `["arm64"]` | `aarch64-apple-darwin` |
| `@taiga-tech/cli-darwin-x64` | `["darwin"]` | `["x64"]` | `x86_64-apple-darwin` |
| `@taiga-tech/cli-win32-x64` | `["win32"]` | `["x64"]` | `x86_64-pc-windows-msvc` |
| `@taiga-tech/cli-linux-x64-gnu` | `["linux"]` | `["x64"]` | `x86_64-unknown-linux-gnu` |
| `@taiga-tech/cli-linux-x64-musl` | `["linux"]` | `["x64"]` | `x86_64-unknown-linux-musl` |

---

### `packages/cli/package.json`

```json
{
    "name": "@taiga-tech/cli",
    "version": "0.1.0",
    "bin": {
        "sample-cli": "./dist/index.cjs"
    },
    "scripts": {
        "build": "tsup src/index.ts --format cjs --outDir dist",
        "test": "vitest run",
        "check-types": "tsc --noEmit"
    },
    "dependencies": {
        "detect-libc": "^2.0.4"
    },
    "devDependencies": {
        "@types/node": "^22.0.0",
        "tsup": "^8.4.0",
        "typescript": "5.9.3",
        "vitest": "^3.0.0"
    },
    "optionalDependencies": {
        "@taiga-tech/cli-darwin-arm64": "0.1.0",
        "@taiga-tech/cli-darwin-x64": "0.1.0",
        "@taiga-tech/cli-win32-x64": "0.1.0",
        "@taiga-tech/cli-linux-x64-gnu": "0.1.0",
        "@taiga-tech/cli-linux-x64-musl": "0.1.0"
    },
    "files": ["dist"],
    "publishConfig": {
        "access": "public"
    }
}
```

### `packages/cli/tsconfig.json`

`base.json` の `module: NodeNext` と `moduleResolution: NodeNext` が CommonJS 設定と競合するため、`base.json` を extend しない standalone 設定とする。

```json
{
    "$schema": "https://json.schemastore.org/tsconfig",
    "compilerOptions": {
        "target": "ES2022",
        "module": "CommonJS",
        "moduleResolution": "Node10",
        "strict": true,
        "esModuleInterop": true,
        "skipLibCheck": true,
        "resolveJsonModule": true,
        "outDir": "dist",
        "rootDir": "src"
    },
    "include": ["src"]
}
```

### `packages/cli/src/platform.ts`

```typescript
export const BINARY_SCOPE = '@taiga-tech'

const PACKAGE_MAP: Record<string, Record<string, { gnu: string; musl?: string }>> = {
    darwin: {
        arm64: { gnu: 'cli-darwin-arm64' },
        x64:   { gnu: 'cli-darwin-x64'   },
    },
    win32: {
        x64:   { gnu: 'cli-win32-x64'    },
    },
    linux: {
        x64:   { gnu: 'cli-linux-x64-gnu', musl: 'cli-linux-x64-musl' },
    },
}

export function resolvePackageName(
    platform: string,
    arch: string,
    isGlibc: boolean,
): string {
    const archMap = PACKAGE_MAP[platform]
    if (!archMap) throw new Error(`Unsupported platform: ${platform}`)
    const entry = archMap[arch]
    if (!entry) throw new Error(`Unsupported arch: ${platform}/${arch}`)
    
    const variant = platform === 'linux' && !isGlibc && entry.musl
        ? entry.musl
        : entry.gnu
    
    return `${BINARY_SCOPE}/${variant}`
}
```

### `packages/cli/src/index.ts`

```typescript
import { familySync } from 'detect-libc'
import { spawnSync } from 'node:child_process'
import { join } from 'node:path'
import { resolvePackageName } from './platform'

const BINARY_NAME = 'sample-cli'

function getBinaryPath(packageName: string, targetTriple: string): string {
    const pkgRoot = require.resolve(`${packageName}/package.json`)
    const vendorDir = join(pkgRoot, '..', 'vendor', targetTriple, BINARY_NAME)
    const isWindows = process.platform === 'win32'
    return join(vendorDir, isWindows ? `${BINARY_NAME}.exe` : BINARY_NAME)
}

function getTargetTriple(packageName: string): string {
    // package name suffix IS the target triple segment → derive from package name
    const suffix = packageName.replace('@taiga-tech/cli-', '')
    const tripleMap: Record<string, string> = {
        'darwin-arm64':  'aarch64-apple-darwin',
        'darwin-x64':    'x86_64-apple-darwin',
        'win32-x64':     'x86_64-pc-windows-msvc',
        'linux-x64-gnu': 'x86_64-unknown-linux-gnu',
        'linux-x64-musl':'x86_64-unknown-linux-musl',
    }
    const triple = tripleMap[suffix]
    if (!triple) throw new Error(`Unknown platform package: ${packageName}`)
    return triple
}

function main(): void {
    const isGlibc = familySync() === 'glibc'
    const pkgName = resolvePackageName(process.platform, process.arch, isGlibc)
    const triple = getTargetTriple(pkgName)
    const binaryPath = getBinaryPath(pkgName, triple)

    const result = spawnSync(binaryPath, process.argv.slice(2), { stdio: 'inherit' })
    if (result.error) {
        process.stderr.write(`Failed to execute ${BINARY_NAME}: ${result.error.message}\n`)
        process.exit(1)
    }
    process.exit(result.status ?? 1)
}

main()
```

---

### `turbo.json` 更新差分

```json
"outputs": [".next/**", "!.next/cache/**", "dist/**"]
```
+ `test` タスク追加:
```json
"test": {
    "dependsOn": ["^build"]
}
```

### `mise.toml` 更新差分

```toml
[tools]
node = "24.14.0"
rust = "latest"
```

---

### `.github/workflows/release.yml` の骨格

- トリガー: `push: tags: ["v*"]`
- `build` ジョブ: 5ターゲットのマトリクス実行
  1. `dtolnay/rust-toolchain@stable` でターゲット追加
  2. musl ターゲットは `musl-tools` インストール
  3. `cargo build --release --target <target>`
  4. バイナリを `packages/<platform-pkg>/vendor/<triple>/sample-cli/` にコピー
  5. `actions/upload-artifact@v4` でアーティファクト保存
- `publish` ジョブ: `needs: build`
  1. アーティファクト取得 → vendorディレクトリへ配置
  2. `pnpm install && pnpm --filter @taiga-tech/cli build`
  3. プラットフォームパッケージ5個を先に publish
  4. ランチャ `@taiga-tech/cli` を後に publish（順序保証）

### `.changeset/config.json`

```json
{
    "$schema": "https://unpkg.com/@changesets/config/schema.json",
    "changelog": "@changesets/cli/changelog",
    "commit": false,
    "linked": [[
        "@taiga-tech/cli",
        "@taiga-tech/cli-darwin-arm64",
        "@taiga-tech/cli-darwin-x64",
        "@taiga-tech/cli-win32-x64",
        "@taiga-tech/cli-linux-x64-gnu",
        "@taiga-tech/cli-linux-x64-musl"
    ]],
    "access": "public",
    "baseBranch": "main",
    "updateInternalDependencies": "patch",
    "ignore": []
}
```

---

## テスト仕様（write_tests フェーズ向け）

`packages/cli/src/platform.test.ts`:

```typescript
import { describe, expect, it } from 'vitest'
import { resolvePackageName } from './platform'

describe('resolvePackageName', () => {
    it('darwin/arm64 → cli-darwin-arm64', () => {
        expect(resolvePackageName('darwin', 'arm64', false))
            .toBe('@taiga-tech/cli-darwin-arm64')
    })
    it('darwin/x64 → cli-darwin-x64', () => {
        expect(resolvePackageName('darwin', 'x64', false))
            .toBe('@taiga-tech/cli-darwin-x64')
    })
    it('win32/x64 → cli-win32-x64', () => {
        expect(resolvePackageName('win32', 'x64', false))
            .toBe('@taiga-tech/cli-win32-x64')
    })
    it('linux/x64 + glibc → cli-linux-x64-gnu', () => {
        expect(resolvePackageName('linux', 'x64', true))
            .toBe('@taiga-tech/cli-linux-x64-gnu')
    })
    it('linux/x64 + no glibc → cli-linux-x64-musl', () => {
        expect(resolvePackageName('linux', 'x64', false))
            .toBe('@taiga-tech/cli-linux-x64-musl')
    })
    it('unsupported platform → throw', () => {
        expect(() => resolvePackageName('freebsd', 'x64', false))
            .toThrow('Unsupported platform: freebsd')
    })
})
```

---

## 変更要否の判定

| 要件 | 判定 | 根拠 |
|------|------|------|
| `pnpm-workspace.yaml` に `packages/cli*` 追加 | **不要** | `pnpm-workspace.yaml:2` の `packages/*` glob が既に全 `packages/` サブディレクトリをカバー |
| `turbo.json` の build outputs 更新 | **必要** | 現状 `outputs` に `dist/**` がなく、`packages/cli` のビルド成果物がキャッシュされない |
| `mise.toml` に Rust 追加 | **必要** | `[tools]` に `rust` エントリが存在しない |
| Rust crates | **必要** | `Cargo.toml`/`crates/` ともに未存在 |
| npm プラットフォームパッケージ | **必要** | `packages/cli-*` 全て未存在 |
| TypeScript ランチャ | **必要** | `packages/cli/` 未存在 |
| CI ワークフロー | **必要** | `.github/workflows/` 未存在 |
| Changesets 設定 | **必要** | `.changeset/` 未存在 |