# figex-cli

Figma のデザインデータを CLI から抽出・変換するツールです。

## インストール

```bash
npm install -g @taiga-tech/figex-cli
```

> **対応プラットフォーム:** macOS (arm64 / x64), Windows (x64), Linux (x64 glibc / musl)

## クイックスタート

```bash
# Figma runtime の接続を確認
figex doctor

# Figma runtime にアタッチ
figex attach
```

## コマンド一覧

### `figex attach` — Figma runtime に接続

選択された Figma runtime に接続し、セッション情報を表示します。
CDP transport のローカル確認は、現状 Chrome + Figma Web 版を使う手順を推奨します。

```bash
figex attach
figex attach --host 127.0.0.1 --port 9222
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

### `figex extract frame <FRAME_REF>` — フレームを抽出

指定フレームの生スナップショットを取得し、JSON ファイルに保存します。

```bash
figex extract frame "My Frame"
figex extract frame "My Frame" --output raw.json
```

| オプション            | 説明             | デフォルト |
| --------------------- | ---------------- | ---------- |
| `-o, --output <PATH>` | 出力ファイルパス | `raw.json` |

現状の `extract frame` は canonical な `raw.json` の保存フローを提供します。
`frame` には source metadata / runtime metadata が入り、`nodes` などの配列は runtime mapper の拡張前は空配列になることがあります。

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
Figma runtime
    ↓  attach / doctor（接続確認）
extract frame → raw.json
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
| `--pretty`            | JSON を整形出力する                                         | true                    |
| `--timeout-ms <MS>`   | 接続タイムアウト（ミリ秒）                                  | `5000`                  |
| `--transport <TYPE>`  | トランスポート種別（`cdp` / `mcp` / `auto`）                | `auto`                  |
| `--host <HOST>`       | 接続先ホスト                                                | `127.0.0.1`             |
| `--port <PORT>`       | 接続先ポート                                                | `9222`                  |
| `--config <PATH>`     | 設定ファイルのパス                                          | `figex.toml` を自動探索 |
| `--log-level <LEVEL>` | ログレベル（`error` / `warn` / `info` / `debug` / `trace`） | `warn`                  |

設定の優先順位: CLI フラグ > 環境変数 > `figex.toml` > デフォルト値

## 設定ファイル

プロジェクトルートに `figex.toml` を置くと設定を共有できます。

```toml
[runtime]
host = "127.0.0.1"
port = 9222
timeout_ms = 5000
transport = "auto"

[output]
pretty = true
```

## 注意点

- `pnpm add --no-optional` では platform package が入らず実行失敗する可能性があります
- 未対応 `platform/arch` はランチャーがエラー終了します
