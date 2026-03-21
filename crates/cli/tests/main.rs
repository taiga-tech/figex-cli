#[path = "support/run_figex_cli.rs"]
mod run_figex_cli;

#[test]
fn figex_cli_runs_and_exits_successfully() {
    // Given
    // When
    let output = run_figex_cli::run_figex_cli();

    // Then
    assert!(output.status.success());
    assert!(!output.stdout.is_empty(), "stdout should not be empty");
}

#[test]
fn figex_cli_does_not_write_to_stderr_on_success() {
    // Given

    // When
    let output = run_figex_cli::run_figex_cli();

    // Then
    assert!(output.status.success());
    assert!(
        output.stderr.is_empty(),
        "stderr should be empty on success, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn figex_cli_reports_error_when_stdout_pipe_is_closed() {
    let output = run_figex_cli::run_figex_cli_with_closed_stdout();

    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stdout.is_empty(),
        "stdout should be empty when the pipe is closed"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("error:"),
        "stderr should contain the formatted error message"
    );
}
