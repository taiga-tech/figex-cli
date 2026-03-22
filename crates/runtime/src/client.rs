use async_trait::async_trait;
use figex_cli_core::RuntimeError;

use crate::health::RuntimeHealth;
use crate::snapshot::{RuntimeFrameRequest, RuntimeFrameSnapshot};

/// Resolved information about a successfully attached runtime session.
#[derive(Debug)]
pub struct RuntimeSession {
    pub target_id: String,
    pub target_title: String,
    pub target_url: String,
    pub host: String,
    pub port: u16,
}

/// Public boundary for Figma runtime operations.
///
/// Three operations are exposed: `attach`, `health`, and `snapshot_frame`.
/// The internal `eval` mechanism is not part of this surface.
#[async_trait]
pub trait RuntimeClient: Send + Sync {
    /// Connect to the Figma runtime, enable the CDP Runtime domain,
    /// and return a session descriptor.
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError>;

    /// Run a full diagnostic probe and return a health report.
    /// Always resolves to `Ok`; failures are reflected in the returned struct.
    async fn health(&self) -> Result<RuntimeHealth, RuntimeError>;

    /// Execute a snapshot evaluation against the connected runtime.
    async fn snapshot_frame(
        &self,
        request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError>;
}
