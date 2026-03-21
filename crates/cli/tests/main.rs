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
