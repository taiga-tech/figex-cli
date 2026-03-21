# データモデル

この文書を `raw.json`, `features.json`, `ui.ir.json` の正規定義とする。
Rust の型名は PascalCase、JSON field は snake_case に統一する。

## Raw

Raw は観測結果であり、意味づけをしない。
冗長でも保持し、後段で必要になりそうな情報を削らない。

```rust
pub struct RawFrameSnapshot {
    pub version: String,
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

## FeatureGraph

FeatureGraph は観測特徴の集合である。
`extractor` は観測と集約に徹し、Button や Card のような意味分類はしない。

```rust
pub struct FeatureGraph {
    pub version: String,
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

## UiIr

UiIr は意味付き中間表現である。
`ui.ir.json` は tree を正規形式とし、flat graph は採用しない。

```rust
pub struct UiIr {
    pub version: String,
    pub source: UiSourceMeta,
    pub tree: UiNode,
    pub diagnostics: Vec<Diagnostic>,
    pub stats: UiIrStats,
}
```

### `UiSourceMeta`

- `frame_id`
- `frame_name`
- `runtime`
- `transport`
- `captured_at`
- `tool_version`

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

`Unknown` を残す設計にして、無理に断定しない。

## v1 でやらないこと

次は v1 の必須データモデルから外す。

- `tokens` のトップレベル同梱
- 別形式の flat `nodes` コレクション併記
- コード生成用 AST の直列化
