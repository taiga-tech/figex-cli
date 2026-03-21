#[path = "support/run_figex_cli.rs"]
mod run_figex_cli;

#[test]
fn figex_cli_prints_greeting_and_exits_successfully() {
    // Given
    // When
    let output = run_figex_cli::run_figex_cli();

    // Then
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .expect("stdout should be valid UTF-8")
            .contains("Hello from figex-cli-core!"),
        "stdout should contain greeting message"
    );
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
