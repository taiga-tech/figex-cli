#![allow(dead_code)]

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};

pub async fn bind_ws_listener() -> (TcpListener, u16) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    (listener, port)
}

pub async fn serve_one(listener: TcpListener) {
    serve_one_with_handler(listener, success_response).await;
}

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

async fn serve_one_with_handler<F>(listener: TcpListener, mut response_for: F)
where
    F: FnMut(&str) -> serde_json::Value,
{
    if let Ok((stream, _)) = listener.accept().await {
        let ws = accept_async(stream).await.unwrap();
        let (mut sink, mut stream) = ws.split();
        while let Some(Ok(Message::Text(text))) = stream.next().await {
            let req: serde_json::Value = serde_json::from_str(&text).unwrap();
            let id = req["id"].as_u64().unwrap_or(0);
            let method = req["method"].as_str().unwrap_or("");
            let payload = response_for(method);

            let mut resp = json!({ "id": id });
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

pub async fn spawn_figma_cdp_server(id: &str, title: &str, ws_port: u16) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let id = id.to_string();
    let title = title.to_string();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };

            let mut buffer = [0u8; 2048];
            let Ok(read_len) = stream.read(&mut buffer).await else {
                continue;
            };
            if read_len == 0 {
                continue;
            }

            let request = String::from_utf8_lossy(&buffer[..read_len]);
            let is_version = request.starts_with("GET /json/version ");
            let is_list = request.starts_with("GET /json/list ");

            let body = if is_version {
                json!({
                    "Browser": "Chrome/114.0.0.0",
                    "Protocol-Version": "1.3"
                })
                .to_string()
            } else if is_list {
                json!([{
                    "id": id,
                    "title": title,
                    "url": format!("https://www.figma.com/file/{}", id),
                    "type": "page",
                    "webSocketDebuggerUrl": format!("ws://127.0.0.1:{}/", ws_port)
                }])
                .to_string()
            } else {
                "{}".to_string()
            };

            let status = if is_version || is_list {
                "200 OK"
            } else {
                "404 Not Found"
            };

            let response = format!(
                "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes()).await;
        }
    });

    port
}
