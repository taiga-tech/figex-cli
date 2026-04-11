# CLI 仕様

この文書は目標とする公開 CLI surface の正本である。
実装進捗の現状は README と testing / CI 文書を参照すること。

## 命名

- 実行コマンド: `figex`
- 設定ファイル: `figex.toml`
- 環境変数 prefix: `FIGEX_`

## コマンド体系

v1 の公開コマンドは次に固定する。

```bash
figex attach
figex doctor
figex inspect frame 123:456
figex extract frame 123:456 --output raw.json
figex features --input raw.json --output features.json
figex normalize --input features.json --output ui.ir.json
figex report --input ui.ir.json --output report.md
```

方針は次の通り。

- runtime に触るのは `attach`, `doctor`, `inspect`, `extract`
- 変換系は file-in / file-out に寄せる
- `report` は `ui.ir.json` を読むだけで、runtime 接続を前提にしない

現時点では `attach`, `doctor`, `extract frame` が先行実装されている。
`inspect`, `features`, `normalize`, `report` は公開 surface 上の予約済みコマンドで、実装は段階的に追加する。

## コマンド定義

### `attach`

- 選択された runtime transport へ接続できるかだけを確認する
- 成功時は transport, host, port, target 情報を出す
- `--json` で機械可読出力

### `doctor`

- port / endpoint discovery
- target scoring 内訳
- 選択された transport と attach 先
- ping 結果
- snapshot 最小試験
- timeout 設定値
- 失敗理由の分類

### `inspect frame <frame_ref>`

- 指定 frame の基本メタ情報だけを表示する
- `frame_ref` は frame id, deep link, または将来の selection alias を許可する
- node count, text count, layout count, repeated candidate count などを返す

### `extract frame <frame_ref>`

- runtime から Raw を取得して `raw.json` を保存する
- 既定出力先は `raw.json`

### `features`

- `raw.json` を読み、`features.json` を保存する
- 既定入力は `raw.json`
- 既定出力は `features.json`

### `normalize`

- `features.json` を読み、分類済みの `ui.ir.json` を保存する
- v1 では classifier を公開 surface から分離しない
- 既定入力は `features.json`
- 既定出力は `ui.ir.json`

### `report`

- `ui.ir.json` を読み、人間向け `report.md` を出力する
- 既定入力は `ui.ir.json`
- 既定出力は `report.md`

## グローバルオプション

- `--json`
- `--pretty`
- `--timeout-ms`
- `--transport cdp|mcp|auto`
- `--host`
- `--port`
- `--config <path>`
- `--log-level error|warn|info|debug|trace`

## 設定ファイル仕様

`figex.toml` を採用する。

```toml
[runtime]
transport = "auto"
host = "127.0.0.1"
port = 9222
timeout_ms = 5000
snapshot_max_depth = 128

[output]
pretty = true
include_diagnostics = true

[classifier]
enabled = true
max_component_candidates = 3
max_atomic_candidates = 3

[normalizer]
collapse_wrappers = true
boundary_threshold = 0.62
preserve_repetition_candidates = true
```

### 設定の優先順位

1. CLI 引数
2. 環境変数
3. `figex.toml`
4. 既定値

### 環境変数

- `FIGEX_TRANSPORT`
- `FIGEX_HOST`
- `FIGEX_PORT`
- `FIGEX_TIMEOUT_MS`
- `FIGEX_LOG_LEVEL`
- `FIGEX_CONFIG`
