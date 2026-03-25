use std::io;

use figex_cli_core::RuntimeError;
use figex_cli_runtime::transport::cdp::map_ws_send_result_for_test;
use tokio_tungstenite::tungstenite::Error as WsError;

#[test]
fn successful_sends_stay_successful() {
    let result = map_ws_send_result_for_test(Ok(()));

    assert!(
        result.is_ok(),
        "successful sends should remain ok: {result:?}"
    );
}

#[test]
fn send_errors_map_to_attach_failed() {
    let error = map_ws_send_result_for_test(Err(WsError::Io(io::Error::other("send failed"))))
        .expect_err("send errors should map to attach failures");

    assert!(matches!(error, RuntimeError::AttachFailed));
}
