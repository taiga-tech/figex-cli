#![allow(dead_code)]

use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};

/// Bind a TCP listener on a random port and return it with its port number.
pub async fn bind_ws_listener() -> (TcpListener, u16) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    (listener, port)
}

/// Accept one WebSocket connection and serve minimal CDP responses.
///
/// Handles:
/// - `Runtime.enable` → `{}`
/// - `Runtime.evaluate` → `{ result: { type: "object", value: { ok: true, ts: ... } } }`
pub async fn serve_one(listener: TcpListener) {
    serve_one_with_handler(listener, success_response).await;
}

/// Accept one connection and inject malformed and unrelated frames before success.
pub async fn serve_one_with_noise_then_success(listener: TcpListener) {
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream).await.unwrap();
        let (mut sink, mut stream) = ws.split();

        if let Some(Ok(Message::Text(text))) = stream.next().await {
            let req: serde_json::Value = serde_json::from_str(&text).unwrap();
            let id = req["id"].as_u64().unwrap_or(0);
            let method = req["method"].as_str().unwrap_or("");

            sink.send(Message::Text("not-json".to_string()))
                .await
                .unwrap();
            sink.send(Message::Text(
                json!({ "id": id + 1, "result": {} }).to_string(),
            ))
            .await
            .unwrap();
            sink.send(Message::Binary(vec![0xde, 0xad])).await.unwrap();

            let resp = json!({ "id": id, "result": success_response(method) });
            sink.send(Message::Text(resp.to_string())).await.unwrap();
        }
    }
}

/// Accept one connection and fail `Runtime.enable` with a CDP error response.
pub async fn serve_one_with_runtime_enable_error(listener: TcpListener) {
    serve_one_with_handler(listener, |method| match method {
        "Runtime.enable" => json!({
            "error": { "code": -32000, "message": "runtime enable failed" }
        }),
        _ => json!({
            "result": success_response(method)
        }),
    })
    .await;
}

/// Accept one connection and omit the `result` field from `Runtime.enable`.
pub async fn serve_one_with_missing_result_for_runtime_enable(listener: TcpListener) {
    serve_one_with_handler(listener, |method| match method {
        "Runtime.enable" => json!({}),
        _ => success_response(method),
    })
    .await;
}

/// Accept one connection, read the first CDP request, and close the socket.
pub async fn serve_one_and_close(listener: TcpListener) {
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream).await.unwrap();
        let (_sink, mut stream) = ws.split();
        let _ = stream.next().await;
    }
}

/// Accept one connection, send a close frame after the first request, then drop.
pub async fn serve_one_with_close_frame(listener: TcpListener) {
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream).await.unwrap();
        let (mut sink, mut stream) = ws.split();
        let _ = stream.next().await;
        let _ = sink.send(Message::Close(None)).await;
    }
}

/// Accept one connection, read the first request, and keep the socket open without responding.
pub async fn serve_one_and_hang(listener: TcpListener, delay: Duration) {
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream).await.unwrap();
        let (_sink, mut stream) = ws.split();
        let _ = stream.next().await;
        tokio::time::sleep(delay).await;
    }
}

/// Accept one connection, complete the handshake, and immediately drop the socket.
#[allow(deprecated)]
pub async fn serve_one_and_drop_immediately(listener: TcpListener) {
    if let Ok((stream, _)) = listener.accept().await {
        let mut ws = accept_async(stream).await.unwrap();
        // Resetting the socket makes the client's first send fail deterministically.
        let _ = ws.get_mut().set_linger(Some(Duration::ZERO));
        drop(ws);
    }
}

/// Accept one connection and fail the first `Runtime.evaluate` request.
pub async fn serve_one_with_ping_failure(listener: TcpListener) {
    let mut evaluate_calls = 0usize;
    serve_one_with_request_handler(listener, move |req| {
        let method = req["method"].as_str().unwrap_or("");
        match method {
            "Runtime.enable" => success_response(method),
            "Runtime.evaluate" => {
                evaluate_calls += 1;
                if evaluate_calls == 1 {
                    json!({
                        "error": { "code": -32001, "message": "ping failed" }
                    })
                } else {
                    success_response(method)
                }
            }
            _ => json!({ "result": {} }),
        }
    })
    .await;
}

/// Accept one connection and fail the second `Runtime.evaluate` request.
pub async fn serve_one_with_snapshot_failure(listener: TcpListener) {
    let mut evaluate_calls = 0usize;
    serve_one_with_request_handler(listener, move |req| {
        let method = req["method"].as_str().unwrap_or("");
        match method {
            "Runtime.enable" => success_response(method),
            "Runtime.evaluate" => {
                evaluate_calls += 1;
                if evaluate_calls == 2 {
                    json!({
                        "error": { "code": -32002, "message": "snapshot failed" }
                    })
                } else {
                    success_response(method)
                }
            }
            _ => json!({ "result": {} }),
        }
    })
    .await;
}

async fn serve_one_with_handler<F>(listener: TcpListener, mut response_for: F)
where
    F: FnMut(&str) -> serde_json::Value,
{
    serve_one_with_request_handler(listener, move |req| {
        let method = req["method"].as_str().unwrap_or("");
        response_for(method)
    })
    .await;
}

async fn serve_one_with_request_handler<F>(listener: TcpListener, mut response_for: F)
where
    F: FnMut(&serde_json::Value) -> serde_json::Value,
{
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream).await.unwrap();
        let (mut sink, mut stream) = ws.split();
        while let Some(Ok(Message::Text(text))) = stream.next().await {
            let req: serde_json::Value = serde_json::from_str(&text).unwrap();
            let id = req["id"].as_u64().unwrap_or(0);

            let mut resp = json!({ "id": id });
            let payload = response_for(&req);
            resp.as_object_mut()
                .unwrap()
                .extend(payload.as_object().unwrap().clone());
            if sink.send(Message::Text(resp.to_string())).await.is_err() {
                break;
            }
        }
    }
}

fn success_response(method: &str) -> serde_json::Value {
    json!({
        "result": match method {
            "Runtime.enable" => json!({}),
            "Runtime.evaluate" => json!({
                "result": {
                    "type": "object",
                    "value": { "ok": true, "ts": 1_700_000_000_000u64 }
                }
            }),
            _ => json!({}),
        }
    })
}

/// Mount a `/json/version` mock that returns a minimal CDP version object.
pub async fn mount_version(server: &wiremock::MockServer) {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    let version = json!({
        "Browser": "Chrome/114.0.0.0",
        "Protocol-Version": "1.3"
    });

    Mock::given(method("GET"))
        .and(path("/json/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&version))
        .mount(server)
        .await;
}

/// Mount a `/json/list` mock that returns a single Figma target pointing at `ws_port`.
pub async fn mount_figma_target(
    server: &wiremock::MockServer,
    id: &str,
    title: &str,
    ws_port: u16,
) {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    let targets = json!([{
        "id": id,
        "title": title,
        "url": format!("https://www.figma.com/file/{}", id),
        "type": "page",
        "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/", ws_port)
    }]);

    Mock::given(method("GET"))
        .and(path("/json/list"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&targets))
        .mount(server)
        .await;
}

/// Mount both `/json/version` and `/json/list` for a Figma target.
///
/// Convenience wrapper used by most mock-based tests.
pub async fn mount_figma_cdp(server: &wiremock::MockServer, id: &str, title: &str, ws_port: u16) {
    mount_version(server).await;
    mount_figma_target(server, id, title, ws_port).await;
}
