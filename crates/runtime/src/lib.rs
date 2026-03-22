pub mod client;
pub mod health;
pub mod snapshot;
pub mod transport;

pub use client::{RuntimeClient, RuntimeSession};
pub use health::RuntimeHealth;
pub use snapshot::{RuntimeFrameRequest, RuntimeFrameSnapshot};
pub use transport::cdp::{discover_best_target, CdpClient, ProbeConfig};

/// Returns the (ping_script_version, snapshot_script_version) tuple.
pub fn transport_versions() -> (&'static str, &'static str) {
    use transport::cdp::js_bridge::{PING_SCRIPT_VERSION, SNAPSHOT_SCRIPT_VERSION};
    (PING_SCRIPT_VERSION, SNAPSHOT_SCRIPT_VERSION)
}
