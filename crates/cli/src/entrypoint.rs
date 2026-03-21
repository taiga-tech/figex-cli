use std::io::Write;
use std::process::ExitCode;

use crate::cli::Cli;
use figex_cli_core::FigexError;

#[doc(hidden)]
#[inline(never)]
pub fn run_and_exit<W, R, E>(cli: Cli, runner: R, stderr: &mut W, exit: E)
where
    W: Write,
    R: FnOnce(Cli) -> anyhow::Result<()>,
    E: FnOnce(i32),
{
    if let Some(code) = run_with_exit_code(cli, runner, stderr) {
        exit(code);
    }
}

#[doc(hidden)]
#[inline(never)]
pub fn run_with_exit_code<W, R>(cli: Cli, runner: R, stderr: &mut W) -> Option<i32>
where
    W: Write,
    R: FnOnce(Cli) -> anyhow::Result<()>,
{
    match runner(cli) {
        Ok(()) => None,
        Err(error) => {
            let _ = writeln!(stderr, "error: {error:#}");
            Some(
                error
                    .downcast_ref::<FigexError>()
                    .map(|figex_error| figex_error.exit_code())
                    .unwrap_or(1),
            )
        }
    }
}

#[doc(hidden)]
#[inline(never)]
pub fn run_with_status<W, R>(cli: Cli, runner: R, stderr: &mut W) -> ExitCode
where
    W: Write,
    R: FnOnce(Cli) -> anyhow::Result<()>,
{
    run_with_exit_code(cli, runner, stderr)
        .map_or(ExitCode::SUCCESS, |code| ExitCode::from(code as u8))
}
