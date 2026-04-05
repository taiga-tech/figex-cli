#[path = "support/mock_cdp_server.rs"]
mod mock_cdp_server;

use std::path::PathBuf;
use std::sync::OnceLock;

use figex_cli::app::AppContext;
use figex_cli::commands::extract;
use figex_cli::config::Settings;
use figex_cli_core::{FigexError, RawFrameSnapshot, RuntimeError};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn extract_context(host: String, port: u16) -> AppContext {
    extract_context_with_pretty(host, port, true)
}

fn extract_context_with_pretty(host: String, port: u16, pretty: bool) -> AppContext {
    AppContext::new(Settings {
        host,
        host_explicit: true,
        port,
        port_explicit: true,
        pretty,
        ..Settings::default()
    })
}

// ---------------------------------------------------------------------------
// Shape tests — no runtime required
// ---------------------------------------------------------------------------

#[test]
fn raw_frame_snapshot_roundtrips_through_json() {
    let snapshot = make_snapshot();
    let json = serde_json::to_string(&snapshot).expect("should serialize");
    let restored: RawFrameSnapshot = serde_json::from_str(&json).expect("should deserialize back");

    assert_eq!(restored.version, snapshot.version);
    assert_eq!(restored.frame.frame_ref, snapshot.frame.frame_ref);
    assert_eq!(restored.frame.transport, snapshot.frame.transport);
    assert_eq!(restored.frame.captured_at, snapshot.frame.captured_at);
    assert_eq!(restored.nodes.len(), snapshot.nodes.len());
    assert_eq!(restored.edges.len(), snapshot.edges.len());
    assert_eq!(restored.texts.len(), snapshot.texts.len());
    assert_eq!(restored.fills.len(), snapshot.fills.len());
    assert_eq!(restored.bounds.len(), snapshot.bounds.len());
    assert_eq!(restored.auto_layout.len(), snapshot.auto_layout.len());
    assert_eq!(restored.effects.len(), snapshot.effects.len());
    assert_eq!(restored.strokes.len(), snapshot.strokes.len());
    assert_eq!(restored.export_hints.len(), snapshot.export_hints.len());
}

#[test]
fn raw_frame_snapshot_fixture_deserializes() {
    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/raw_snapshot.json");
    let json = std::fs::read_to_string(&fixture_path)
        .expect("fixture file should exist at tests/fixtures/raw_snapshot.json");

    let snapshot: RawFrameSnapshot =
        serde_json::from_str(&json).expect("fixture should deserialize into RawFrameSnapshot");

    assert_eq!(snapshot.version, "1");
    assert_eq!(snapshot.frame.frame_ref.as_deref(), Some("my-frame"));
    assert_eq!(snapshot.frame.frame_id.as_deref(), Some("1:2"));
    assert_eq!(snapshot.frame.transport, "cdp");
    assert_eq!(snapshot.frame.captured_at, 1_700_000_000_000);
    assert_eq!(snapshot.nodes.len(), 2);
    assert_eq!(snapshot.edges.len(), 1);
    assert_eq!(snapshot.texts.len(), 1);
    assert_eq!(snapshot.fills.len(), 1);
    assert_eq!(snapshot.bounds.len(), 2);
    assert_eq!(snapshot.auto_layout.len(), 1);
    assert_eq!(snapshot.effects.len(), 1);
    assert_eq!(snapshot.strokes.len(), 1);
    assert_eq!(snapshot.export_hints.len(), 1);

    // Verify corner_radius is preserved
    let button = snapshot.nodes.iter().find(|n| n.name == "Button").unwrap();
    let cr = button.corner_radius.as_ref().unwrap();
    assert_eq!(cr.top_left, 8.0);
    assert!(button.is_instance);
    assert_eq!(button.component_id.as_deref(), Some("100:1"));
}

// ---------------------------------------------------------------------------
// Integration test — mock CDP server required
// ---------------------------------------------------------------------------

#[tokio::test]
async fn extract_command_writes_raw_json() {
    // attach() and snapshot_frame() each open their own WS connection,
    // so we need serve_two to accept both.
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_two(ws_listener));
    let http_port =
        mock_cdp_server::spawn_figma_cdp_server("t-extract", "Figma - Extract", ws_port).await;

    let ctx = extract_context("127.0.0.1".to_string(), http_port);

    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    let output = tmp_dir.path().join("raw.json");

    extract::run(&ctx, "test-frame", Some(output.clone()))
        .await
        .expect("extract should succeed with a mock runtime");

    let content = std::fs::read_to_string(&output).expect("raw.json should have been written");
    let snapshot: RawFrameSnapshot =
        serde_json::from_str(&content).expect("raw.json should deserialize into RawFrameSnapshot");

    assert_eq!(snapshot.version, "1");
    assert_eq!(snapshot.frame.frame_ref.as_deref(), Some("test-frame"));
    assert_eq!(snapshot.frame.transport, "cdp");
    assert_eq!(snapshot.frame.host, "127.0.0.1");
    // Timestamp comes from the mock server's snapshot response (ts = 1_700_000_000_000).
    assert_eq!(snapshot.frame.captured_at, 1_700_000_000_000);
    // Current runtime returns no node data yet — collections are empty.
    assert!(snapshot.nodes.is_empty());
    assert!(snapshot.edges.is_empty());
}

#[tokio::test]
async fn extract_command_uses_default_output_path_when_output_is_none() {
    let _guard = current_dir_lock().lock().await;
    let original_dir = CurrentDirGuard::capture();
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    std::env::set_current_dir(tmp_dir.path()).expect("test should enter the temp dir");

    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_two(ws_listener));
    let http_port = mock_cdp_server::spawn_figma_cdp_server(
        "t-extract-default-output",
        "Figma - Extract",
        ws_port,
    )
    .await;

    let ctx = extract_context("127.0.0.1".to_string(), http_port);
    extract::run(&ctx, "test-frame", None)
        .await
        .expect("extract should succeed when output uses the default raw.json path");

    assert!(tmp_dir.path().join("raw.json").exists());
    drop(original_dir);
}

#[tokio::test]
async fn extract_command_writes_compact_json_when_pretty_is_disabled() {
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_two(ws_listener));
    let http_port =
        mock_cdp_server::spawn_figma_cdp_server("t-extract-compact", "Figma - Extract", ws_port)
            .await;

    let ctx = extract_context_with_pretty("127.0.0.1".to_string(), http_port, false);
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    let output = tmp_dir.path().join("raw.json");

    extract::run(&ctx, "test-frame", Some(output.clone()))
        .await
        .expect("extract should succeed with compact JSON output");

    let content = std::fs::read_to_string(&output).expect("raw.json should have been written");
    let snapshot: RawFrameSnapshot =
        serde_json::from_str(&content).expect("raw.json should deserialize into RawFrameSnapshot");

    assert_eq!(
        content,
        serde_json::to_string(&snapshot).expect("snapshot should serialize as compact JSON")
    );
    assert!(!content.contains('\n'));
}

#[tokio::test]
async fn extract_command_maps_target_not_found_to_figex_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/json/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "Browser": "Chrome/114.0.0.0",
            "Protocol-Version": "1.3"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    let ctx = extract_context("127.0.0.1".to_string(), server.address().port());
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    let output = tmp_dir.path().join("raw.json");

    let error = extract::run(&ctx, "missing-frame", Some(output))
        .await
        .expect_err("extract should surface a target-not-found error");

    assert!(matches!(
        error.downcast_ref::<FigexError>(),
        Some(FigexError::TargetNotFound(_))
    ));
}

#[tokio::test]
async fn extract_command_maps_runtime_enable_errors_to_connection_failed() {
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_one_with_runtime_enable_error(
        ws_listener,
    ));
    let http_port = mock_cdp_server::spawn_figma_cdp_server(
        "t-extract-connect-fail",
        "Figma - Extract",
        ws_port,
    )
    .await;

    let ctx = extract_context("127.0.0.1".to_string(), http_port);
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    let output = tmp_dir.path().join("raw.json");

    let error = extract::run(&ctx, "test-frame", Some(output))
        .await
        .expect_err("extract should map runtime enable failures to connection errors");

    assert!(matches!(
        error.downcast_ref::<FigexError>(),
        Some(FigexError::ConnectionFailed(_))
    ));
}

#[tokio::test]
async fn extract_command_returns_snapshot_failed_when_runtime_reports_not_ok() {
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(serve_two_with_snapshot_not_ok(ws_listener));
    let http_port = mock_cdp_server::spawn_figma_cdp_server(
        "t-extract-snapshot-not-ok",
        "Figma - Extract",
        ws_port,
    )
    .await;

    let ctx = extract_context("127.0.0.1".to_string(), http_port);
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    let output = tmp_dir.path().join("raw.json");

    let error = extract::run(&ctx, "test-frame", Some(output))
        .await
        .expect_err("extract should fail when the runtime reports ok=false");

    assert!(matches!(
        error.downcast_ref::<FigexError>(),
        Some(FigexError::SnapshotFailed(RuntimeError::SnapshotFailed))
    ));
}

#[tokio::test]
async fn extract_command_maps_snapshot_transport_errors_to_snapshot_failed() {
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(serve_two_with_snapshot_error(ws_listener));
    let http_port = mock_cdp_server::spawn_figma_cdp_server(
        "t-extract-snapshot-error",
        "Figma - Extract",
        ws_port,
    )
    .await;

    let ctx = extract_context("127.0.0.1".to_string(), http_port);
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");
    let output = tmp_dir.path().join("raw.json");

    let error = extract::run(&ctx, "test-frame", Some(output))
        .await
        .expect_err("extract should map snapshot transport failures");

    assert!(matches!(
        error.downcast_ref::<FigexError>(),
        Some(FigexError::SnapshotFailed(_))
    ));
}

#[tokio::test]
async fn extract_command_propagates_output_write_failures() {
    let (ws_listener, ws_port) = mock_cdp_server::bind_ws_listener().await;
    tokio::spawn(mock_cdp_server::serve_two(ws_listener));
    let http_port =
        mock_cdp_server::spawn_figma_cdp_server("t-extract-write-fail", "Figma - Extract", ws_port)
            .await;

    let ctx = extract_context_with_pretty("127.0.0.1".to_string(), http_port, false);
    let tmp_dir = tempfile::TempDir::new().expect("temp dir should be created");

    let error = extract::run(&ctx, "test-frame", Some(tmp_dir.path().to_path_buf()))
        .await
        .expect_err("extract should fail when writing to a directory path");

    assert!(
        error.downcast_ref::<std::io::Error>().is_some(),
        "write errors should preserve the underlying io::Error: {error:#}"
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

struct CurrentDirGuard(PathBuf);

impl CurrentDirGuard {
    fn capture() -> Self {
        Self(std::env::current_dir().expect("current dir should be readable"))
    }
}

impl Drop for CurrentDirGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}

fn current_dir_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

async fn serve_two_with_snapshot_not_ok(listener: TcpListener) {
    serve_ws_with_handler(&listener, |_| json!({ "result": {} })).await;
    serve_ws_with_handler(&listener, |method| match method {
        "Runtime.enable" => json!({ "result": {} }),
        "Runtime.evaluate" => json!({
            "result": {
                "result": {
                    "type": "object",
                    "value": { "ok": false, "ts": 1_700_000_000_000u64 }
                }
            }
        }),
        _ => json!({ "result": {} }),
    })
    .await;
}

async fn serve_two_with_snapshot_error(listener: TcpListener) {
    serve_ws_with_handler(&listener, |_| json!({ "result": {} })).await;
    serve_ws_with_handler(&listener, |method| match method {
        "Runtime.enable" => json!({ "result": {} }),
        "Runtime.evaluate" => json!({
            "error": { "code": -32002, "message": "snapshot transport failed" }
        }),
        _ => json!({ "result": {} }),
    })
    .await;
}

async fn serve_ws_with_handler<F>(listener: &TcpListener, mut response_for: F)
where
    F: FnMut(&str) -> serde_json::Value,
{
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream)
            .await
            .expect("websocket handshake should succeed");
        let (mut sink, mut stream) = ws.split();

        while let Some(Ok(Message::Text(text))) = stream.next().await {
            let request: serde_json::Value =
                serde_json::from_str(&text).expect("request should be valid JSON");
            let id = request["id"].as_u64().unwrap_or(0);
            let method = request["method"].as_str().unwrap_or("");
            let payload = response_for(method);

            let mut response = json!({ "id": id });
            response
                .as_object_mut()
                .expect("response should be an object")
                .extend(
                    payload
                        .as_object()
                        .expect("payload should be an object")
                        .clone(),
                );

            if sink
                .send(Message::Text(response.to_string()))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}

fn make_snapshot() -> RawFrameSnapshot {
    use figex_cli_core::{
        RawAutoLayout, RawBounds, RawColor, RawCornerRadius, RawEdge, RawEffect, RawExportHint,
        RawFill, RawFrameMeta, RawNode, RawStroke, RawText, RawTextStyle,
    };

    RawFrameSnapshot {
        version: "1".to_string(),
        frame: RawFrameMeta {
            frame_ref: Some("my-frame".to_string()),
            frame_id: Some("1:2".to_string()),
            frame_name: Some("Home Screen".to_string()),
            transport: "cdp".to_string(),
            host: "127.0.0.1".to_string(),
            port: 9222,
            captured_at: 1_700_000_000_000,
            tool_version: "0.1.0".to_string(),
        },
        nodes: vec![RawNode {
            id: "1:2".to_string(),
            parent_id: None,
            name: "Home Screen".to_string(),
            node_type: "FRAME".to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            corner_radius: Some(RawCornerRadius {
                top_left: 4.0,
                top_right: 4.0,
                bottom_right: 4.0,
                bottom_left: 4.0,
            }),
            component_id: None,
            is_instance: false,
        }],
        edges: vec![RawEdge {
            parent_id: "1:1".to_string(),
            child_id: "1:2".to_string(),
            index: 0,
        }],
        texts: vec![RawText {
            node_id: "1:3".to_string(),
            content: "Hello".to_string(),
            style: Some(RawTextStyle {
                font_family: Some("Inter".to_string()),
                font_size: Some(16.0),
                font_weight: Some(400),
                line_height: None,
                letter_spacing: None,
                text_align: Some("left".to_string()),
            }),
        }],
        fills: vec![RawFill {
            node_id: "1:2".to_string(),
            fill_type: "solid".to_string(),
            color: Some(RawColor {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }),
            opacity: None,
            visible: true,
            blend_mode: Some("normal".to_string()),
        }],
        bounds: vec![RawBounds {
            node_id: "1:2".to_string(),
            x: 0.0,
            y: 0.0,
            width: 375.0,
            height: 812.0,
            bounds_type: "absolute".to_string(),
        }],
        auto_layout: vec![RawAutoLayout {
            node_id: "1:2".to_string(),
            direction: "vertical".to_string(),
            gap: 16.0,
            padding_top: 24.0,
            padding_right: 16.0,
            padding_bottom: 24.0,
            padding_left: 16.0,
            align_main: Some("flex-start".to_string()),
            align_cross: Some("stretch".to_string()),
            wrap: false,
            sizing_mode_main: None,
            sizing_mode_cross: None,
        }],
        effects: vec![RawEffect {
            node_id: "1:2".to_string(),
            effect_type: "drop_shadow".to_string(),
            visible: true,
            radius: 4.0,
            color: Some(RawColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.15,
            }),
            offset_x: Some(0.0),
            offset_y: Some(2.0),
        }],
        strokes: vec![RawStroke {
            node_id: "1:2".to_string(),
            stroke_type: "solid".to_string(),
            color: Some(RawColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }),
            weight: 1.0,
            align: Some("inside".to_string()),
            visible: true,
            blend_mode: None,
        }],
        export_hints: vec![RawExportHint {
            node_id: "1:2".to_string(),
            format: "PNG".to_string(),
            suffix: Some("@2x".to_string()),
            scale: Some(2.0),
        }],
    }
}
