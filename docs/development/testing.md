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
FIGMA_RUNTIME_TEST=1 cargo test -- --ignored
```

ここでは少なくとも次を確認する。

- attach
- doctor
- frame snapshot

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
