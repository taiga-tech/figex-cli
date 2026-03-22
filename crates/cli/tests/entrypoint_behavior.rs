#[path = "support/failing_writer.rs"]
mod failing_writer;

use clap::Parser;
use figex_cli::cli::Cli;
use figex_cli::entrypoint::{run_with_exit_code, run_with_status};
use figex_cli_core::{FigexError, PipelineError};
use std::process::ExitCode;

fn make_cli() -> Cli {
    Cli::try_parse_from(["figex"]).expect("CLI args should parse")
}

#[tokio::test]
async fn run_with_exit_code_returns_none_on_success() {
    let mut stderr = Vec::new();

    let exit_code = run_with_exit_code(make_cli(), |_| async { Ok(()) }, &mut stderr).await;

    assert_eq!(exit_code, None);
    assert!(stderr.is_empty());
}

#[tokio::test]
async fn run_with_status_maps_success_and_figex_error_to_exit_codes() {
    let mut success_stderr = Vec::new();
    let mut error_stderr = Vec::new();

    let success = run_with_status(make_cli(), |_| async { Ok(()) }, &mut success_stderr).await;
    let error = run_with_status(
        make_cli(),
        |_| async { Err(FigexError::NormalizeFailed(PipelineError::NormalizeFailed).into()) },
        &mut error_stderr,
    )
    .await;

    assert_eq!(success, ExitCode::SUCCESS);
    assert!(success_stderr.is_empty());

    let error_stderr = String::from_utf8(error_stderr).expect("stderr should be valid UTF-8");
    assert_eq!(error, ExitCode::from(30));
    assert!(error_stderr.contains("error: normalize error: normalize failed"));
}

#[tokio::test]
async fn run_with_exit_code_maps_figex_error_to_spec_exit_code() {
    let mut stderr = Vec::new();

    let exit_code = run_with_exit_code(
        make_cli(),
        |_| async { Err(FigexError::NormalizeFailed(PipelineError::NormalizeFailed).into()) },
        &mut stderr,
    )
    .await;

    let stderr = String::from_utf8(stderr).expect("stderr should be valid UTF-8");
    assert_eq!(exit_code, Some(30));
    assert!(stderr.contains("error: normalize error: normalize failed"));
}

#[tokio::test]
async fn run_with_exit_code_ignores_stderr_write_failures() {
    let mut stderr = failing_writer::FailingWriter;

    let exit_code = run_with_exit_code(
        make_cli(),
        |_| async { Err(anyhow::anyhow!("boom")) },
        &mut stderr,
    )
    .await;

    assert_eq!(exit_code, Some(1));
}

#[tokio::test]
async fn run_with_status_maps_non_figex_error_to_exit_code_one() {
    let mut stderr = Vec::new();

    let exit_code = run_with_status(
        make_cli(),
        |_| async { Err(anyhow::anyhow!("boom")) },
        &mut stderr,
    )
    .await;

    assert_eq!(exit_code, ExitCode::from(1));
}
