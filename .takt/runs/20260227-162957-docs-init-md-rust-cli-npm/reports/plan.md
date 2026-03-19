# タスク計画

## 元の要求

`docs/init.md` の仕様に従い、Rust CLI + npm 配布構成をゼロから構築する。具体的には：
- Rust ワークスペース（`crates/core` + `crates/cli`）
- npm プラットフォームパッケージ（5パッケージ）
- TypeScript ランチャ（`packages/cli`）
- ルート設定更新（`mise.toml`, `turbo.json`）
- CI ワークフロー（GitHub Actions）
- Changesets 設定

## 分析結果

### 目的

Turborepo + pnpm モノレポに Rust CLI ツールと npm 配布基盤をゼロから追加する。ユーザーは `npm install -g @taiga-tech/cli` または `pnpm add @taiga-tech/cli` でインストールでき、プラットフォームに応じたネイティブバイナリが `optionalDependencies` 経由で配置される構成を構築する。

### 参照資料の調査結果

`docs/init.md` は存在しない（git log 確認済み）。タスク指示書が完全な仕様を包含しているため、タスク指示書をソース・オブ・トゥルースとして使用する。`docs/init.md` はこのタスクの成果物として新規作成する。

### スコープ

**新規作成ファイル（32ファイル）:**

| カテゴリ | ファイル数 | パス |
|---------|-----------|------|
| Rust ワークスペース | 5 | `Cargo.toml`, `crates/core/**`, `crates/cli/**` |
| npm プラットフォームパッケージ | 10 | `packages/cli-<5platform>/package.json` + `.gitkeep` |
| TypeScript ランチャ | 4 | `packages/cli/package.json`, `tsconfig.json`, `src/index.ts`, `src/platform.ts` |
| CI | 1 | `.github/workflows/release.yml` |
| Changesets | 1 | `.changeset/config.json` |
| ドキュメント | 1 | `docs/init.md` |

**変更ファイル（2ファイル）:**

| ファイル | 変更内容 |
|---------|---------|
| `turbo.json` | `outputs` に `"dist/**"` 追加、`test` タスク追加 |
| `mise.toml` | `[tools]` に `rust = "latest"` 追加 |

**変更不要（根拠あり）:**

| ファイル | 判定 | 根拠 |
|---------|------|------|
| `pnpm-workspace.yaml` | 不要 | `pnpm-workspace.yaml:2` の `'packages/*'` glob が `packages/cli*` を既にカバーしている |

### 検討したアプローチ

| 判断ポイント | アプローチ | 採否 | 理由 |
|------------|-----------|------|------|
| TypeScript ビルドツール | `tsup` | **採用** | `bin` が `./dist/index.cjs` を指す。`tsc --module CommonJS` の出力は `.js` であり `.cjs` にならない。`tsup --format cjs` はネイティブに `.cjs` を出力する |
| TypeScript ビルドツール | `tsc` | 不採用 | `.cjs` 拡張子の出力には後処理が必要でハック的。`module: CommonJS` との組み合わせで拡張子問題が生じる |
| テストフレームワーク | `vitest` | **採用** | TypeScript ファースト。ts-jest 不要。Node.js 24 + pnpm 10 環境と相性が良い |
| テストフレームワーク | `jest` + `ts-jest` | 不採用 | 設定が複雑、TypeScript 5.9 との互換性調整コストが高い |
| `tsconfig.json` の継承元 | standalone（継承なし） | **採用** | `base.json` の `module: NodeNext` + `moduleResolution: NodeNext` が `module: CommonJS` 設定と競合するため。タスク指示書が `module: CommonJS` を明示しているため |
| `tsconfig.json` の継承元 | `@taiga-tech/typescript-config/base` 継承 | 不採用 | `module: NodeNext` を上書きしても `moduleResolution` の競合が残り、TypeScript エラーを引き起こす |
| `platform.ts` の設計 | 純粋関数（引数でパラメータ受け取り） | **採用** | `resolvePackageName(platform, arch, isGlibc)` とすることで `process.platform` や `detect-libc` をモックせずにテスト可能 |
| `platform.ts` の設計 | `process.platform` を直接参照 | 不採用 | テスト時に `process.platform` のモックが必要になり複雑化 |

### 実装アプローチ

#### フェーズ 1: Rust ワークスペース（高優先度）

ルートに `Cargo.toml` を作成し、`crates/core`（library）と `crates/cli`（binary）を members として定義する。

- `crates/core`: `pub fn greet() -> String` のスタブ実装
- `crates/cli`: `sample_cli_core::greet()` を呼び出して print、`process::exit(0)` で終了
- バイナリ名 `sample-cli` は `crates/cli/Cargo.toml` の `[[bin]] name` で定義（Rust 側の唯一の定義）

#### フェーズ 2: npm プラットフォームパッケージ（高優先度）

5パッケージを `packages/cli-<platform>/` に作成。各パッケージ:

```
package.json   ← name/version/os/cpu/files 定義
vendor/<target-triple>/sample-cli/.gitkeep   ← CI がバイナリをここに配置
```

`package.json` には `publishConfig: { "access": "public" }` を含める。

#### フェーズ 3: TypeScript ランチャ（高優先度）

**`platform.ts`**（純粋関数モジュール）:
- `resolvePackageName(platform: string, arch: string, isGlibc: boolean): string` をエクスポート
- プラットフォーム→パッケージ名の変換を Map パターンで実装（`if-else` チェーンを避ける）
- 未対応プラットフォームは `Error` をスロー

**`index.ts`**（エントリポイント）:
- `const BINARY_NAME = 'sample-cli'` を定数として定義（TypeScript 内の唯一の定義）
- `detect-libc` の `familySync()` で glibc 検出（`=== 'glibc'` が `true` なら gnu）
- `resolvePackageName` でパッケージ名を決定
- `require.resolve('<pkg>/package.json')` でインストールパスを取得
- `vendor/<triple>/sample-cli/sample-cli(.exe)` パスを構築
- `spawnSync` で実行し `result.status ?? 1` を `process.exit` に渡す

**`package.json`**:
- `scripts.build`: `tsup src/index.ts --format cjs --outDir dist`
- `scripts.test`: `vitest run`
- `dependencies`: `detect-libc: "^2.0.4"`
- `devDependencies`: `tsup`, `typescript: 5.9.3`, `vitest`, `@types/node`
- `optionalDependencies`: 全5プラットフォームパッケージ `"0.1.0"`

**`tsconfig.json`**（standalone）:
```json
{
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

#### フェーズ 4: ルート設定更新（中優先度）

**`mise.toml`**: `[tools]` セクションに `rust = "latest"` を追加（既存の `node = "24.14.0"` の次行）

**`turbo.json`**:
- `build.outputs` を `[".next/**", "!.next/cache/**", "dist/**"]` に更新
- `test` タスクを追加: `{ "dependsOn": ["^build"] }`

#### フェーズ 5: CI / Changesets（低優先度）

**`.github/workflows/release.yml`**:
- トリガー: `push: tags: ["v*"]`
- `build` ジョブ: 5ターゲットのマトリクス
  1. `dtolnay/rust-toolchain@stable` + クロスコンパイルターゲット追加
  2. musl ターゲットのみ `musl-tools` インストール
  3. `cargo build --release --target <target>`
  4. バイナリを `packages/<platform-pkg>/vendor/<triple>/sample-cli/` にコピー
  5. `actions/upload-artifact@v4` でアーティファクト保存
- `publish` ジョブ: `needs: [build]`
  1. アーティファクト取得 → vendor ディレクトリへ配置
  2. `pnpm install && pnpm --filter @taiga-tech/cli build`
  3. プラットフォームパッケージ5個を先に publish
  4. ランチャ `@taiga-tech/cli` を後に publish（タスク指示書の publish 順序制約を保証）

**`.changeset/config.json`**:
- `linked`: 全6パッケージ（ランチャ + 5プラットフォーム）を1グループに設定
- `access: "public"`

## 実装ガイドライン

### 全体

- バイナリ名 `sample-cli` は各言語内で1箇所のみ定義する
  - TypeScript: `index.ts` の `const BINARY_NAME = 'sample-cli'`
  - Rust: `crates/cli/Cargo.toml` の `[[bin]] name = "sample-cli"`
- npm スコープは `@taiga-tech` で統一
- 全 npm パッケージのバージョンは `0.1.0` で統一

### `platform.ts`

- `resolvePackageName` は純粋関数とし、`process` オブジェクトや `detect-libc` に直接アクセスしない
- 未対応プラットフォームは `new Error('Unsupported platform: <value>')` をスロー（テストで検証可能にする）
- `if-else` チェーンではなく Map/Record パターンで分岐を実装（5分岐以上の switch/if-else は禁止）

### `index.ts`

- `detect-libc` の `familySync()` は Linux 以外では `null` を返す。`=== 'glibc'` の比較で `boolean` に変換する
- `spawnSync` を使用する（`spawn` + `on('close')` より同期的でシンプル）
- エラー時は `process.stderr.write` で出力し `process.exit(1)`

### テスト（write_tests フェーズ向け）

`packages/cli/src/platform.test.ts` として作成:

```typescript
import { describe, expect, it } from 'vitest'
import { resolvePackageName } from './platform'

describe('resolvePackageName', () => {
    it('darwin/arm64 → @taiga-tech/cli-darwin-arm64')
    it('darwin/x64 → @taiga-tech/cli-darwin-x64')
    it('win32/x64 → @taiga-tech/cli-win32-x64')
    it('linux/x64 + isGlibc=true → @taiga-tech/cli-linux-x64-gnu')
    it('linux/x64 + isGlibc=false → @taiga-tech/cli-linux-x64-musl')
    it('unsupported platform → throws Error')
})
```

### CI

- publish 順序を保証する: プラットフォームパッケージ5個の publish が完了してからランチャを publish
- musl ビルドには `apt-get install -y musl-tools` が必要
- `NODE_AUTH_TOKEN` は `secrets.NPM_TOKEN` から取得

### Changeset

- `linked` で全6パッケージを同一バージョンで管理（SemVer 同一バージョン運用の要件）

## スコープ外

| 項目 | 除外理由 |
|------|---------|
| `pnpm-workspace.yaml` の変更 | `packages/*` glob が `packages/cli*` を既にカバー済み（`pnpm-workspace.yaml:2`）。追加すると冗長なエントリになる |
| `pnpm add --no-optional` での動作保証 | タスク指示書に「保証不要」と明記されている |
| 後方互換コード | タスク指示書に明示的な指示なし。ゼロから構築のため不要 |

## 確認事項

なし（タスク指示書に全仕様が明記されており、コードを読んで解決できない不明点はない）