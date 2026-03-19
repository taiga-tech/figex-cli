use std::process::{Command, Output};

fn run_sample_cli() -> Output {
    let current_exe = std::env::current_exe().expect("current_exe should be available");
    let target_dir = current_exe
        .parent()
        .and_then(|deps| deps.parent())
        .expect("test binary should be under target/<profile>/deps");
    let binary_name = if cfg!(windows) {
        "sample-cli.exe"
    } else {
        "sample-cli"
    };
    let binary_path = target_dir.join(binary_name);

    Command::new(&binary_path)
        .output()
        .expect("sample-cli process should run")
}

#[test]
fn sample_cli_prints_greeting_and_exits_successfully() {
    // Given
    // When
    let output = run_sample_cli();

    // Then
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout should be valid UTF-8"),
        "Hello from sample-cli-core!\n"
    );
}

#[test]
fn sample_cli_does_not_write_to_stderr_on_success() {
    // Given

    // When
    let output = run_sample_cli();

    // Then
    assert!(output.status.success());
    assert!(
        output.stderr.is_empty(),
        "stderr should be empty on success, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
