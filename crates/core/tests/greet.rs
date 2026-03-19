#[test]
fn greet_returns_expected_message() {
    // Given
    // (no preconditions)

    // When
    let message = sample_cli_core::greet();

    // Then
    assert_eq!(message, "Hello from sample-cli-core!");
}
