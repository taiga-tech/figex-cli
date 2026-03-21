use std::io::{self, Write};

use anyhow::Result;

use crate::cli::{Cli, Commands, ExtractSubcommand, InspectSubcommand, LogLevel, Transport};
use crate::commands;
use crate::config::{self, CliOverrides, Settings};
use crate::logo;

pub const DEFAULT_LOGO_FONT: &str = "DOS Rebel";
pub const DEFAULT_LOGO_TEXT: &str = "FIGEX CLI";

// ---------------------------------------------------------------------------
// AppContext
// ---------------------------------------------------------------------------

pub struct AppContext {
    pub settings: Settings,
}

impl AppContext {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

// ---------------------------------------------------------------------------
// Entry points
// ---------------------------------------------------------------------------

pub fn run(cli: Cli) -> Result<()> {
    run_io_with_logo(
        cli,
        DEFAULT_LOGO_FONT,
        DEFAULT_LOGO_TEXT,
        &mut io::stdout(),
        &mut io::stderr(),
    )
}

pub fn run_io(cli: Cli, stdout: &mut dyn Write, stderr: &mut dyn Write) -> Result<()> {
    run_io_with_logo(cli, DEFAULT_LOGO_FONT, DEFAULT_LOGO_TEXT, stdout, stderr)
}

pub fn run_io_with_logo(
    cli: Cli,
    logo_font: &str,
    logo_text: &str,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> Result<()> {
    match logo::render_logo(logo_font, logo_text) {
        Ok(rendered) => logo::write_rendered_logo(stdout, &rendered)?,
        Err(e) => writeln!(stderr, "Error printing logo: {e}")?,
    }

    let settings = build_settings(&cli);
    let ctx = AppContext::new(settings);

    match cli.command {
        None => writeln!(stdout, "Run `figex --help` for usage.")?,
        Some(cmd) => return dispatch(cmd, &ctx),
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Settings construction
// ---------------------------------------------------------------------------

// `pub` so that integration tests in tests/ can call these directly.
#[doc(hidden)]
pub fn build_settings(cli: &Cli) -> Settings {
    let overrides = CliOverrides {
        json: cli.json,
        pretty: cli.pretty,
        timeout_ms: cli.timeout_ms,
        transport: cli.transport.as_ref().map(transport_to_str),
        host: cli.host.clone(),
        port: cli.port,
        log_level: cli.log_level.as_ref().map(log_level_to_str),
    };

    // Locate config file:
    //   1. --config CLI arg
    //   2. figex.toml found by walking up from cwd
    //   3. ~/.config/figex/config.toml (global default)
    let config_path = cli
        .config
        .clone()
        .or_else(|| config::find_config_file(&std::env::current_dir().unwrap_or_default()))
        .or_else(|| dirs::config_dir().map(|d| d.join("figex").join("config.toml")));

    let file_cfg = config_path.as_deref().map(config::load_file_config);

    config::resolve_settings(&overrides, |k| std::env::var(k).ok(), file_cfg.as_ref())
}

#[doc(hidden)]
pub fn transport_to_str(t: &Transport) -> String {
    match t {
        Transport::Cdp => "cdp".to_string(),
        Transport::Mcp => "mcp".to_string(),
        Transport::Auto => "auto".to_string(),
    }
}

#[doc(hidden)]
pub fn log_level_to_str(l: &LogLevel) -> String {
    match l {
        LogLevel::Error => "error".to_string(),
        LogLevel::Warn => "warn".to_string(),
        LogLevel::Info => "info".to_string(),
        LogLevel::Debug => "debug".to_string(),
        LogLevel::Trace => "trace".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

fn dispatch(command: Commands, ctx: &AppContext) -> Result<()> {
    match command {
        Commands::Attach => commands::attach::run(ctx),
        Commands::Doctor => commands::doctor::run(ctx),
        Commands::Inspect {
            subcommand: InspectSubcommand::Frame { frame_ref },
        } => commands::inspect::run(ctx, &frame_ref),
        Commands::Extract {
            subcommand: ExtractSubcommand::Frame { frame_ref, output },
        } => commands::extract::run(ctx, &frame_ref, output),
        Commands::Features { input, output } => commands::features::run(ctx, input, output),
        Commands::Normalize { input, output } => commands::normalize::run(ctx, input, output),
        Commands::Report { input, output } => commands::report::run(ctx, input, output),
    }
}
