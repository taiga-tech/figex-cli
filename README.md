# sample-cli

Rust 製ネイティブ CLI を npm で配布するためのモノレポです。
`@taiga-tech/cli`（TypeScript ランチャ）が実行環境を判定し、対応するプラットフォーム別パッケージに同梱されたネイティブバイナリを起動します。

## 構成概要

- `crates/core`: Rust コアライブラリ
- `crates/cli`: Rust バイナリ本体（`sample-cli`）
- `packages/cli`: npm 配布用ランチャ（`sample-cli` コマンド）
- `packages/cli-<platform>`: プラットフォーム別バイナリ同梱パッケージ
- `scripts/place-binary-like-ci.sh`: CI と同じ規約で `vendor/` にバイナリを配置

## サポートプラットフォーム

| Node 判定           | Rust target triple          | npm package                      |
| ------------------- | --------------------------- | -------------------------------- |
| `darwin/arm64`      | `aarch64-apple-darwin`      | `@taiga-tech/cli-darwin-arm64`   |
| `darwin/x64`        | `x86_64-apple-darwin`       | `@taiga-tech/cli-darwin-x64`     |
| `win32/x64`         | `x86_64-pc-windows-msvc`    | `@taiga-tech/cli-win32-x64`      |
| `linux/x64` + glibc | `x86_64-unknown-linux-gnu`  | `@taiga-tech/cli-linux-x64-gnu`  |
| `linux/x64` + musl  | `x86_64-unknown-linux-musl` | `@taiga-tech/cli-linux-x64-musl` |

## 要件

- Node.js `>= 24`
- pnpm `10.30.3`
- Rust `1.93.1`
- `mise`（推奨。`mise.toml` のツール定義を利用）

## セットアップ

```bash
mise install
pnpm install
```

`mise` を使わない場合は、必要バージョンの Node/pnpm/Rust を手動で用意してください。

## よく使うコマンド

```bash
# JS パッケージ + Rust ワークスペースをビルド
mise run build

# JS (Vitest) + Rust テスト
mise run test

# Lint + 型チェック + テスト
mise run check

# Prettier + cargo fmt
mise run format

# CI 互換フローでローカル実行（バイナリ配置 -> ランチャービルド -> 実行）
mise run cli

# 任意ターゲット向け CI 互換フロー
TARGET=x86_64-unknown-linux-musl mise run cli-target
```

## ローカルで CLI を直接動かす

```bash
# 1) ホスト向け Rust バイナリをビルドして vendor に配置
scripts/place-binary-like-ci.sh --build

# 2) ランチャーをビルド
pnpm --filter @taiga-tech/cli run build

# 3) ランチャーを実行
node packages/cli/dist/index.cjs
```

## バイナリ配置規約

プラットフォーム別パッケージには次のパスでバイナリを格納します。

```text
packages/cli-*/vendor/<target-triple>/sample-cli/sample-cli(.exe)
```

この規約は `scripts/place-binary-like-ci.sh` に実装されています。

## テスト方針

- TypeScript: `packages/cli/src/*.test.ts`（Vitest）
- Rust: `cargo test --workspace`
- プラットフォーム判定やバイナリパス解決を変更した場合は、Vitest ケースを追加・更新してください

## リポジトリ構成（抜粋）

```text
.
├── crates/
│   ├── core/
│   └── cli/
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
├── scripts/
│   └── place-binary-like-ci.sh
└── mise.toml
```

## 変更管理とリリース

- コミットメッセージは Conventional Commits（`feat:`, `fix:`, `refactor:` など）
- 公開挙動やバージョンに影響する変更は `.changeset/` を追加
- publish 順序は platform package を先、`@taiga-tech/cli` を後

## 注意点

- `pnpm add --no-optional` では platform package が入らず実行失敗する可能性があります
- 未対応 `platform/arch` はランチャーがエラー終了します
