use anyhow::Result;
use clap::Parser;

use figex_cli::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    figex_cli::app::run(cli)
}
