use std::io::{self, Write};

use crate::logo;

pub const DEFAULT_LOGO_FONT: &str = "DOS Rebel";
pub const DEFAULT_LOGO_TEXT: &str = "FIGEX CLI";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppConfig<'a> {
    pub logo_font: &'a str,
    pub logo_text: &'a str,
}

pub const DEFAULT_APP_CONFIG: AppConfig<'static> = AppConfig {
    logo_font: DEFAULT_LOGO_FONT,
    logo_text: DEFAULT_LOGO_TEXT,
};

pub fn run(stdout: &mut dyn Write, stderr: &mut dyn Write) -> io::Result<()> {
    run_with_config(DEFAULT_APP_CONFIG, stdout, stderr)
}

pub fn run_with_config(
    config: AppConfig<'_>,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> io::Result<()> {
    match logo::render_logo(config.logo_font, config.logo_text) {
        Ok(rendered_logo) => logo::write_rendered_logo(stdout, &rendered_logo)?,
        Err(error) => writeln!(stderr, "Error printing logo: {error}")?,
    }

    writeln!(stdout, "{}", figex_cli_core::greet())
}
