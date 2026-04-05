use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use figex_cli_core::{FigexError, RawFrameMeta, RawFrameSnapshot, RuntimeError};
use figex_cli_runtime::{CdpClient, RuntimeClient, RuntimeFrameRequest, RuntimeSession};

use crate::app::AppContext;

pub async fn run(ctx: &AppContext, frame: &str, output: Option<PathBuf>) -> Result<()> {
    let output = output.unwrap_or_else(|| PathBuf::from("raw.json"));

    let client = build_client(ctx);

    let session = client.attach().await.map_err(|e| match e {
        RuntimeError::TargetNotFound => FigexError::TargetNotFound(e),
        _ => FigexError::ConnectionFailed(e),
    })?;

    let request = RuntimeFrameRequest {
        frame_ref: Some(frame.to_string()),
    };
    let snapshot = client
        .snapshot_frame(request)
        .await
        .map_err(FigexError::SnapshotFailed)?;

    if !snapshot.ok {
        return Err(FigexError::SnapshotFailed(RuntimeError::SnapshotFailed).into());
    }

    let raw = map_to_raw(frame, &session, snapshot.ts);

    let json = if ctx.settings.pretty {
        serde_json::to_string_pretty(&raw)
    } else {
        serde_json::to_string(&raw)
    }
    .expect("RawFrameSnapshot should always serialize to JSON");

    std::fs::write(&output, json)?;

    Ok(())
}

fn map_to_raw(frame_ref: &str, session: &RuntimeSession, captured_at: u64) -> RawFrameSnapshot {
    RawFrameSnapshot {
        version: "1".to_string(),
        frame: RawFrameMeta {
            frame_ref: Some(frame_ref.to_string()),
            frame_id: None,
            frame_name: None,
            transport: "cdp".to_string(),
            host: session.host.clone(),
            port: session.port,
            captured_at,
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        nodes: vec![],
        edges: vec![],
        texts: vec![],
        fills: vec![],
        bounds: vec![],
        auto_layout: vec![],
        effects: vec![],
        strokes: vec![],
        export_hints: vec![],
    }
}

fn build_client(ctx: &AppContext) -> CdpClient {
    CdpClient::new(
        ctx.settings
            .host_explicit
            .then(|| ctx.settings.host.clone()),
        ctx.settings.port_explicit.then_some(ctx.settings.port),
        Duration::from_millis(ctx.settings.timeout_ms),
    )
}
