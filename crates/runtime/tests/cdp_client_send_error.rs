use std::io;

use figex_cli_core::RuntimeError;
use figex_cli_runtime::transport::cdp::{map_ws_incoming_message, map_ws_send_result};
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;

#[test]
fn successful_sends_stay_successful() {
    let result = map_ws_send_result(Ok(()));

    assert!(
        result.is_ok(),
        "successful sends should remain ok: {result:?}"
    );
}

#[test]
fn send_errors_map_to_attach_failed() {
    let error = map_ws_send_result(Err(WsError::Io(io::Error::other("send failed"))))
        .expect_err("send errors should map to attach failures");

    assert!(matches!(error, RuntimeError::AttachFailed));
}

#[test]
fn websocket_non_text_messages_are_ignored() {
    let result = map_ws_incoming_message(Some(Ok(Message::Binary(vec![0xde, 0xad]))), 1)
        .expect("non-text frames should be ignored");

    assert!(result.is_none(), "non-text frames should be ignored");
}

#[test]
fn websocket_stream_errors_map_to_attach_failed() {
    let error = map_ws_incoming_message(Some(Err(WsError::AlreadyClosed)), 1)
        .expect_err("stream errors should map to attach failures");

    assert!(matches!(error, RuntimeError::AttachFailed));
}

#[test]
fn websocket_stream_end_maps_to_attach_failed() {
    let error =
        map_ws_incoming_message(None, 1).expect_err("stream end should map to attach failures");

    assert!(matches!(error, RuntimeError::AttachFailed));
}
