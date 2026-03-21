use std::process::{Command, Output};

pub fn run_figex_cli() -> Output {
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
    let binary_path = target_dir.join(binary_name);

    Command::new(&binary_path)
        .output()
        .expect("figex-cli process should run")
}
