use std::process::{Command, Output, Stdio};

pub fn run_figex_cli() -> Output {
    let binary_path = figex_cli_binary_path();

    Command::new(&binary_path)
        .output()
        .expect("figex-cli process should run")
}

pub fn run_figex_cli_with_closed_stdout() -> Output {
    let binary_path = figex_cli_binary_path();

    let mut child = Command::new(&binary_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("figex-cli process should spawn");

    drop(child.stdout.take());

    child
        .wait_with_output()
        .expect("figex-cli process should finish")
}

fn figex_cli_binary_path() -> std::path::PathBuf {
    let current_exe = std::env::current_exe().expect("current_exe should be available");
    let target_dir = current_exe
        .parent()
        .and_then(|deps| deps.parent())
        .expect("test binary should be under target/<profile>/deps");
    let binary_name = if cfg!(windows) {
        "figex-cli.exe"
    } else {
        "figex-cli"
    };
    target_dir.join(binary_name)
}
