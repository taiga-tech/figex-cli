use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use figex_cli_core::{FigexError, RuntimeError};
use figex_cli_runtime::{CdpClient, RuntimeClient};

use crate::app::AppContext;

pub async fn run(ctx: &AppContext, stdout: &mut dyn Write) -> Result<()> {
    let client = build_client(ctx);
    let session = client.attach().await.map_err(|e| match e {
        RuntimeError::TargetNotFound => FigexError::TargetNotFound(e),
        _ => FigexError::ConnectionFailed(e),
    })?;

    writeln!(stdout, "Attached to Figma runtime")?;
    writeln!(stdout, "  host:      {}", session.host)?;
    writeln!(stdout, "  port:      {}", session.port)?;
    writeln!(stdout, "  target_id: {}", session.target_id)?;
    writeln!(stdout, "  title:     {}", session.target_title)?;
    writeln!(stdout, "  url:       {}", session.target_url)?;

    Ok(())
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
