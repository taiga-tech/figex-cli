#[path = "support/failing_writer.rs"]
mod failing_writer;

use std::path::PathBuf;

use clap::Parser;
use figex_cli::app::{config_dir_with_base, run_io, run_io_with_logo, AppContext};
use figex_cli::cli::Cli;
use std::io;

fn make_cli(args: &[&str]) -> Cli {
    Cli::try_parse_from(std::iter::once("figex-cli").chain(args.iter().copied()))
        .expect("args should parse successfully")
}

// ---------------------------------------------------------------------------
// No subcommand
// ---------------------------------------------------------------------------

#[test]
fn no_subcommand_writes_logo_and_help_hint() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    run_io(make_cli(&[]), &mut stdout, &mut stderr).expect("run should succeed");

    let stdout = String::from_utf8(stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("--help"));
    assert!(!stdout.trim().is_empty());
    assert!(stderr.is_empty());
}

// ---------------------------------------------------------------------------
// Logo error paths
// ---------------------------------------------------------------------------

#[test]
fn logo_error_reports_to_stderr_and_continues() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    run_io_with_logo(
        make_cli(&[]),
        "missing-font",
        "FIGEX CLI",
        &mut stdout,
        &mut stderr,
    )
    .expect("logo error should not stop execution");

    assert!(
        String::from_utf8(stderr)
            .unwrap()
            .contains("Error printing logo:"),
        "stderr should contain logo error"
    );
}

#[test]
fn logo_error_propagates_stderr_write_failure() {
    let mut stdout = Vec::new();
    let mut stderr = failing_writer::FailingWriter;

    let error = run_io_with_logo(
        make_cli(&[]),
        "missing-font",
        "FIGEX CLI",
        &mut stdout,
        &mut stderr,
    )
    .expect_err("stderr write failure should bubble up");

    assert!(error
        .downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::Other)
        .unwrap_or(false));
}

// ---------------------------------------------------------------------------
// stdout write failures
// ---------------------------------------------------------------------------

#[test]
fn logo_error_then_stdout_write_failure_propagates() {
    // Logo fails → stderr write OK → writeln!(stdout, "help hint")? fails
    // Covers the ? error branch on the None arm of the cli.command match
    let mut stdout = failing_writer::FailingWriter;
    let mut stderr = Vec::new();

    let error = run_io_with_logo(
        make_cli(&[]),
        "missing-font",
        "FIGEX CLI",
        &mut stdout,
        &mut stderr,
    )
    .expect_err("stdout write failure should bubble up");

    assert!(error
        .downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::Other)
        .unwrap_or(false));
}

#[test]
fn run_io_propagates_stdout_write_failures() {
    let mut stdout = failing_writer::FailingWriter;
    let mut stderr = Vec::new();

    let error = run_io(make_cli(&[]), &mut stdout, &mut stderr)
        .expect_err("write failure should bubble up");

    assert!(error
        .downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::Other)
        .unwrap_or(false));
}

// ---------------------------------------------------------------------------
// Subcommands — dispatch coverage
// ---------------------------------------------------------------------------

#[test]
fn subcommand_attach_runs_without_error() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["attach"]), &mut stdout, &mut stderr).expect("attach should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_doctor_runs_without_error() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["doctor"]), &mut stdout, &mut stderr).expect("doctor should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_inspect_runs_without_error() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["inspect", "my-frame"]), &mut stdout, &mut stderr)
        .expect("inspect should succeed");
    assert!(stderr.is_empty());
}

// extract: output None / Some
#[test]
fn subcommand_extract_default_output() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["extract", "my-frame"]), &mut stdout, &mut stderr)
        .expect("extract should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_extract_custom_output() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["extract", "my-frame", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("extract with output should succeed");
    assert!(stderr.is_empty());
}

// features: input None / Some, output None / Some
#[test]
fn subcommand_features_default_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["features"]), &mut stdout, &mut stderr)
        .expect("features default should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_features_custom_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["features", "--input", "in.json", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("features custom should succeed");
    assert!(stderr.is_empty());
}

// normalize: input None / Some, output None / Some
#[test]
fn subcommand_normalize_default_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["normalize"]), &mut stdout, &mut stderr)
        .expect("normalize default should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_normalize_custom_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["normalize", "--input", "in.json", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("normalize custom should succeed");
    assert!(stderr.is_empty());
}

// ---------------------------------------------------------------------------
// AppContext and config path helpers
// ---------------------------------------------------------------------------

#[test]
fn config_dir_with_base_uses_provided_path() {
    let result = config_dir_with_base(Some(PathBuf::from("/custom")));
    assert_eq!(result, PathBuf::from("/custom/figex/config.json"));
}

#[test]
fn config_dir_with_base_falls_back_to_dot_when_none() {
    let result = config_dir_with_base(None);
    assert_eq!(result, PathBuf::from("./figex/config.json"));
}

#[test]
fn app_context_uses_explicit_config_path() {
    let ctx = AppContext::new(true, Some(PathBuf::from("/custom/config.json")));
    assert_eq!(ctx.config_path, PathBuf::from("/custom/config.json"));
    assert!(ctx.verbose);
}

#[test]
fn app_context_defaults_to_config_dir_when_none() {
    let ctx = AppContext::new(false, None);
    assert!(ctx.config_path.ends_with("figex/config.json"));
    assert!(!ctx.verbose);
}

// ---------------------------------------------------------------------------
// report: input None / Some, output None / Some
// ---------------------------------------------------------------------------

#[test]
fn subcommand_report_default_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["report"]), &mut stdout, &mut stderr).expect("report default should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_report_custom_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["report", "--input", "in.json", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("report custom should succeed");
    assert!(stderr.is_empty());
}
