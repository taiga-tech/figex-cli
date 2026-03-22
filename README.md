# figex-cli

Figma のデザインデータを CLI から抽出・変換するツールです。

## インストール

```bash
npm install -g @taiga-tech/figex-cli
```

> **対応プラットフォーム:** macOS (arm64 / x64), Windows (x64), Linux (x64 glibc / musl)

## クイックスタート

```bash
# Figma Desktop の接続を確認
figex doctor

# Figma Desktop にアタッチ
figex attach
```

## コマンド一覧

### `figex attach` — Figma Desktop に接続

Figma Desktop ランタイムに接続し、セッション情報を表示します。

```bash
figex attach
figex attach --host localhost --port 18412
```

### `figex doctor` — 接続環境を診断

ポート探索・ping・スナップショット取得の疎通確認を行い、診断結果を表示します。

```bash
figex doctor
figex doctor --json --pretty
```

### `figex inspect frame <FRAME_REF>` — フレームのメタデータを確認 ⚠️ 未実装

指定フレームのノード数・テキスト数・レイアウト数などのメタデータをプレビューします。

```bash
figex inspect frame "My Frame"
figex inspect frame figma://...
```

### `figex extract frame <FRAME_REF>` — フレームを抽出 ⚠️ 未実装

指定フレームの生スナップショットを取得し、JSON ファイルに保存します。

```bash
figex extract frame "My Frame"
figex extract frame "My Frame" --output raw.json
```

| オプション            | 説明             | デフォルト |
| --------------------- | ---------------- | ---------- |
| `-o, --output <PATH>` | 出力ファイルパス | `raw.json` |

### `figex features` — 特徴量ファイルを生成 ⚠️ 未実装

`raw.json` を解析して `features.json` を生成します。

```bash
figex features
figex features --input raw.json --output features.json
```

| オプション            | 説明             | デフォルト      |
| --------------------- | ---------------- | --------------- |
| `-i, --input <PATH>`  | 入力ファイルパス | `raw.json`      |
| `-o, --output <PATH>` | 出力ファイルパス | `features.json` |

### `figex normalize` — UI 中間表現を生成 ⚠️ 未実装

`features.json` を正規化して `ui.ir.json`（UI 中間表現）を生成します。

```bash
figex normalize
figex normalize --input features.json --output ui.ir.json
```

| オプション            | 説明             | デフォルト      |
| --------------------- | ---------------- | --------------- |
| `-i, --input <PATH>`  | 入力ファイルパス | `features.json` |
| `-o, --output <PATH>` | 出力ファイルパス | `ui.ir.json`    |

### `figex report` — レポートを生成 ⚠️ 未実装

`ui.ir.json` から Markdown レポートを生成します。

```bash
figex report
figex report --input ui.ir.json --output report.md
```

| オプション            | 説明             | デフォルト   |
| --------------------- | ---------------- | ------------ |
| `-i, --input <PATH>`  | 入力ファイルパス | `ui.ir.json` |
| `-o, --output <PATH>` | 出力ファイルパス | `report.md`  |

## 処理パイプライン

```text
Figma Desktop
    ↓  attach / doctor（接続確認）
extract frame → raw.json          ← ⚠️ 未実装
    ↓  features
features.json                     ← ⚠️ 未実装
    ↓  normalize
ui.ir.json                        ← ⚠️ 未実装
    ↓  report
report.md                         ← ⚠️ 未実装
```

## グローバルフラグ

全コマンドで共通して使用できるフラグです。

| フラグ                | 説明                                                        | デフォルト              |
| --------------------- | ----------------------------------------------------------- | ----------------------- |
| `--json`              | 出力を JSON 形式にする                                      | false                   |
| `--pretty`            | JSON を整形出力する                                         | false                   |
| `--timeout-ms <MS>`   | 接続タイムアウト（ミリ秒）                                  | 設定ファイル依存        |
| `--transport <TYPE>`  | トランスポート種別（`cdp` / `mcp` / `auto`）                | `auto`                  |
| `--host <HOST>`       | 接続先ホスト                                                | `localhost`             |
| `--port <PORT>`       | 接続先ポート                                                | `18412`                 |
| `--config <PATH>`     | 設定ファイルのパス                                          | `figex.toml` を自動探索 |
| `--log-level <LEVEL>` | ログレベル（`error` / `warn` / `info` / `debug` / `trace`） | `warn`                  |

設定の優先順位: CLI フラグ > 環境変数 > `figex.toml` > デフォルト値

## 設定ファイル

プロジェクトルートに `figex.toml` を置くと設定を共有できます。

```toml
host = "localhost"
port = 18412
timeout_ms = 5000
transport = "auto"
log_level = "warn"
```

## 注意点

- `pnpm add --no-optional` では platform package が入らず実行失敗する可能性があります
- 未対応 `platform/arch` はランチャーがエラー終了します
