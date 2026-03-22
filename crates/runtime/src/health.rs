use serde::{Deserialize, Serialize};

/// Diagnostic result returned by `RuntimeClient::health`.
///
/// Always produced — even when the runtime is unreachable.
/// Failures are reflected in `ping_ok`, `snapshot_ok`, `warnings`, and `errors`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub transport: String,
    pub host: String,
    pub port: u16,
    pub target_id: Option<String>,
    pub target_title: Option<String>,
    pub target_url: Option<String>,
    /// Target score from discovery — higher means more confident this is a Figma page.
    pub target_score: Option<u32>,
    #[serde(rename = "ping")]
    pub ping_ok: bool,
    #[serde(rename = "snapshot")]
    pub snapshot_ok: bool,
    pub latency_ms: Option<u64>,
    /// Version of the ping script that was evaluated.
    pub ping_script_version: String,
    /// Version of the snapshot script that was evaluated.
    pub snapshot_script_version: String,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
