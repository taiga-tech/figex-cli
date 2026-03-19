# 変更スコープ宣言

## タスク
Rust CLI + npm 配布構成（Rust ワークスペース、npm プラットフォームパッケージ5個、TypeScript ランチャ）をゼロから構築する

## 変更予定
| 種別 | ファイル |
|------|---------|
| 作成 | `Cargo.toml` |
| 作成 | `crates/core/Cargo.toml` |
| 作成 | `crates/core/src/lib.rs` |
| 作成 | `crates/cli/Cargo.toml` |
| 作成 | `crates/cli/src/main.rs` |
| 作成 | `packages/cli-darwin-arm64/package.json` |
| 作成 | `packages/cli-darwin-arm64/vendor/aarch64-apple-darwin/sample-cli/.gitkeep` |
| 作成 | `packages/cli-darwin-x64/package.json` |
| 作成 | `packages/cli-darwin-x64/vendor/x86_64-apple-darwin/sample-cli/.gitkeep` |
| 作成 | `packages/cli-win32-x64/package.json` |
| 作成 | `packages/cli-win32-x64/vendor/x86_64-pc-windows-msvc/sample-cli/.gitkeep` |
| 作成 | `packages/cli-linux-x64-gnu/package.json` |
| 作成 | `packages/cli-linux-x64-gnu/vendor/x86_64-unknown-linux-gnu/sample-cli/.gitkeep` |
| 作成 | `packages/cli-linux-x64-musl/package.json` |
| 作成 | `packages/cli-linux-x64-musl/vendor/x86_64-unknown-linux-musl/sample-cli/.gitkeep` |
| 作成 | `packages/cli/package.json` |
| 作成 | `packages/cli/tsconfig.json` |
| 作成 | `packages/cli/src/platform.ts` |
| 作成 | `packages/cli/src/index.ts` |
| 変更 | `mise.toml` |
| 変更 | `turbo.json` |
| 作成 | `.github/workflows/release.yml` |
| 作成 | `.changeset/config.json` |

## 推定規模
Large

## 影響範囲
- Rust ワークスペース（`Cargo.toml`、`crates/`）
- npm プラットフォームパッケージ（`packages/cli-<platform>/`）
- TypeScript ランチャ（`packages/cli/`）
- ルート設定（`mise.toml`、`turbo.json`）
- CI/CD（`.github/workflows/release.yml`）
- バージョン管理（`.changeset/`）