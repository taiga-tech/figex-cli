use clap::Parser;
use figex_cli::cli::Cli;
use figex_cli::entrypoint::{run_and_exit, run_with_exit_code, run_with_status};
use figex_cli_core::{FigexError, PipelineError};
use std::process::ExitCode;

fn make_cli() -> Cli {
    Cli::try_parse_from(["figex"]).expect("CLI args should parse")
}

fn ok_runner(_: Cli) -> anyhow::Result<()> {
    Ok(())
}

fn boom_runner(_: Cli) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("boom"))
}

fn normalize_failed_runner(_: Cli) -> anyhow::Result<()> {
    Err(FigexError::NormalizeFailed(PipelineError::NormalizeFailed).into())
}

fn noop_exit(_: i32) {}

#[test]
fn run_with_exit_code_returns_none_on_success() {
    let mut stderr = Vec::new();

    let exit_code = run_with_exit_code(make_cli(), |_| Ok(()), &mut stderr);

    assert_eq!(exit_code, None);
    assert!(stderr.is_empty());
}

#[test]
fn run_and_exit_does_not_exit_on_success() {
    let mut stderr = Vec::new();
    let mut exited_with = None;

    run_and_exit(
        make_cli(),
        |_| Ok(()),
        &mut stderr,
        |code| exited_with = Some(code),
    );

    assert_eq!(exited_with, None);
    assert!(stderr.is_empty());
}

#[test]
fn run_and_exit_covers_both_paths_for_fn_pointer_instantiation() {
    let mut success_stderr = Vec::new();
    let mut error_stderr = Vec::new();

    run_and_exit(
        make_cli(),
        ok_runner as fn(Cli) -> anyhow::Result<()>,
        &mut success_stderr,
        noop_exit as fn(i32),
    );
    run_and_exit(
        make_cli(),
        boom_runner as fn(Cli) -> anyhow::Result<()>,
        &mut error_stderr,
        noop_exit as fn(i32),
    );

    assert!(success_stderr.is_empty());
    let error_stderr = String::from_utf8(error_stderr).expect("stderr should be valid UTF-8");
    assert!(error_stderr.contains("error: boom"));
}

#[test]
fn run_with_status_maps_success_and_figex_error_to_exit_codes() {
    let mut success_stderr = Vec::new();
    let mut error_stderr = Vec::new();

    let success = run_with_status(
        make_cli(),
        ok_runner as fn(Cli) -> anyhow::Result<()>,
        &mut success_stderr,
    );
    let error = run_with_status(
        make_cli(),
        normalize_failed_runner as fn(Cli) -> anyhow::Result<()>,
        &mut error_stderr,
    );

    assert_eq!(success, ExitCode::SUCCESS);
    assert!(success_stderr.is_empty());

    let error_stderr = String::from_utf8(error_stderr).expect("stderr should be valid UTF-8");
    assert_eq!(error, ExitCode::from(30));
    assert!(error_stderr.contains("error: normalize error: normalize failed"));
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
