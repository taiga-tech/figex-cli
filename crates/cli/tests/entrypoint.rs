use clap::Parser;
use figex_cli::cli::Cli;
use figex_cli::entrypoint::{run_and_exit, run_with_exit_code};
use figex_cli_core::{FigexError, PipelineError};

fn make_cli() -> Cli {
    Cli::try_parse_from(["figex"]).expect("CLI args should parse")
}

#[test]
fn run_with_exit_code_returns_none_on_success() {
    let mut stderr = Vec::new();

    let exit_code = run_with_exit_code(make_cli(), |_| Ok(()), &mut stderr);

    assert_eq!(exit_code, None);
    assert!(stderr.is_empty());
}

#[test]
fn run_with_exit_code_maps_figex_error_to_spec_exit_code() {
    let mut stderr = Vec::new();

    let exit_code = run_with_exit_code(
        make_cli(),
        |_| Err(FigexError::NormalizeFailed(PipelineError::NormalizeFailed).into()),
        &mut stderr,
    );

    let stderr = String::from_utf8(stderr).expect("stderr should be valid UTF-8");
    assert_eq!(exit_code, Some(30));
    assert!(stderr.contains("error: normalize error: normalize failed"));
}

#[test]
fn run_and_exit_uses_default_exit_code_for_non_figex_errors() {
    let mut stderr = Vec::new();
    let mut exited_with = None;

    run_and_exit(
        make_cli(),
        |_| Err(anyhow::anyhow!("boom")),
        &mut stderr,
        |code| exited_with = Some(code),
    );

    let stderr = String::from_utf8(stderr).expect("stderr should be valid UTF-8");
    assert_eq!(exited_with, Some(1));
    assert!(stderr.contains("error: boom"));
}
