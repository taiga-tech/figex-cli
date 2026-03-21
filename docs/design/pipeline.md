# パイプライン設計

## 1. runtime 設計

### 公開境界

```rust
#[async_trait]
pub trait RuntimeClient: Send + Sync {
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError>;
    async fn health(&self) -> Result<RuntimeHealth, RuntimeError>;
    async fn snapshot_frame(
        &self,
        request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError>;
}
```

外から見せるのはこの 3 操作だけである。
`eval(String)` は内部実装に閉じる。

### transport adapter

```text
runtime/
├── lib.rs
├── client.rs
├── error.rs
├── health.rs
├── snapshot.rs
└── transport/
    ├── mod.rs
    ├── cdp/
    │   ├── client.rs
    │   ├── protocol.rs
    │   ├── probe.rs
    │   └── js_bridge.rs
    └── mcp/
        ├── client.rs
        ├── protocol.rs
        └── mapper.rs
```

### CDP adapter

#### 役割

- `/json/version` `/json/list` 探索
- WebSocket attach
- `Runtime.enable`
- `Runtime.evaluate`
- 応答待機と request id 管理
- ping / snapshot 実行

#### `probe.rs`

- port range 探索
- target 列挙
- score 付与
- best target 選択

#### `protocol.rs`

- CDP request / response 型
- Evaluate params / result
- Runtime exception 変換

#### `js_bridge.rs`

- runtime へ流す JavaScript 生成
- ping script
- snapshot script
- script version 管理

### MCP adapter

初版は CDP を優先してもよいが、transport interface は最初から `mcp` を受け入れる形にする。

Figma 公式ドキュメントでは、2026-03-22 時点で次の 2 つが公開されている。

- desktop server: `http://127.0.0.1:3845/mcp`
- remote server: `https://mcp.figma.com/mcp`

desktop server は Figma desktop app 上で有効化する。
公式手順では、最新 desktop app で Figma Design file を開き、Dev Mode の inspect panel から `Enable desktop MCP server` を有効にする。

参照:

- https://developers.figma.com/docs/figma-mcp-server/local-server-installation/
- https://developers.figma.com/docs/figma-mcp-server/remote-server-installation/

したがって、CDP 直結を唯一経路に固定しない。

### target discovery

`/json/version` のみで target を選ばない設計にする。
`/json/list` も使い、次の score を合算する。

- `type == page`
- `title` に `figma`
- `url` に `figma`
- websocket URL の存在
- 既知の DevTools タイトル除外
- user override の一致

`doctor --json` で score 内訳を見えるようにする。

### port discovery

探索順序は次とする。

1. CLI / config で明示された host/port
2. 既知の CDP port 候補
3. `9222-9322` の走査
4. MCP の既定 endpoint (`127.0.0.1:3845`)

### runtime health

`RuntimeHealth` は次を持つ。

- `transport`
- `host`
- `port`
- `target_id`
- `target_title`
- `target_url`
- `ping_ok`
- `snapshot_ok`
- `latency_ms`
- `warnings[]`
- `errors[]`

## 2. extractor 設計

### やること

- depth 計算
- subtree_size 計算
- sibling_index 計算
- text feature 抽出
- visual feature 抽出
- auto layout feature 抽出
- subtree fingerprint 計算
- repeated_subtree_count 集計

### subtree fingerprint

初版では粗い fingerprint でよいが、**子 fingerprint を再帰的に含める**ことを条件にする。

```text
node_type + layout_kind + textuality + visuality + sorted(child_fingerprints)
```

### extractor が抱えない責務

- component naming
- atomic classification
- boundary 確定
- reusability score

## 3. normalizer 設計

この層が中核である。価値は接続より normalizer の品質で決まる。

### 責務

1. 木の再構築
2. layout-only wrapper collapse
3. boundary detection
4. `FeatureGraph -> UiIr`

### wrapper collapse

collapse 条件は signal 合議にする。

#### 候補条件

- child が 1 つ
- 自身が text でない
- fill / effect / stroke / radius が弱い
- repeated pattern の核でない
- `wrapper`, `container`, `frame`, `layout`, `group` 的な名前
- semantic hint が乏しい

#### collapse 禁止条件

- repetition 候補
- explicit component name を含む
- children の意味境界を保っている
- diagnostics で保全指定されたもの

### boundary detection

単一規則ではなく、複数 signal の採点とする。

#### signal

- visual_group
- repetition
- layout
- text_semantic
- explicit_name
- spacing_discontinuity
- sibling_homogeneity

#### 出力

- `signals`
- `total_score`
- `accepted`
- `reasons`

判定結果は diagnostics に保持する。

### 再帰変換

`build_ui_node` の責務は次に限定する。

- 現在ノードを 1 UI ノードへ変換
- 子を正規化
- wrapper なら grandchildren を持ち上げる
- boundary accepted を kind 推定に反映

初版では sibling grouping までは入れない。

## 4. classifier 設計

classifier は推定器であって断定器ではない。

### 出力

- `component_candidates[]`
- `atomic_candidates[]`
- `reusability`
- `confidence`

### component candidate

#### 命名規則

- name を正規化して PascalCase 化
- kind に応じて suffix を補う
- `Card`, `Button`, `Section`, `ListItem` を優先
- secondary candidate も出す

#### 例

- `Checkout Button` -> `CheckoutButton`
- `Order Summary` + `Card` -> `OrderSummaryCard`
- `Filter Section` -> `FilterSection`

### atomic candidate

#### atom

- Button
- Input
- Checkbox
- Radio
- Switch
- Icon
- Badge
- Heading
- Text

#### molecule

- Card
- ListItem
- FieldGroup

#### organism

- Section
- Navigation
- Form
- List

#### template/page

- Template
- Screen

`Unknown` も残す。

### reusability score

#### 加点要素

- primitive UI pattern
- compact child structure
- semantic name hint
- portable textual content
- repeated subtree

#### 減点要素

- top-level layout
- domain-specific name
- excessive contextuality
- screen 固有文脈への強依存

`reusability.score`, `reusability.level`, `reusability.reasons[]` を保持する。

## 5. diagnostics 設計

diagnostics を軽視すると、誤判定時に手が止まる。

### レベル

- error
- warn
- info
- debug

### コード例

- `runtime.target.not_found`
- `runtime.snapshot.timeout`
- `raw.node.missing_bounds`
- `feature.layout.unknown`
- `normalize.wrapper.collapsed`
- `normalize.boundary.low_confidence`
- `classify.atomic.ambiguous`
- `report.reusability.low_signal`

### 記録内容

- code
- message
- severity
- source_node_ids
- module
- evidence
- suggestion

## 6. reporter 設計

出力は `report.md` である。人間が読める形を優先する。

### 含める章

#### 概要

- frame id
- frame name
- node count
- normalized node count
- collapsed wrapper count
- transport 種別
- snapshot 時刻

#### 構造要約

```text
Screen
  Section: Header
  Section: Filters
  List
    ListItem x 8
  Section: FooterActions
```

#### 低信頼箇所

- boundary accepted だが score が低い
- unknown layout
- atomic ambiguity
- low confidence component name

#### 再利用性上位

- score 上位 10 件
- component candidate
- atomic candidate
- reasons

#### 実装順序提案

- reusable primitive
- repeated list item
- card-like composite
- section-like composite
- top-level screen assembly

### reporter の禁則

- JSON の貼り付けで埋めない
- 低信頼箇所を隠さない
- 推定を断定口調で書かない

## 7. store 設計

初期保存対象は次に固定する。

- `raw.json`
- `features.json`
- `ui.ir.json`
- `report.md`

### 保存方針

- 各コマンドは単一の file-in / file-out を基本にする
- `extract`, `features`, `normalize`, `report` は既定ファイル名を持つ
- 将来 `--output-dir` や bundle command を追加しても、この file-in / file-out 契約を崩さない

### 将来の SQLite

後段で `analysis.db` を導入する場合、少なくとも次を持つ。

- `nodes`
- `edges`
- `style_clusters`
- `repetition_groups`
- `component_candidates`
- `diagnostics`

初期段階では入れない。
