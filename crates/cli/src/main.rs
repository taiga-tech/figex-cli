use clap::Parser;
use figex_cli::cli::Cli;

fn main() {
    let cli = Cli::parse();
    figex_cli::entrypoint::run_and_exit(cli, figex_cli::app::run, &mut std::io::stderr(), |code| {
        std::process::exit(code)
    });
}
