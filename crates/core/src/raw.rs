use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Top-level snapshot
// ---------------------------------------------------------------------------

/// Raw snapshot of a single Figma frame.
///
/// This is the canonical output of `figex extract frame` and serves as the
/// lossless record of everything observed at capture time.  Later pipeline
/// stages (extractor, normalizer, …) consume this representation; they must
/// never require information that is absent here.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// ---------------------------------------------------------------------------
// Frame metadata — source + runtime metadata in one struct
// ---------------------------------------------------------------------------

/// Frame-level metadata that captures both *source* and *runtime* context.
///
/// **Source metadata** identifies what was captured:
/// - `frame_ref`  — the reference the user supplied (id, deep-link, alias)
/// - `frame_id`   — Figma node id resolved by the runtime (when available)
/// - `frame_name` — display name of the frame (when available)
///
/// **Runtime metadata** records how and when the capture was performed:
/// - `transport`, `host`, `port` — connection details
/// - `captured_at` — Unix timestamp in milliseconds
/// - `tool_version` — figex-cli version string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawFrameMeta {
    // source metadata
    pub frame_ref: Option<String>,
    pub frame_id: Option<String>,
    pub frame_name: Option<String>,
    // runtime metadata
    pub transport: String,
    pub host: String,
    pub port: u16,
    pub captured_at: u64,
    pub tool_version: String,
}

// ---------------------------------------------------------------------------
// Node
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub node_type: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub corner_radius: Option<RawCornerRadius>,
    /// Component id when this node is a component instance.
    pub component_id: Option<String>,
    pub is_instance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCornerRadius {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
}

// ---------------------------------------------------------------------------
// Edge (parent → child relationship)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEdge {
    pub parent_id: String,
    pub child_id: String,
    /// Zero-based index among the parent's children.
    pub index: u32,
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawText {
    pub node_id: String,
    pub content: String,
    pub style: Option<RawTextStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTextStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub font_weight: Option<u32>,
    pub line_height: Option<f64>,
    pub letter_spacing: Option<f64>,
    pub text_align: Option<String>,
}

// ---------------------------------------------------------------------------
// Color (shared by fills, strokes, effects)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

// ---------------------------------------------------------------------------
// Fill
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawFill {
    pub node_id: String,
    /// E.g. `"solid"`, `"gradient_linear"`, `"gradient_radial"`, `"image"`.
    pub fill_type: String,
    pub color: Option<RawColor>,
    pub opacity: Option<f64>,
    pub visible: bool,
    pub blend_mode: Option<String>,
}

// ---------------------------------------------------------------------------
// Bounds
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawBounds {
    pub node_id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    /// `"local"` or `"absolute"`.
    pub bounds_type: String,
}

// ---------------------------------------------------------------------------
// Auto-layout
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAutoLayout {
    pub node_id: String,
    /// `"horizontal"`, `"vertical"`, or `"none"`.
    pub direction: String,
    pub gap: f64,
    pub padding_top: f64,
    pub padding_right: f64,
    pub padding_bottom: f64,
    pub padding_left: f64,
    pub align_main: Option<String>,
    pub align_cross: Option<String>,
    pub wrap: bool,
    pub sizing_mode_main: Option<String>,
    pub sizing_mode_cross: Option<String>,
}

// ---------------------------------------------------------------------------
// Effect
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawEffect {
    pub node_id: String,
    /// E.g. `"drop_shadow"`, `"inner_shadow"`, `"layer_blur"`, `"background_blur"`.
    pub effect_type: String,
    pub visible: bool,
    pub radius: f64,
    pub color: Option<RawColor>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
}

// ---------------------------------------------------------------------------
// Stroke
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawStroke {
    pub node_id: String,
    pub stroke_type: String,
    pub color: Option<RawColor>,
    pub weight: f64,
    /// `"inside"`, `"outside"`, or `"center"`.
    pub align: Option<String>,
    pub visible: bool,
    pub blend_mode: Option<String>,
}

// ---------------------------------------------------------------------------
// Export hint
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawExportHint {
    pub node_id: String,
    /// E.g. `"PNG"`, `"SVG"`, `"PDF"`, `"JPG"`.
    pub format: String,
    pub suffix: Option<String>,
    pub scale: Option<f64>,
}
