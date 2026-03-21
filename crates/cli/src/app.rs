use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::Result;

use crate::cli::{Cli, Commands};
use crate::commands;
use crate::logo;

pub const DEFAULT_LOGO_FONT: &str = "DOS Rebel";
pub const DEFAULT_LOGO_TEXT: &str = "FIGEX CLI";

pub struct AppContext {
    pub verbose: bool,
    pub config_path: PathBuf,
}

impl AppContext {
    pub fn new(verbose: bool, config: Option<PathBuf>) -> Self {
        let config_path = config.unwrap_or_else(|| config_dir_with_base(dirs::config_dir()));
        Self {
            verbose,
            config_path,
        }
    }
}

pub fn config_dir_with_base(base: Option<PathBuf>) -> PathBuf {
    base.unwrap_or_else(|| PathBuf::from("."))
        .join("figex")
        .join("config.json")
}

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

    let ctx = AppContext::new(cli.verbose, cli.config);

    match cli.command {
        None => writeln!(stdout, "Run `figex-cli --help` for usage.")?,
        Some(cmd) => return dispatch(cmd, &ctx),
    }

    Ok(())
}

fn dispatch(command: Commands, ctx: &AppContext) -> Result<()> {
    match command {
        Commands::Attach => commands::attach::run(ctx),
        Commands::Doctor => commands::doctor::run(ctx),
        Commands::Inspect { frame } => commands::inspect::run(ctx, &frame),
        Commands::Extract { frame, output } => commands::extract::run(ctx, &frame, output),
        Commands::Features { input, output } => commands::features::run(ctx, input, output),
        Commands::Normalize { input, output } => commands::normalize::run(ctx, input, output),
        Commands::Report { input, output } => commands::report::run(ctx, input, output),
    }
}
