use std::time::{Duration, Instant};

use async_trait::async_trait;
use figex_cli_core::RuntimeError;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::js_bridge;
use super::probe::{discover_best_target, DiscoveredTarget, ProbeConfig};
use super::protocol::{CdpRequest, CdpResponse};
use crate::client::{RuntimeClient, RuntimeSession};
use crate::health::RuntimeHealth;
use crate::snapshot::{RuntimeFrameRequest, RuntimeFrameSnapshot};

// ---------------------------------------------------------------------------
// CdpClient
// ---------------------------------------------------------------------------

/// CDP-based implementation of `RuntimeClient`.
///
/// Probes for a Figma Desktop target using `/json/list`, connects via
/// WebSocket, and drives the CDP Runtime domain.
pub struct CdpClient {
    host: Option<String>,
    port: Option<u16>,
    timeout: Duration,
}

impl CdpClient {
    pub fn new(host: Option<String>, port: Option<u16>, timeout: Duration) -> Self {
        Self {
            host,
            port,
            timeout,
        }
    }

    fn probe_config(&self) -> ProbeConfig {
        ProbeConfig {
            host: self.host.clone(),
            port: self.port,
            timeout: self.timeout,
        }
    }
}

// ---------------------------------------------------------------------------
// Internal WebSocket session
// ---------------------------------------------------------------------------

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

struct WsSession {
    sink: futures_util::stream::SplitSink<WsStream, Message>,
    stream: futures_util::stream::SplitStream<WsStream>,
    next_id: u32,
}

impl WsSession {
    async fn connect(ws_url: &str) -> Result<Self, RuntimeError> {
        let ws = match connect_async(ws_url).await {
            Ok((ws, _)) => ws,
            Err(_) => return Err(RuntimeError::AttachFailed),
        };
        let (sink, stream) = ws.split();
        Ok(Self {
            sink,
            stream,
            next_id: 1,
        })
    }

    /// Send a CDP method call and wait for the matching response.
    async fn call(
        &mut self,
        method: &str,
        params: serde_json::Value,
        call_timeout: Duration,
    ) -> Result<serde_json::Value, RuntimeError> {
        let id = self.next_id;
        self.next_id += 1;

        let req = CdpRequest {
            id,
            method: method.to_string(),
            params,
        };
        let text = serde_json::to_string(&req).expect("CDP requests should always serialize");

        let send_result = self.sink.send(Message::Text(text)).await;
        map_ws_send_result(send_result)?;
        wait_for_matching_response(&mut self.stream, id, call_timeout).await
    }
}

#[doc(hidden)]
pub fn map_ws_send_result(
    result: Result<(), tokio_tungstenite::tungstenite::Error>,
) -> Result<(), RuntimeError> {
    match result {
        Ok(()) => Ok(()),
        Err(_) => Err(RuntimeError::AttachFailed),
    }
}

#[doc(hidden)]
pub fn map_ws_incoming_message(
    message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
    id: u32,
) -> Result<Option<serde_json::Value>, RuntimeError> {
    let message = match message {
        Some(Ok(message)) => message,
        Some(Err(_)) => return Err(RuntimeError::AttachFailed),
        None => return Err(RuntimeError::AttachFailed),
    };

    let body = match message {
        Message::Text(body) => body,
        _ => return Ok(None),
    };

    map_ws_text_message(&body, id)
}

fn map_ws_text_message(body: &str, id: u32) -> Result<Option<serde_json::Value>, RuntimeError> {
    let resp = match serde_json::from_str::<CdpResponse>(body) {
        Ok(resp) => resp,
        Err(_) => return Ok(None),
    };
    if resp.id != Some(id) {
        return Ok(None);
    }
    if let Some(err) = resp.error {
        let message = format!("code={}, message={}", err.code, err.message);
        return Err(RuntimeError::EvaluateFailed(message));
    }

    let value = match resp.result {
        Some(value) => value,
        None => serde_json::Value::Null,
    };

    Ok(Some(value))
}

async fn read_until_matching_response(
    stream: &mut futures_util::stream::SplitStream<WsStream>,
    id: u32,
) -> Result<serde_json::Value, RuntimeError> {
    loop {
        let value = map_ws_incoming_message(stream.next().await, id)?;
        if let Some(value) = value {
            return Ok(value);
        }
    }
}

async fn wait_for_matching_response(
    stream: &mut futures_util::stream::SplitStream<WsStream>,
    id: u32,
    call_timeout: Duration,
) -> Result<serde_json::Value, RuntimeError> {
    let result = timeout(call_timeout, read_until_matching_response(stream, id)).await;

    match result {
        Ok(value) => value,
        Err(_) => Err(RuntimeError::Timeout),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn health_from_no_target(
    fallback_host: String,
    fallback_port: u16,
    errors: Vec<String>,
) -> RuntimeHealth {
    RuntimeHealth {
        transport: "cdp".to_string(),
        host: fallback_host,
        port: fallback_port,
        target_id: None,
        target_title: None,
        target_url: None,
        target_score: None,
        ping_ok: false,
        snapshot_ok: false,
        latency_ms: None,
        ping_script_version: js_bridge::PING_SCRIPT_VERSION.to_string(),
        snapshot_script_version: js_bridge::SNAPSHOT_SCRIPT_VERSION.to_string(),
        warnings: vec![],
        errors,
    }
}

fn health_from_target_no_ws(target: &DiscoveredTarget, errors: Vec<String>) -> RuntimeHealth {
    RuntimeHealth {
        transport: "cdp".to_string(),
        host: target.host.clone(),
        port: target.port,
        target_id: Some(target.target_id.clone()),
        target_title: Some(target.title.clone()),
        target_url: Some(target.url.clone()),
        target_score: Some(target.score),
        ping_ok: false,
        snapshot_ok: false,
        latency_ms: None,
        ping_script_version: js_bridge::PING_SCRIPT_VERSION.to_string(),
        snapshot_script_version: js_bridge::SNAPSHOT_SCRIPT_VERSION.to_string(),
        warnings: vec![],
        errors,
    }
}

fn result_value(result: &serde_json::Value) -> Option<&serde_json::Value> {
    result.get("value")
}

fn result_ok(value: &serde_json::Value) -> Option<&serde_json::Value> {
    value.get("ok")
}

fn result_ts(value: &serde_json::Value) -> Option<&serde_json::Value> {
    value.get("ts")
}

// ---------------------------------------------------------------------------
// RuntimeClient impl
// ---------------------------------------------------------------------------

#[async_trait]
impl RuntimeClient for CdpClient {
    async fn attach(&self) -> Result<RuntimeSession, RuntimeError> {
        let target = discover_best_target(&self.probe_config()).await?;
        let mut session = WsSession::connect(&target.ws_debugger_url).await?;
        session
            .call("Runtime.enable", json!({}), self.timeout)
            .await?;

        Ok(RuntimeSession {
            target_id: target.target_id,
            target_title: target.title,
            target_url: target.url,
            host: target.host,
            port: target.port,
        })
    }

    async fn health(&self) -> Result<RuntimeHealth, RuntimeError> {
        let mut warnings: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        let fallback_host = match &self.host {
            Some(host) => host.clone(),
            None => "127.0.0.1".to_string(),
        };
        let fallback_port = self.port.unwrap_or(0);

        // -- Target discovery --
        let target = match discover_best_target(&self.probe_config()).await {
            Ok(t) => t,
            Err(e) => {
                errors.push(format!("target discovery failed: {e}"));
                return Ok(health_from_no_target(fallback_host, fallback_port, errors));
            }
        };

        // -- WebSocket connect --
        let mut ws = match WsSession::connect(&target.ws_debugger_url).await {
            Ok(s) => s,
            Err(e) => {
                errors.push(format!("websocket connect failed: {e}"));
                return Ok(health_from_target_no_ws(&target, errors));
            }
        };

        // -- Runtime.enable --
        if let Err(e) = ws.call("Runtime.enable", json!({}), self.timeout).await {
            errors.push(format!("Runtime.enable failed: {e}"));
        }

        // -- Ping --
        let ping_start = Instant::now();
        let ping_ok = ws
            .call(
                "Runtime.evaluate",
                json!({ "expression": js_bridge::ping_script(), "returnByValue": true }),
                self.timeout,
            )
            .await
            .is_ok();
        let latency_ms = Some(u64::try_from(ping_start.elapsed().as_millis()).unwrap_or(u64::MAX));

        if !ping_ok {
            errors.push("ping failed".to_string());
        }

        // -- Snapshot check --
        let snapshot_ok = ws
            .call(
                "Runtime.evaluate",
                json!({ "expression": js_bridge::snapshot_script(), "returnByValue": true }),
                self.timeout,
            )
            .await
            .is_ok();

        if !snapshot_ok {
            warnings.push("snapshot check failed".to_string());
        }

        Ok(RuntimeHealth {
            transport: "cdp".to_string(),
            host: target.host,
            port: target.port,
            target_id: Some(target.target_id),
            target_title: Some(target.title),
            target_url: Some(target.url),
            target_score: Some(target.score),
            ping_ok,
            snapshot_ok,
            latency_ms,
            ping_script_version: js_bridge::PING_SCRIPT_VERSION.to_string(),
            snapshot_script_version: js_bridge::SNAPSHOT_SCRIPT_VERSION.to_string(),
            warnings,
            errors,
        })
    }

    async fn snapshot_frame(
        &self,
        _request: RuntimeFrameRequest,
    ) -> Result<RuntimeFrameSnapshot, RuntimeError> {
        let target = discover_best_target(&self.probe_config()).await?;
        let mut ws = WsSession::connect(&target.ws_debugger_url).await?;
        ws.call("Runtime.enable", json!({}), self.timeout).await?;

        let result = ws
            .call(
                "Runtime.evaluate",
                json!({ "expression": js_bridge::snapshot_script(), "returnByValue": true }),
                self.timeout,
            )
            .await;
        let result = match result {
            Ok(result) => result,
            Err(_) => return Err(RuntimeError::SnapshotFailed),
        };

        // Extract ok and ts from the JS eval result ({ ok: bool, ts: number }).
        let value = result.get("result").and_then(result_value);
        let ok = value
            .and_then(result_ok)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        let ts = value
            .and_then(result_ts)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0);

        Ok(RuntimeFrameSnapshot { ok, ts })
    }
}
