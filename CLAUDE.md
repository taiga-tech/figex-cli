# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

Rust 製ネイティブ CLI ツールを npm 経由で配布するハイブリッドモノレポ。TypeScript ランチャーがプラットフォームを自動判定し、対応するネイティブバイナリを実行する。

```
@taiga-tech/cli (TypeScript launcher)
    ↓ プラットフォーム判定
@taiga-tech/cli-<platform> (optional dependency)
    ↓
vendor/<target-triple>/sample-cli (native binary)
```

## コマンド

開発は `mise run *` を優先使用。

```bash
mise run build        # JS + Rust ワークスペースをビルド
mise run test         # Vitest + cargo test --workspace
mise run format       # Prettier + cargo fmt
mise run lint         # ESLint + cargo clippy -D warnings
mise run check        # lint + 型チェック + テスト一括
mise run cli          # CI互換フロー（バイナリ配置 → ランチャービルド → 実行）
TARGET=<triple> mise run cli-target  # 任意ターゲット向け同フロー
```

個別実行が必要な場合:

```bash
pnpm --filter @taiga-tech/cli run build
node packages/cli/dist/index.cjs
scripts/place-binary-like-ci.sh --build
cargo test --workspace
cargo clippy --all-targets --all-features -- -D warnings
```

## アーキテクチャ

**Rust クレート:**

- `crates/core` — コアライブラリ (`sample-cli-core`)
- `crates/cli` — バイナリエントリポイント、`core` に依存

**npm パッケージ:**

- `packages/cli` — TypeScript ランチャー（プラットフォーム解決・バイナリ起動）
- `packages/cli-darwin-arm64`, `cli-darwin-x64`, `cli-linux-x64-gnu`, `cli-linux-x64-musl`, `cli-win32-x64` — プラットフォーム別パッケージ。バイナリは `vendor/<target-triple>/sample-cli/` に配置

**補助:**

- `scripts/place-binary-like-ci.sh` — CI と同じ規約でバイナリをローカル配置
- `apps/web`, `apps/docs` — CLI 実行に不要な Next.js アプリ

## コーディング規約

**TypeScript:** strict モード、インデント 4 スペース、シングルクォート、テストファイルは `*.test.ts`

**Rust:** `cargo fmt` + `cargo clippy --all-targets --all-features -- -D warnings`

## コミット・PR

- Conventional Commits 準拠（`feat:`, `fix:`, `refactor:` 等）
- PR には変更内容・実行した検証コマンド・検証した target triple を記載
- 公開挙動やバージョン影響がある変更は `.changeset/` に Changeset を追加
