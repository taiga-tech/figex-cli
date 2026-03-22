#[path = "support/mock_cdp_server.rs"]
mod mock_cdp_server;

use std::time::Duration;

use figex_cli_core::RuntimeError;
use figex_cli_runtime::{discover_best_target, ProbeConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn probe(port: u16) -> ProbeConfig {
    ProbeConfig {
        host: Some("127.0.0.1".to_string()),
        port: Some(port),
        timeout: Duration::from_secs(2),
    }
}

async fn spawn_version_only_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let port = listener
        .local_addr()
        .expect("listener should expose local address")
        .port();

    tokio::spawn(async move {
        if let Ok((mut stream, _)) = listener.accept().await {
            let mut buffer = [0u8; 1024];
            let _ = stream.read(&mut buffer).await;

            let body = r#"{"Browser":"Chrome/114.0.0.0","Protocol-Version":"1.3"}"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            stream
                .write_all(response.as_bytes())
                .await
                .expect("server should write the version response");
        }
    });

    port
}

#[tokio::test]
async fn discovers_figma_target_on_explicit_port() {
    let server = MockServer::start().await;
    let port = server.address().port();

    mock_cdp_server::mount_version(&server).await;

    let targets = serde_json::json!([{
        "id": "t-001",
        "title": "Figma - My Design",
        "url": "https://www.figma.com/file/abc123",
        "type": "page",
        "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/ws/t-001", port)
    }]);

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&targets))
        .mount(&server)
        .await;

    let result = discover_best_target(&probe(port)).await;
    assert!(result.is_ok(), "should discover Figma target");
    let target = result.unwrap();
    assert_eq!(target.target_id, "t-001");
    assert_eq!(target.port, port);
}

#[tokio::test]
async fn returns_target_not_found_when_no_figma_page() {
    let server = MockServer::start().await;
    let port = server.address().port();

    mock_cdp_server::mount_version(&server).await;

    let targets = serde_json::json!([{
        "id": "t-002",
        "title": "Chrome DevTools",
        "url": "chrome-devtools://devtools/",
        "type": "other",
        "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/ws/t-002", port)
    }]);

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&targets))
        .mount(&server)
        .await;

    let result = discover_best_target(&probe(port)).await;
    assert!(matches!(result, Err(RuntimeError::TargetNotFound)));
}

#[tokio::test]
async fn prefers_higher_scoring_target() {
    let server = MockServer::start().await;
    let port = server.address().port();

    mock_cdp_server::mount_version(&server).await;

    let targets = serde_json::json!([
        {
            "id": "low",
            "title": "Some page",
            "url": "https://figma.com/file/abc",
            "type": "other",
            "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/ws/low", port)
        },
        {
            "id": "high",
            "title": "Figma - Main",
            "url": "https://www.figma.com/file/xyz",
            "type": "page",
            "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/ws/high", port)
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&targets))
        .mount(&server)
        .await;

    let result = discover_best_target(&probe(port)).await.unwrap();
    assert_eq!(result.target_id, "high");
}

#[tokio::test]
async fn skips_targets_without_websocket_url() {
    let server = MockServer::start().await;
    let port = server.address().port();

    mock_cdp_server::mount_version(&server).await;

    let targets = serde_json::json!([{
        "id": "no-ws",
        "title": "Figma - Design",
        "url": "https://www.figma.com/file/nows",
        "type": "page"
    }]);

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&targets))
        .mount(&server)
        .await;

    let result = discover_best_target(&probe(port)).await;
    assert!(matches!(result, Err(RuntimeError::TargetNotFound)));
}

#[tokio::test]
async fn returns_target_not_found_when_target_list_is_invalid_json() {
    let server = MockServer::start().await;
    let port = server.address().port();

    mock_cdp_server::mount_version(&server).await;

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not-json"))
        .mount(&server)
        .await;

    let result = discover_best_target(&probe(port)).await;
    assert!(matches!(result, Err(RuntimeError::TargetNotFound)));
}

#[tokio::test]
async fn returns_target_not_found_when_target_list_request_cannot_be_sent() {
    let port = spawn_version_only_server().await;

    let result = discover_best_target(&probe(port)).await;
    assert!(matches!(result, Err(RuntimeError::TargetNotFound)));
}

#[tokio::test]
async fn excludes_about_blank_false_positives_after_checking_all_keywords() {
    let server = MockServer::start().await;
    let port = server.address().port();

    mock_cdp_server::mount_version(&server).await;

    let targets = serde_json::json!([{
        "id": "about-blank",
        "title": "Figma - Blank",
        "url": "about:blank",
        "type": "page",
        "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/ws/about-blank", port)
    }]);

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&targets))
        .mount(&server)
        .await;

    let result = discover_best_target(&probe(port)).await;
    assert!(matches!(result, Err(RuntimeError::TargetNotFound)));
}
