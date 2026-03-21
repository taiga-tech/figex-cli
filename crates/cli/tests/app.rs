#[path = "support/failing_writer.rs"]
mod failing_writer;

use figex_cli::app::{run, run_with_config, AppConfig};
use std::io::{self, Write};

#[test]
fn run_writes_logo_and_greeting_with_default_config() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    run(&mut stdout, &mut stderr).expect("default app run should succeed");

    let stdout = String::from_utf8(stdout).expect("stdout should be valid UTF-8");

    assert!(stdout.contains("Hello from figex-cli-core!"));
    assert!(!stdout.trim().is_empty());
    assert!(stderr.is_empty());
}

#[test]
fn run_with_config_reports_logo_errors_and_continues() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    run_with_config(
        AppConfig {
            logo_font: "missing-font",
            logo_text: "FIGEX CLI",
        },
        &mut stdout,
        &mut stderr,
    )
    .expect("missing font should not stop greeting output");

    assert_eq!(
        String::from_utf8(stdout).expect("stdout should be valid UTF-8"),
        "Hello from figex-cli-core!\n"
    );
    assert_eq!(
        String::from_utf8(stderr).expect("stderr should be valid UTF-8"),
        "Error printing logo: Failed to load font: missing-font\n"
    );
}

#[test]
fn run_with_config_propagates_stderr_write_failures() {
    let mut stdout = Vec::new();
    let mut stderr = failing_writer::FailingWriter;

    let error = run_with_config(
        AppConfig {
            logo_font: "missing-font",
            logo_text: "FIGEX CLI",
        },
        &mut stdout,
        &mut stderr,
    )
    .expect_err("stderr write failures should bubble up");

    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert!(stdout.is_empty());
    stderr
        .flush()
        .expect("failing writer flush should still succeed");
}

#[test]
fn run_with_config_propagates_stdout_write_failures() {
    let mut stdout = failing_writer::FailingWriter;
    let mut stderr = Vec::new();

    let error = run_with_config(
        AppConfig {
            logo_font: "DOS Rebel",
            logo_text: "FIGEX CLI",
        },
        &mut stdout,
        &mut stderr,
    )
    .expect_err("stdout write failures should bubble up");

    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert!(stderr.is_empty());
}
