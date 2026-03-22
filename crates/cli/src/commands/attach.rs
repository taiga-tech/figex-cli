use std::time::Duration;

use anyhow::Result;
use figex_cli_core::{FigexError, RuntimeError};
use figex_cli_runtime::{CdpClient, RuntimeClient};

use crate::app::AppContext;

pub async fn run(ctx: &AppContext) -> Result<()> {
    let client = build_client(ctx);
    let session = client.attach().await.map_err(|e| match e {
        RuntimeError::TargetNotFound => FigexError::TargetNotFound(e),
        _ => FigexError::ConnectionFailed(e),
    })?;

    println!("Attached to Figma runtime");
    println!("  host:      {}", session.host);
    println!("  port:      {}", session.port);
    println!("  target_id: {}", session.target_id);
    println!("  title:     {}", session.target_title);
    println!("  url:       {}", session.target_url);

    Ok(())
}

fn build_client(ctx: &AppContext) -> CdpClient {
    CdpClient::new(
        Some(ctx.settings.host.clone()),
        Some(ctx.settings.port),
        Duration::from_millis(ctx.settings.timeout_ms),
    )
}
