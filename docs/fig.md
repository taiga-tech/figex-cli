# Figma Runtime Extractor 完全設計書

作成日: 2026-03-18

## 1. 文書の位置づけ

本書は、アップロードされた「Figma CLI 解析」の方針を起点に、実装へ移れる水準まで設計を固定した文書です。
前提条件は会話で示されたものをそのまま採用します。

- Figma REST API は使わない
- Figma plugin は使わない
- Figma Desktop 実行環境への接続のみを使う
- 配布骨格は `figex-cli` を使う
- 出力の中核はコードではなく UI IR とする
- コード生成は後段の別責務とする

ただし、2026-03 時点では Figma Desktop 向けの公式 Desktop MCP Server が公開されているため、CDP 直結のみを唯一の実装方式として固定すると将来差し替えが難しくなります。したがって本設計では、**外部仕様としては Desktop runtime 接続一本に見せつつ、内部では transport adapter を分離し、CDP adapter と将来の MCP adapter を差し替え可能にする**構成を採ります。

## 2. プロダクト定義

このプロジェクトは「Figma CLI」ではなく、**Figma Runtime Extractor** と定義します。
役割は次の 4 つに限定します。

1. Figma Desktop に接続する
2. 指定フレームを観測する
3. 実装判断に使える UI IR に正規化する
4. JSON と人間向けレポートを出力する

本体に入れないものを明確にします。

- Figma ドキュメントの新規作成
- ノード更新、移動、削除
- コメント操作
- 画像 export
- 任意 eval の公開
- JSX / React / Storybook 生成
- デザイントークン完成命名
- LLM による全面分類

この切り分けを崩すと、接続層・抽出層・投影層の責務が混ざり、品質の検証点が失われます。

## 3. 設計判断の総括

### 3.1 採用する判断

- 配布は `figex-cli` の Rust + npm ランチャ + プラットフォーム別パッケージ構成を踏襲する
- ドメイン構造は `runtime -> raw -> features -> ui ir -> report` の直列パイプラインとする
- runtime の公開 API は `attach` `health` `snapshot_frame` に限定する
- 取得データは Raw / Features / UiIr に三段分離する
- 正規化の中核は `wrapper collapse` `boundary detection` `diagnostics` とする
- classifier は断定器ではなく推定器とする
- 保存形式は当初 JSON のみとし、SQLite は後段とする

### 3.2 採用しない判断

- plugin 経由ブリッジ
- daemon 常駐前提
- arbitrary eval の公開
- 書き込み操作を混ぜた万能 CLI
- コード生成まで一気通貫で抱える構成

### 3.3 2026-03 時点で見直した点

`samples-cli` の公開テンプレートは現時点で `crates/core`, `crates/cli`, `packages/cli`, `packages/cli-<platform>` を軸にしたモノレポであり、Node 24 / pnpm 10.30.3 / Rust 1.93.1 / mise を前提にしています。これを配布骨格として採る判断は妥当です。一方で `silships/figma-cli` は現在も plugin と daemon を含む構成で、safe mode でも plugin API を利用します。したがって、**接続思想の参考**にはなりますが、**そのままの runtime 構造は採らない**のが妥当です。さらに Figma 公式には Desktop MCP Server があり、ローカルで `http://127.0.0.1:3845/mcp` を提供します。このため runtime 境界を transport 抽象で切っておく価値が高くなっています。

## 4. 完成像

```text
Desktop runtime
  -> RuntimeFrameSnapshot
  -> RawFrameSnapshot
  -> FeatureGraph
  -> UiIrDraft
  -> UiIrClassified
  -> report.md
```

設計の評価基準は次の通りです。

- Desktop への attach が安定している
- 同一フレームから同一 IR が再現される
- wrapper collapse の過不足が低い
- boundary 判定の理由を diagnostics で追える
- classifier の推定根拠を report で読める
- projector 層を後付けしても core を壊さない

## 5. リポジトリ構成

`figex-cli` の配布方式に合わせ、ドメイン側は次の構成に固定します。

```text
.
├── Cargo.toml
├── rust-toolchain.toml
├── mise.toml
├── package.json
├── pnpm-workspace.yaml
├── turbo.json
│
├── crates/
│   ├── core/
│   ├── cli/
│   ├── runtime/
│   ├── extractor/
│   ├── normalizer/
│   ├── classifier/
│   ├── reporter/
│   ├── schema/
│   └── store/
│
├── packages/
│   ├── cli/
│   ├── cli-darwin-arm64/
│   ├── cli-darwin-x64/
│   ├── cli-win32-x64/
│   ├── cli-linux-x64-gnu/
│   └── cli-linux-x64-musl/
│
├── schemas/
│   ├── raw.schema.json
│   ├── features.schema.json
│   └── ui.ir.schema.json
│
├── fixtures/
│   ├── raw/
│   ├── features/
│   └── ui-ir/
│
└── scripts/
    └── place-binary-like-ci.sh
```

### 5.1 crate ごとの責務

#### `crates/core`

- 基本型定義
- Raw / Feature / UiIr の共通モデル
- diagnostics 型
- 共通エラー型
- serde / schemars 対応

#### `crates/cli`

- `clap` による引数定義
- 設定ファイル読込
- コマンド dispatch
- 結果出力と終了コード制御

#### `crates/runtime`

- Desktop との接続
- target 探索
- transport adapter
- snapshot 取得
- runtime health 診断

#### `crates/extractor`

- `RawFrameSnapshot -> FeatureGraph`
- 構造特徴、視覚特徴、layout 特徴、text 特徴、反復特徴の抽出

#### `crates/normalizer`

- 木の再構築
- layout-only wrapper collapse
- boundary detection
- `FeatureGraph -> UiIrDraft`

#### `crates/classifier`

- component candidate 推定
- atomic candidate 推定
- reusability score 推定
- confidence 付与

#### `crates/reporter`

- `UiIrClassified -> report.md`
- 低信頼箇所の要約
- 実装順序案の出力

#### `crates/schema`

- `schemars` を用いた JSON Schema 生成
- schema version 管理

#### `crates/store`

- JSON 保存
- 将来の SQLite backend 拡張点

## 6. 実行環境と配布

### 6.1 採用方針

- 開発環境管理は `mise`
- JavaScript 側は `pnpm workspace`
- Rust 側は workspace
- ビルド統括は `mise run` + `turbo`
- 配布は npm ランチャ + プラットフォーム別バイナリ

### 6.2 期待するファイル

#### `mise.toml`

- node
- pnpm
- rust
- cargo-nextest などの補助ツール
- `tasks.build`, `tasks.test`, `tasks.check`, `tasks.cli`

#### `package.json`

- `packageManager: pnpm@...`
- workspace scripts
- release 補助 scripts

#### `pnpm-workspace.yaml`

- `packages/*`

#### `Cargo.toml`

- workspace members
- lint 設定

### 6.3 配布コマンド名

実行コマンドは `figma-ir` に固定します。
理由は、Figma を操作する万能 CLI と誤読されにくく、IR 抽出器としての性格が明瞭だからです。

## 7. CLI 仕様

公開コマンドは次に固定します。

```bash
figma-ir attach
figma-ir doctor
figma-ir inspect frame --id 123:456
figma-ir extract frame --id 123:456 --out raw.json
figma-ir features frame --id 123:456 --out features.json
figma-ir normalize frame --id 123:456 --out ui.ir.json
figma-ir report frame --id 123:456 --out report.md
```

### 7.1 コマンド定義

#### `attach`

- Desktop へ接続できるかだけを確認する
- 成功時は target 情報と transport 種別を出す
- `--json` を付けると機械可読出力

#### `doctor`

- port discovery
- target scoring 内訳
- 現在選ばれた target
- runtime ping 結果
- snapshot 最小試験
- timeout 設定値
- transport 種別
- 失敗理由の分類

#### `inspect frame`

- 指定 frame の基本メタ情報だけを表示
- node count, text count, layout count, repeated candidate count など

#### `extract frame`

- Raw を保存

#### `features frame`

- Raw 取得後、FeatureGraph を保存

#### `normalize frame`

- UiIrDraft または UiIrClassified を保存
- `--classify=off` で classifier を止められるようにしてもよい

#### `report frame`

- IR と diagnostics を読み、人間向け Markdown を出力

### 7.2 追加してよい補助オプション

- `--json`
- `--pretty`
- `--timeout-ms`
- `--transport cdp|mcp|auto`
- `--host`
- `--port`
- `--config <path>`
- `--log-level error|warn|info|debug|trace`

コマンド体系そのものは増やさず、運用オプションで吸収します。

## 8. 設定ファイル仕様

`figma-ir.toml` を採用します。

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
write_report = true

[classifier]
enabled = true
max_component_candidates = 3
max_atomic_candidates = 3

[normalizer]
collapse_wrappers = true
boundary_threshold = 0.62
preserve_repetition_candidates = true
```

### 8.1 設定の優先順位

1. CLI 引数
2. 環境変数
3. `figma-ir.toml`
4. 既定値

### 8.2 環境変数

- `FIGMA_IR_TRANSPORT`
- `FIGMA_IR_HOST`
- `FIGMA_IR_PORT`
- `FIGMA_IR_TIMEOUT_MS`
- `FIGMA_IR_LOG_LEVEL`
- `FIGMA_RUNTIME_TEST`

## 9. runtime 設計

## 9.1 公開境界

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

外から見せるのはこの 3 操作だけです。
`eval(String)` は内部実装に閉じます。

## 9.2 transport adapter

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

### 9.3 CDP adapter

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

### 9.4 MCP adapter

現段階では実装しないが、trait を満たす将来 adapter として余地だけ作ります。
理由は、Figma 公式の Desktop MCP Server がある以上、CDP 直結を唯一経路に固定すると維持費が増えるためです。

### 9.5 target discovery

`/json/version` のみで target を選ばない設計にします。
`/json/list` も使い、次の score を合算します。

- `type == page`
- `title` に `figma`
- `url` に `figma`
- websocket URL の存在
- 既知の DevTools タイトル除外
- user override の一致

`doctor --json` で score 内訳を見えるようにします。

### 9.6 port discovery

探索順序は次とします。

1. CLI / config で明示された host/port
2. 既知の port 候補
3. `9222-9322` の走査
4. 将来の mcp 固定 port `3845`

### 9.7 runtime health

`RuntimeHealth` は次を持ちます。

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

## 10. 取得データモデル

## 10.1 Raw

Raw は観測結果であり、意味づけをしません。

```rust
pub struct RawFrameSnapshot {
    pub frame: RawFrameMeta,
    pub nodes: Vec<RawNode>,
    pub edges: Vec<RawEdge>,
    pub texts: Vec<RawText>,
    pub fills: Vec<RawFill>,
    pub bounds: Vec<RawBounds>,
    pub auto_layout: Vec<RawAutoLayout>,
    pub effects: Vec<RawEffect>,
    pub strokes: Vec<RawStroke>,
    pub export_hints: Vec<RawExportHint>,
}
```

### Raw に含めるべき項目

- node id, parent id, name, node type
- visible, locked, opacity
- local bounds / absolute bounds
- text content, style, line information
- fills / strokes / effects
- corner radius
- auto layout direction, gap, padding, sizing mode
- instance / component 由来情報が観測できるなら保持

Raw で削ると後段で復元できません。冗長でも保持します。

## 10.2 FeatureGraph

FeatureGraph は観測特徴の集合です。

```rust
pub struct FeatureGraph {
    pub nodes: Vec<FeatureNode>,
    pub edges: Vec<FeatureEdge>,
    pub stats: FeatureStats,
}
```

### `FeatureNode` の主な属性

#### structure

- depth
- child_count
- sibling_index
- subtree_size
- descendant_text_count
- descendant_visual_count

#### layout

- layout_kind
- direction
- gap
- padding_top/right/bottom/left
- align_main
- align_cross
- wrap

#### visual

- has_fill
- has_radius
- has_shadow
- has_blur
- has_image_fill
- has_vector
- fill_count
- contrast_hint

#### text

- is_textual
- raw_text
- content_length
- line_count_estimate
- token_count_estimate
- heading_hint
- label_hint

#### repetition

- fingerprint
- repeated_subtree_count
- repeated_sibling_group_size
- repetition_rank

`extractor` では Button や Card の判定をしません。

## 10.3 UiIr

UiIr は意味付き中間表現です。

```rust
pub struct UiIr {
    pub version: String,
    pub frame: UiFrameMeta,
    pub nodes: Vec<UiNode>,
    pub root_id: String,
    pub diagnostics: Vec<Diagnostic>,
    pub stats: UiIrStats,
}
```

### `UiNode`

- `id`
- `name`
- `kind`
- `source_node_ids`
- `layout`
- `style_refs`
- `semantics`
- `content`
- `children`
- `component_candidates`
- `atomic_candidates`
- `confidence`
- `reusability`
- `diagnostics`

### `kind` 候補

- `Screen`
- `Section`
- `Navigation`
- `List`
- `ListItem`
- `Card`
- `Form`
- `FieldGroup`
- `Button`
- `Input`
- `Checkbox`
- `Radio`
- `Switch`
- `Badge`
- `Heading`
- `Text`
- `Icon`
- `Image`
- `Container`
- `Unknown`

`Unknown` を残す設計にして、無理に断定しません。

## 11. extractor 設計

### 11.1 やること

- depth 計算
- subtree_size 計算
- sibling_index 計算
- text feature 抽出
- visual feature 抽出
- auto layout feature 抽出
- subtree fingerprint 計算
- repeated_subtree_count 集計

### 11.2 subtree fingerprint

初版では粗い fingerprint でよいが、**子 fingerprint を再帰的に含める**ことを条件にします。
例としては次のような構成です。

```text
node_type + layout_kind + textuality + visuality + sorted(child_fingerprints)
```

### 11.3 extractor が抱えない責務

- component naming
- atomic classification
- boundary 確定
- reusability score

## 12. normalizer 設計

この層が中核です。価値は接続より normalizer の品質で決まります。

### 12.1 責務

1. 木の再構築
2. layout-only wrapper collapse
3. boundary detection
4. UiNode への変換

### 12.2 wrapper collapse

collapse 条件は signal 合議にします。

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

### 12.3 boundary detection

単一規則ではなく、複数 signal の採点とします。

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

判定結果は diagnostics に保持します。

### 12.4 再帰変換

`build_ui_node` の責務は次に限定します。

- 現在ノードを 1 UI ノードへ変換
- 子を正規化
- wrapper なら grandchildren を持ち上げる
- boundary accepted を kind 推定に反映

初版では sibling grouping までは入れません。

## 13. classifier 設計

classifier は推定器であって断定器ではありません。

### 13.1 出力

- `component_candidates[]`
- `atomic_candidates[]`
- `reusability`
- `confidence`

### 13.2 component candidate

#### 命名規則

- name を正規化して PascalCase 化
- kind に応じて suffix を補う
- `Card`, `Button`, `Section`, `ListItem` を優先
- secondary candidate も出す

#### 例

- `Checkout Button` -> `CheckoutButton`
- `Order Summary` + `Card` -> `OrderSummaryCard`
- `Filter Section` -> `FilterSection`

### 13.3 atomic candidate

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

`Unknown` も残します。

### 13.4 reusability score

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

`reusability.score`, `reusability.level`, `reusability.reasons[]` を保持します。

## 14. diagnostics 設計

diagnostics を軽視すると、誤判定時に手が止まります。

### 14.1 レベル

- error
- warn
- info
- debug

### 14.2 コード例

- `runtime.target.not_found`
- `runtime.snapshot.timeout`
- `raw.node.missing_bounds`
- `feature.layout.unknown`
- `normalize.wrapper.collapsed`
- `normalize.boundary.low_confidence`
- `classify.atomic.ambiguous`
- `report.reusability.low_signal`

### 14.3 記録内容

- code
- message
- severity
- source_node_ids
- module
- evidence
- suggestion

## 15. reporter 設計

出力は `report.md` です。人間が読める形を優先します。

### 15.1 含める章

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

### 15.2 reporter の禁則

- JSON の貼り付けで埋めない
- 低信頼箇所を隠さない
- 推定を断定口調で書かない

## 16. store 設計

初期保存対象は次に固定します。

- `raw.json`
- `features.json`
- `ui.ir.json`
- `report.md`

### 16.1 保存先レイアウト

```text
.artifacts/
└── <timestamp>/
    ├── raw.json
    ├── features.json
    ├── ui.ir.json
    └── report.md
```

### 16.2 将来の SQLite

後段で `analysis.db` を導入する場合、少なくとも次を持ちます。

- `nodes`
- `edges`
- `style_clusters`
- `repetition_groups`
- `component_candidates`
- `diagnostics`

初期段階では入れません。

## 17. Schema versioning

`ui.ir.schema.json` は互換性を意識して運用します。

- 後方互換が壊れない追加: minor
- 破壊的変更: major
- diagnostics や optional field 追加: minor

`UiIr.version` と schema version を一致させます。

## 18. エラー設計

### 18.1 終了コード

- `0` 正常終了
- `10` 接続不可
- `11` target 未発見
- `12` snapshot 失敗
- `20` frame 未発見
- `21` data 不整合
- `30` normalize 失敗
- `31` classify 失敗
- `40` write 失敗
- `50` 設定不正

### 18.2 error taxonomy

```rust
pub enum RuntimeError {
    ProbeFailed,
    TargetNotFound,
    AttachFailed,
    Timeout,
    EvaluateFailed,
    SnapshotFailed,
}

pub enum PipelineError {
    RawMappingFailed,
    FeatureExtractionFailed,
    NormalizeFailed,
    ClassificationFailed,
    ReportFailed,
    WriteFailed,
}
```

## 19. セキュリティと運用上の注意

この設計は REST API を使わず、Desktop 実行環境へ接続します。したがって API token 管理は抱えませんが、その代わり **ローカル実行中の Figma セッションに接続する** という性質を持ちます。

運用規則は次の通りとします。

- 任意 eval を外へ公開しない
- snapshot script はバージョン管理し、差分審査対象にする
- `doctor` で attach 先の title / url / target id を可視化する
- ログに snapshot 全文を勝手に吐かない
- エラーログからテキスト本文が漏れないよう redact option を持つ

## 20. テスト戦略

### 20.1 unit test

対象:

- extractor
- normalizer
- classifier
- mapper
- protocol
- probe

### 20.2 fixture test

`fixtures/` に次を置きます。

- `raw/*.json`
- `features/*.json`
- `ui-ir/*.json`

同一入力に対して出力が揺れないことを snapshot test で保証します。

### 20.3 runtime mock test

mock WebSocket server を立て、次を検証します。

- `Runtime.enable`
- `Runtime.evaluate`
- `send_request`
- `evaluate_string`
- `snapshot_frame`

### 20.4 実機テスト

実機依存は ignored test にします。

```bash
FIGMA_RUNTIME_TEST=1 cargo test -- --ignored
```

### 20.5 回帰対象フレーム

- button-heavy frame
- card/list-heavy frame
- dashboard-like frame
- form-like frame
- modal/dialog frame
- empty state frame

## 21. CI / Release

### 21.1 CI job

- Rust format
- Rust clippy
- Rust test
- JS lint
- JS test
- binary build per target
- `scripts/place-binary-like-ci.sh`
- npm launcher build
- smoke test

### 21.2 release

- Git tag
- Rust binary build
- platform package に vendor 配置
- npm publish
- GitHub release notes 自動生成

### 21.3 smoke test

- host platform で `figma-ir doctor --json`
- fixture 入力で `normalize` 実行
- `ui.ir.json` の schema 検証

## 22. 実装順序

### 第1段階: runtime 接続成立

- `/json/list` discovery
- `Runtime.enable`
- `Runtime.evaluate`
- ping
- snapshot frame

### 第2段階: raw の厚みを増やす

- auto layout
- text raw content
- fills / effects / radius
- subtree fingerprint
- repeated counts

### 第3段階: normalizer を固める

- wrapper collapse
- boundary detection
- node recursion
- diagnostics

### 第4段階: classifier を追加

- component naming
- atomic candidates
- reusability
- confidence

### 第5段階: reporter と schema

- report.md
- schema versioning
- schema validation

### 第6段階: fixture 拡充と回帰試験

- 実データ fixture の追加
- regression suite 固定

## 23. 価値が出る箇所

本プロジェクトで差が出るのは次の 3 点です。

### 23.1 wrapper collapse

これが弱いと出力が冗長になります。

### 23.2 boundary detection

これが弱いと component 分割が信用されません。

### 23.3 diagnostics

これが弱いと、人が誤判定を追えません。

runtime が完全ではなくても、この 3 点が強ければ道具として成立します。

## 24. 現時点のリスク

### 24.1 最大の技術リスク

CDP 直結は Figma Desktop 側の変更で壊れやすい可能性があります。
対策として transport adapter を分離し、snapshot 取得の公開契約を trait で固定します。

### 24.2 データ品質リスク

Auto Layout, text, bounds の欠落があると downstream が崩れます。
Raw で削らず保持する方針で緩和します。

### 24.3 誤分類リスク

classifier を推定器として扱い、confidence と diagnostics を常に同梱します。

### 24.4 配布リスク

ネイティブ配布はプラットフォーム差異で詰まりやすいため、`figex-cli` の vendor 配置規約に沿って CI で早期に検証します。

## 25. 将来拡張

本体完成後に追加する crate は次を想定します。

- `projector-react`
- `projector-html`
- `projector-storybook`
- `store-sqlite`
- `runtime-mcp`

これらは **UiIr を入力に取る後段** とし、core pipeline を汚さない形で追加します。

## 26. 最終判断

このプロジェクトは、Figma から直接コードを吐く道具として作るより、**Figma から実装判断に耐える IR を引き抜く抽出器**として作る方が構造的に健全です。

完成条件は次の 1 本です。

```text
Desktop runtime
  -> RawFrameSnapshot
  -> FeatureGraph
  -> UiIr draft
  -> UiIr classified
  -> report.md
```

配布は `figex-cli` の骨格に寄せる。
接続は Desktop runtime だけに寄せる。
plugin / daemon / codegen を本体から切る。
normalizer と diagnostics に工数を集中させる。

この方針であれば、現段階の制約を守りつつ、将来の official MCP adapter への移行余地も確保できます。
