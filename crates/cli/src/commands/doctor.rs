use std::io::{self, Write};
use std::time::Duration;

use anyhow::Result;
use figex_cli_core::RuntimeError;
use figex_cli_runtime::{CdpClient, RuntimeClient, RuntimeHealth};

use crate::app::AppContext;

#[inline(never)]
pub async fn run(ctx: &AppContext) -> Result<()> {
    let mut stdout = io::stdout();
    render_health(ctx, build_client(ctx).health().await, &mut stdout)
}

#[doc(hidden)]
#[inline(never)]
pub fn render_health(
    ctx: &AppContext,
    health_result: std::result::Result<RuntimeHealth, RuntimeError>,
    writer: &mut dyn Write,
) -> Result<()> {
    let health = match health_result {
        Ok(health) => health,
        Err(error) => fallback_health(ctx, error),
    };

    if ctx.settings.json {
        let json = format_health_json(&health, ctx.settings.pretty);
        write_line(writer, &json)?;
    } else {
        write_health_human(writer, &health)?;
    }

    Ok(())
}

#[doc(hidden)]
#[inline(never)]
pub fn fallback_health(ctx: &AppContext, error: RuntimeError) -> RuntimeHealth {
    let (ping_script_version, snapshot_script_version) = figex_cli_runtime::transport_versions();

    RuntimeHealth {
        transport: "cdp".to_string(),
        host: ctx.settings.host.clone(),
        port: ctx.settings.port,
        target_id: None,
        target_title: None,
        target_url: None,
        target_score: None,
        ping_ok: false,
        snapshot_ok: false,
        latency_ms: None,
        ping_script_version: ping_script_version.to_string(),
        snapshot_script_version: snapshot_script_version.to_string(),
        warnings: vec![],
        errors: vec![format!("{error}")],
    }
}

#[doc(hidden)]
#[inline(never)]
pub fn format_health_json(health: &RuntimeHealth, pretty: bool) -> String {
    if pretty {
        serde_json::to_string_pretty(health).expect("RuntimeHealth should serialize to pretty JSON")
    } else {
        serde_json::to_string(health).expect("RuntimeHealth should serialize to JSON")
    }
}

#[doc(hidden)]
#[inline(never)]
pub fn write_health_human(writer: &mut dyn Write, health: &RuntimeHealth) -> Result<()> {
    write_line(writer, format!("transport:  {}", health.transport))?;
    write_line(writer, format!("host:       {}", health.host))?;
    write_line(writer, format!("port:       {}", health.port))?;
    write_line(
        writer,
        format!("target_id:  {}", health.target_id.as_deref().unwrap_or("-")),
    )?;
    write_line(
        writer,
        format!(
            "title:      {}",
            health.target_title.as_deref().unwrap_or("-")
        ),
    )?;
    write_line(
        writer,
        format!(
            "url:        {}",
            health.target_url.as_deref().unwrap_or("-")
        ),
    )?;
    write_line(
        writer,
        format!("ping:       {}", if health.ping_ok { "ok" } else { "fail" }),
    )?;
    write_line(
        writer,
        format!(
            "snapshot:   {}",
            if health.snapshot_ok { "ok" } else { "fail" }
        ),
    )?;
    if let Some(latency_ms) = health.latency_ms {
        write_line(writer, format!("latency:    {latency_ms}ms"))?;
    }
    for warning in &health.warnings {
        write_line(writer, format!("warning:    {warning}"))?;
    }
    for error in &health.errors {
        write_line(writer, format!("error:      {error}"))?;
    }

    Ok(())
}

fn write_line(writer: &mut dyn Write, line: impl AsRef<str>) -> Result<()> {
    let line = format!("{}\n", line.as_ref());
    writer.write_all(line.as_bytes())?;
    Ok(())
}

#[inline(never)]
fn build_client(ctx: &AppContext) -> CdpClient {
    CdpClient::new(
        ctx.settings
            .host_explicit
            .then(|| ctx.settings.host.clone()),
        ctx.settings.port_explicit.then_some(ctx.settings.port),
        Duration::from_millis(ctx.settings.timeout_ms),
    )
}
