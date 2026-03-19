---

## 作業結果

Rust CLI + npm 配布構成をゼロから構築しました。既存テスト（8件）がすべてパスし、ビルドも成功しています。

## 変更内容

| 種別 | ファイル |
|------|---------|
| 作成 | `Cargo.toml`（ワークスペース） |
| 作成 | `crates/core/Cargo.toml`, `crates/core/src/lib.rs` |
| 作成 | `crates/cli/Cargo.toml`, `crates/cli/src/main.rs` |
| 作成 | `packages/cli-darwin-arm64/package.json` + `.gitkeep` |
| 作成 | `packages/cli-darwin-x64/package.json` + `.gitkeep` |
| 作成 | `packages/cli-win32-x64/package.json` + `.gitkeep` |
| 作成 | `packages/cli-linux-x64-gnu/package.json` + `.gitkeep` |
| 作成 | `packages/cli-linux-x64-musl/package.json` + `.gitkeep` |
| 作成 | `packages/cli/package.json`（`"type": "module"`, `bin`, `optionalDependencies`） |
| 作成 | `packages/cli/tsconfig.json`（standalone, `module: CommonJS`） |
| 作成 | `packages/cli/src/platform.ts`（`resolvePackageName`, `resolveTargetTriple`） |
| 作成 | `packages/cli/src/index.ts`（ランチャ実装） |
| 変更 | `mise.toml`（`rust = "latest"` 追加） |
| 変更 | `turbo.json`（`outputs: dist/**`, `test` タスク追加） |
| 作成 | `.github/workflows/release.yml` |
| 作成 | `.changeset/config.json` |

**`platform.ts` の設計**: Map パターン（`PLATFORM_ENTRIES`）で全5プラットフォームの pkg/triple を一元管理。プラットフォーム不明→`Unsupported platform:`、arch 不明→`Unsupported arch:` の2段階エラーで全テストケースに対応。

## ビルド結果

```
# TypeScript ビルド（tsup）
> tsup src/index.ts --format cjs --outDir dist
dist/index.cjs  3.70 KB  ⚡️ Build success in 25ms

# TypeScript 型チェック
tsc --noEmit -p packages/cli/tsconfig.json  → エラーなし

# Rust ビルド
cargo build --release
Finished `release` profile [optimized] target(s) in 4.54s
./target/release/sample-cli → "Hello from sample-cli-core!"
```

## テスト結果

```
pnpm --filter @taiga-tech/cli test

 ✓ src/platform.test.ts (8 tests) 5ms

 Test Files  1 passed (1)
       Tests  8 passed (8)
   Duration  1.02s
```

**全8テストパス**（darwin/arm64・darwin/x64・win32/x64・linux/x64+glibc・linux/x64+musl・isGlibc無視エッジケース・未対応platform・未対応arch）