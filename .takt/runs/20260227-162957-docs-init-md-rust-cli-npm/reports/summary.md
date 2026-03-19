# タスク完了サマリー

## タスク
`docs/init.md` の仕様に従い、Rust CLI（`crates/core` + `crates/cli`）と npm 配布構成（5プラットフォームパッケージ + TypeScript ランチャ `@taiga-tech/cli`）をゼロから構築した。

## 結果
完了

## 変更内容
| 種別 | ファイル | 概要 |
|------|---------|------|
| 作成 | `Cargo.toml` | Rust ワークスペース定義（members: crates/core, crates/cli） |
| 作成 | `crates/core/Cargo.toml` | library crate `sample-cli-core` 定義 |
| 作成 | `crates/core/src/lib.rs` | `pub fn greet() -> String` スタブ実装 |
| 作成 | `crates/cli/Cargo.toml` | binary crate `sample-cli` 定義、core 依存 |
| 作成 | `crates/cli/src/main.rs` | `sample_cli_core::greet()` 呼び出し、標準出力 |
| 作成 | `packages/cli-darwin-arm64/package.json` | プラットフォームパッケージ（os:darwin, cpu:arm64） |
| 作成 | `packages/cli-darwin-arm64/vendor/aarch64-apple-darwin/sample-cli/.gitkeep` | バイナリ配置先ディレクトリ |
| 作成 | `packages/cli-darwin-x64/package.json` | プラットフォームパッケージ（os:darwin, cpu:x64） |
| 作成 | `packages/cli-darwin-x64/vendor/x86_64-apple-darwin/sample-cli/.gitkeep` | バイナリ配置先ディレクトリ |
| 作成 | `packages/cli-win32-x64/package.json` | プラットフォームパッケージ（os:win32, cpu:x64） |
| 作成 | `packages/cli-win32-x64/vendor/x86_64-pc-windows-msvc/sample-cli/.gitkeep` | バイナリ配置先ディレクトリ |
| 作成 | `packages/cli-linux-x64-gnu/package.json` | プラットフォームパッケージ（os:linux, cpu:x64, glibc） |
| 作成 | `packages/cli-linux-x64-gnu/vendor/x86_64-unknown-linux-gnu/sample-cli/.gitkeep` | バイナリ配置先ディレクトリ |
| 作成 | `packages/cli-linux-x64-musl/package.json` | プラットフォームパッケージ（os:linux, cpu:x64, musl） |
| 作成 | `packages/cli-linux-x64-musl/vendor/x86_64-unknown-linux-musl/sample-cli/.gitkeep` | バイナリ配置先ディレクトリ |
| 作成 | `packages/cli/package.json` | ランチャパッケージ（bin: sample-cli, optionalDependencies: 全5プラットフォーム） |
| 作成 | `packages/cli/tsconfig.json` | TypeScript 設定（module: CommonJS, target: ES2022） |
| 作成 | `packages/cli/tsup.config.ts` | tsup ビルド設定（format: cjs, outExtension: .cjs, clean: true） |
| 作成 | `packages/cli/src/index.ts` | ランチャ実装（platform検出・require.resolve・spawnSync・終了コード伝播） |
| 作成 | `packages/cli/src/platform.ts` | プラットフォーム判定純粋関数（resolvePackageName / resolveTargetTriple） |
| 作成 | `packages/cli/src/platform.test.ts` | 単体テスト 13件（必須6ケース + エッジケース7件） |
| 作成 | `.github/workflows/release.yml` | 5ターゲットビルドマトリクス + npm publish ワークフロー |
| 作成 | `.changeset/config.json` | 全6パッケージ linked バージョン管理設定 |
| 変更 | `mise.toml` | `rust = "latest"` 追加 |
| 変更 | `turbo.json` | `build.outputs` に `dist/**` 追加、`test` タスク追加 |

## 確認コマンド
```bash
# Rust ビルド
cargo build --release

# TypeScript ランチャビルド
pnpm --filter @taiga-tech/cli build

# 型チェック
pnpm --filter @taiga-tech/cli tsc --noEmit

# テスト実行（13 passed）
pnpm --filter @taiga-tech/cli test
```