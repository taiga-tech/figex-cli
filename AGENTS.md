# リポジトリ運用ガイドライン

## プロジェクト構成とモジュール

- `crates/core`: Rust のコアライブラリ。
- `crates/cli`: Rust バイナリ本体（`sample-cli` のエントリポイント）。
- `packages/cli`: TypeScript 製ランチャー。プラットフォーム別パッケージを解決してネイティブバイナリを実行。
- `packages/cli-<platform>`: プラットフォーム別 npm パッケージ。`vendor/<target-triple>/sample-cli/` にバイナリを格納。
- `apps/web`, `apps/docs`: モノレポ内の Next.js アプリ（CLI 実行には必須ではない）。
- `scripts/place-binary-like-ci.sh`: CI と同じ配置規約でバイナリを配置するローカル補助スクリプト。

## ビルド・テスト・開発コマンド

基本は `mise` タスクを利用します。

- `mise run build`: JS パッケージと Rust ワークスペースをビルド。
- `mise run test`: `packages/cli` の Vitest と `cargo test --workspace` を実行。
- `mise run format`: Prettier と `cargo fmt` を実行。
- `mise run check`: lint + 型チェック + テストを一括実行。
- `mise run cli`: CI 互換のローカル CLI フロー（バイナリ配置 → ランチャービルド → 実行）。
- `TARGET=x86_64-unknown-linux-musl mise run cli-target`: 任意ターゲット向けに同フローを実行。

必要に応じて直接実行:

- `pnpm --filter @taiga-tech/cli run build`
- `node packages/cli/dist/index.cjs`
- `scripts/place-binary-like-ci.sh --build`

## コーディング規約・命名

- TypeScript: strict モード、インデント 4 スペース、シングルクォート、テスト名は `*.test.ts`。
- Rust: `cargo fmt` で整形、`cargo clippy --all-targets --all-features -- -D warnings` で lint。
- フォーマット: `mise run format` を使用（Prettier + `cargo fmt`）。
- パッケージ命名: プラットフォーム別は `@taiga-tech/cli-*`。ターゲットマッピングはコード上で明示。

## テスト方針

- JS テストは `packages/cli/src` で Vitest を使用（例: `platform.test.ts`）。
- Rust テストはワークスペース単位で実行（`cargo test --workspace`）。
- プラットフォーム解決やバイナリパス、ランチャー挙動を変更した場合は、Vitest ケースを追加・更新し `mise run test` を実行。

## コミット・PR ガイド

- コミットは Conventional Commits 準拠（例: `feat:`, `fix:`, `refactor:`）。
- 1コミット1目的を意識し、特に配布・プラットフォーム変更は理由を明記。
- PR には以下を記載:
- 変更内容と目的
- 実行した検証コマンド（例: `mise run check`, `mise run cli`, `mise run format`）
- ローカルで検証した target triple
- 公開挙動やバージョン影響がある変更では `.changeset/` に Changeset を追加。
