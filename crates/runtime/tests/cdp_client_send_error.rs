use figex_cli_core::RuntimeError;
use figex_cli_runtime::transport::cdp::map_ws_send_result_for_test;

#[test]
fn send_errors_map_to_attach_failed() {
    let error = map_ws_send_result_for_test(Err(()))
        .expect_err("send errors should map to attach failures");

    assert!(matches!(error, RuntimeError::AttachFailed));
}
