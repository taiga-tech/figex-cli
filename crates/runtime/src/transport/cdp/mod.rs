mod client;
pub mod js_bridge;
pub mod probe;
mod protocol;

#[doc(hidden)]
pub use client::map_ws_incoming_message;
#[doc(hidden)]
pub use client::map_ws_send_result;
pub use client::CdpClient;
pub use probe::{discover_best_target, ProbeConfig};
