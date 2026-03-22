use std::future::Future;
use std::io::Write;
use std::process::ExitCode;

use crate::cli::Cli;
use figex_cli_core::FigexError;

#[doc(hidden)]
#[inline(never)]
pub async fn run_with_exit_code<W, R, F>(cli: Cli, runner: R, stderr: &mut W) -> Option<i32>
where
    W: Write,
    R: FnOnce(Cli) -> F,
    F: Future<Output = anyhow::Result<()>>,
{
    let result = runner(cli).await;
    if let Err(error) = result {
        let _ = writeln!(stderr, "error: {error:#}");
        return Some(
            error
                .downcast_ref::<FigexError>()
                .map_or(1, FigexError::exit_code),
        );
    }

    None
}

#[doc(hidden)]
#[inline(never)]
pub async fn run_with_status<W, R, F>(cli: Cli, runner: R, stderr: &mut W) -> ExitCode
where
    W: Write,
    R: FnOnce(Cli) -> F,
    F: Future<Output = anyhow::Result<()>>,
{
    let exit_code = run_with_exit_code(cli, runner, stderr).await;
    if let Some(code) = exit_code {
        return ExitCode::from(code as u8);
    }

    ExitCode::SUCCESS
}
