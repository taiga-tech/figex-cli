mod client;
pub mod js_bridge;
pub mod probe;
mod protocol;

#[doc(hidden)]
pub use client::map_ws_send_result_for_test;
pub use client::CdpClient;
pub use probe::{discover_best_target, ProbeConfig};
