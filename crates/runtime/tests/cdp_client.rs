//! Integration tests for `CdpClient`.
//!
//! Mock tests use `wiremock` (HTTP) + `tokio-tungstenite` (WebSocket).
//!
//! # Live runtime tests
//!
//! The `#[ignore]` tests at the bottom require a running Figma Desktop.
//!
//! Prerequisites:
//! 1. Figma Desktop is running with at least one file open.
//! 2. Remote debugging must be enabled:
//!    - Figma → Help → Enable Remote Debugging, **or**
//!    - launch Figma with `--remote-debugging-port=9222`.
//!
//! Run command:
//! ```bash
//! FIGMA_RUNTIME_TEST=1 cargo test -p figex-cli-runtime -- --ignored
//! ```

#[path = "support/mock_cdp_server.rs"]
mod mock_cdp_server;

use std::time::Duration;

use figex_cli_runtime::{CdpClient, RuntimeClient, RuntimeFrameRequest};
use wiremock::MockServer;

#[test]
fn transport_versions_are_exposed() {
    let (ping_script_version, snapshot_script_version) = figex_cli_runtime::transport_versions();

    assert!(!ping_script_version.is_empty());
    assert!(!snapshot_script_version.is_empty());
}

// ---------------------------------------------------------------------------
// attach
// ---------------------------------------------------------------------------

#[tokio::test]
async fn attach_succeeds_with_mock_server() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-attach", "Figma - Attach", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let result = client.attach().await;
    assert!(result.is_ok(), "attach should succeed: {result:?}");
    let session = result.unwrap();
    assert_eq!(session.target_id, "t-attach");
}

#[tokio::test]
async fn attach_ignores_noise_before_matching_ws_response() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_noise_then_success(
        ws_listener,
    ));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-noise", "Figma - Attach", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let result = client.attach().await;
    assert!(
        result.is_ok(),
        "attach should ignore unrelated frames: {result:?}"
    );
    assert_eq!(result.unwrap().target_id, "t-noise");
}

#[tokio::test]
async fn attach_returns_evaluate_failed_when_runtime_enable_fails() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_runtime_enable_error(
        ws_listener,
    ));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-enable-fail", "Figma - Attach", ws_port)
        .await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let error = client
        .attach()
        .await
        .expect_err("attach should surface Runtime.enable failures");
    assert!(
        matches!(error, figex_cli_core::RuntimeError::EvaluateFailed(message) if message.contains("runtime enable failed"))
    );
}

#[tokio::test]
async fn attach_accepts_runtime_enable_responses_without_result_payload() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_missing_result_for_runtime_enable(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-missing-result", "Figma - Attach", ws_port)
        .await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let result = client.attach().await;
    assert!(
        result.is_ok(),
        "attach should accept missing result payloads: {result:?}"
    );
    assert_eq!(result.unwrap().target_id, "t-missing-result");
}

#[tokio::test]
async fn attach_returns_attach_failed_when_socket_closes_before_response() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_close_frame(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-close", "Figma - Attach", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let error = client
        .attach()
        .await
        .expect_err("attach should fail when the websocket closes early");
    assert!(matches!(error, figex_cli_core::RuntimeError::AttachFailed));
}

#[tokio::test]
async fn attach_returns_timeout_when_runtime_enable_never_responds() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_and_hang(
        ws_listener,
        Duration::from_millis(200),
    ));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-timeout", "Figma - Attach", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_millis(50),
    );

    let error = client
        .attach()
        .await
        .expect_err("attach should time out when Runtime.enable never responds");
    assert!(matches!(error, figex_cli_core::RuntimeError::Timeout));
}

#[tokio::test]
async fn attach_returns_attach_failed_when_websocket_send_fails() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_and_drop_immediately(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-send-fail", "Figma - Attach", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let error = client
        .attach()
        .await
        .expect_err("attach should fail when sending the first CDP command fails");
    assert!(matches!(error, figex_cli_core::RuntimeError::AttachFailed));
}

// ---------------------------------------------------------------------------
// health
// ---------------------------------------------------------------------------

#[tokio::test]
async fn health_returns_ping_ok_with_mock_server() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-health", "Figma - Health", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let health = client.health().await.unwrap();
    assert!(health.ping_ok, "ping should succeed");
    assert!(health.snapshot_ok, "snapshot should succeed");
    assert_eq!(health.target_id.as_deref(), Some("t-health"));
    assert!(
        health.errors.is_empty(),
        "no errors expected: {:?}",
        health.errors
    );
    // Script version metadata should be populated.
    assert!(!health.ping_script_version.is_empty());
    assert!(!health.snapshot_script_version.is_empty());
}

#[tokio::test]
async fn health_returns_errors_when_no_runtime() {
    // Port 1 is privileged and never open on a dev machine.
    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(1),
        Duration::from_millis(200),
    );

    let health = client.health().await.unwrap();
    assert!(!health.ping_ok);
    assert!(!health.snapshot_ok);
    assert!(!health.errors.is_empty());
}

#[tokio::test]
async fn health_returns_target_context_when_websocket_connect_fails() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let closed_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let closed_port = closed_listener
        .local_addr()
        .expect("listener should expose local address")
        .port();
    drop(closed_listener);

    mock_cdp_server::mount_figma_cdp(&http_server, "t-no-ws", "Figma - Health", closed_port).await;

    let client = CdpClient::new(None, Some(http_port), Duration::from_secs(3));

    let health = client.health().await.unwrap();
    assert_eq!(health.host, "127.0.0.1");
    assert_eq!(health.port, http_port);
    assert_eq!(health.target_id.as_deref(), Some("t-no-ws"));
    assert_eq!(health.target_title.as_deref(), Some("Figma - Health"));
    assert_eq!(
        health.target_url.as_deref(),
        Some("https://www.figma.com/file/t-no-ws")
    );
    assert_eq!(health.target_score, Some(8));
    assert!(!health.ping_ok);
    assert!(!health.snapshot_ok);
    assert!(health
        .errors
        .iter()
        .any(|error| error.contains("websocket connect failed")));
}

#[tokio::test]
async fn health_records_runtime_enable_errors_and_continues() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_runtime_enable_error(
        ws_listener,
    ));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-enable-health", "Figma - Health", ws_port)
        .await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let health = client.health().await.unwrap();
    assert!(health.ping_ok);
    assert!(health.snapshot_ok);
    assert!(health.warnings.is_empty());
    assert!(health
        .errors
        .iter()
        .any(|error| error.contains("Runtime.enable failed")));
}

#[tokio::test]
async fn health_reports_ping_failures() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_ping_failure(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-ping-fail", "Figma - Health", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let health = client.health().await.unwrap();
    assert!(!health.ping_ok);
    assert!(health.snapshot_ok);
    assert!(health.warnings.is_empty());
    assert!(health.errors.iter().any(|error| error == "ping failed"));
}

#[tokio::test]
async fn health_reports_snapshot_failures_as_warnings() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_snapshot_failure(
        ws_listener,
    ));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-snapshot-fail", "Figma - Health", ws_port)
        .await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let health = client.health().await.unwrap();
    assert!(health.ping_ok);
    assert!(!health.snapshot_ok);
    assert!(health.errors.is_empty());
    assert!(health
        .warnings
        .iter()
        .any(|warning| warning == "snapshot check failed"));
}

// ---------------------------------------------------------------------------
// snapshot_frame
// ---------------------------------------------------------------------------

#[tokio::test]
async fn snapshot_frame_succeeds_with_mock_server() {
    let http_server = MockServer::start().await;
    let http_port = http_server.address().port();

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one(ws_listener));
    mock_cdp_server::mount_figma_cdp(&http_server, "t-snap", "Figma - Snapshot", ws_port).await;

    let client = CdpClient::new(
        Some("127.0.0.1".to_string()),
        Some(http_port),
        Duration::from_secs(3),
    );

    let snap = client
        .snapshot_frame(RuntimeFrameRequest { frame_ref: None })
        .await;
    assert!(snap.is_ok(), "snapshot should succeed: {snap:?}");
    assert!(snap.unwrap().ok);
}

// ---------------------------------------------------------------------------
// Live runtime tests (require FIGMA_RUNTIME_TEST=1)
// ---------------------------------------------------------------------------

fn is_live_test_enabled() -> bool {
    std::env::var("FIGMA_RUNTIME_TEST").is_ok_and(|v| v == "1")
}

#[tokio::test]
#[ignore = "requires live Figma Desktop runtime (FIGMA_RUNTIME_TEST=1)"]
async fn live_attach_connects_to_figma() {
    if !is_live_test_enabled() {
        return;
    }
    let client = CdpClient::new(None, None, Duration::from_secs(5));
    let session = client.attach().await.expect("live attach should succeed");
    assert!(!session.target_id.is_empty());
    println!(
        "Attached: {} @ {}:{}",
        session.target_title, session.host, session.port
    );
}

#[tokio::test]
#[ignore = "requires live Figma Desktop runtime (FIGMA_RUNTIME_TEST=1)"]
async fn live_doctor_returns_healthy_state() {
    if !is_live_test_enabled() {
        return;
    }
    let client = CdpClient::new(None, None, Duration::from_secs(5));
    let health = client.health().await.unwrap();
    assert!(health.ping_ok, "live ping should succeed: {health:?}");
    println!("{}", serde_json::to_string_pretty(&health).unwrap());
}

#[tokio::test]
#[ignore = "requires live Figma Desktop runtime (FIGMA_RUNTIME_TEST=1)"]
async fn live_snapshot_frame_succeeds() {
    if !is_live_test_enabled() {
        return;
    }
    let client = CdpClient::new(None, None, Duration::from_secs(5));
    let snap = client
        .snapshot_frame(RuntimeFrameRequest { frame_ref: None })
        .await
        .expect("live snapshot should succeed");
    assert!(snap.ok);
    println!("snapshot ts={}", snap.ts);
}
