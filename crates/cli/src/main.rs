use std::process::ExitCode;

use clap::Parser;
use figex_cli::cli::Cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    figex_cli::entrypoint::run_with_status(
        Cli::parse(),
        figex_cli::app::run,
        &mut std::io::stderr(),
    )
    .await
}
