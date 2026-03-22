use serde::{Deserialize, Serialize};

/// Input for a snapshot operation.
pub struct RuntimeFrameRequest {
    /// Optional frame reference (id, deep link, or alias).
    /// When `None`, the runtime returns whatever frame is currently active.
    pub frame_ref: Option<String>,
}

/// Minimal snapshot result returned by the runtime.
///
/// In the first implementation stage this confirms that the CDP
/// `Runtime.evaluate` round-trip succeeds; actual frame data extraction
/// is deferred to a later stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeFrameSnapshot {
    pub ok: bool,
    pub ts: u64,
}
