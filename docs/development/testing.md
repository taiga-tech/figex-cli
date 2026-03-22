# テスト戦略

v1 のテストは「変換の安定性」と「配布の安定性」の 2 軸で組む。

## 1. Unit test

対象:

- extractor
- normalizer
- classifier
- runtime mapper
- runtime protocol
- runtime probe

単体テストでは、規則単位で壊れたときの原因がすぐ追える粒度を優先する。

## 2. Fixture test

`fixtures/` に次を置く。

- `raw/*.json`
- `features/*.json`
- `ui-ir/*.json`
- `reports/*.md`

同一入力に対して出力が揺れないことを snapshot / golden test で保証する。

## 3. Schema validation test

- `raw.json`
- `features.json`
- `ui.ir.json`

生成結果が対応する schema artifact に通ることを CI で保証する。

## 4. Runtime mock test

mock HTTP / WebSocket server を立て、次を検証する。

- endpoint discovery
- `Runtime.enable`
- `Runtime.evaluate`
- `send_request`
- `evaluate_string`
- `snapshot_frame`

## 5. Live runtime test

実機依存は ignored test にする。

```bash
FIGMA_RUNTIME_TEST=1 cargo test -p figex-cli-runtime -- --ignored
```

ここでは少なくとも次を確認する。

- attach
- doctor
- frame snapshot

### 5.1 事前準備: Chrome + Figma Web 版

Figma Desktop は現状 CDP ポートを公開しないため、Chrome に Figma Web 版を開いて代替する。

**1. デバッグポート付きで Chrome を起動する**

既存の Chrome を完全に終了してから起動すること。`open -a` はフラグを引き継がないため、バイナリを直接実行する。

```bash
killall "Google Chrome" 2>/dev/null
sleep 2
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --user-data-dir=/tmp/chrome-debug-profile &
```

**2. Figma ファイルを開く**

起動した Chrome で `figma.com` のデザインファイルを開く（ファイルが開いていないと target が見つからない）。

**3. CDP エンドポイントを確認する**

```bash
curl -s http://localhost:9222/json/version
# → {"Browser": "Chrome/...", "Protocol-Version": "1.3", ...} が返れば OK
```

### 5.2 CLI での動作確認

**`doctor` コマンド（JSON）**

```bash
./target/debug/figex-cli doctor --json
```

期待する出力:

```json
{
  "transport": "cdp",
  "host": "127.0.0.1",
  "port": 9222,
  "target_id": "...",
  "target_title": "... – Figma",
  "target_url": "https://www.figma.com/...",
  "target_score": 8,
  "ping": true,
  "snapshot": true,
  "latency_ms": ...,
  "warnings": [],
  "errors": []
}
```

**`attach` コマンド**

```bash
./target/debug/figex-cli attach
# → "Attached to Figma runtime" と接続情報が表示され exit code 0 で終了
```

**ignored テスト一括実行**

```bash
FIGMA_RUNTIME_TEST=1 cargo test -p figex-cli-runtime -- --ignored
```

## 6. Launcher / packaging test

- platform 解決
- gnu / musl 判定
- vendor path 解決
- 終了コードの伝播
- optionalDependencies 未解決時のエラーメッセージ

## 7. 回帰対象フレーム

- button-heavy frame
- card/list-heavy frame
- dashboard-like frame
- form-like frame
- modal/dialog frame
- empty state frame
