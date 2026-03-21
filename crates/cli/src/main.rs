use std::io;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    figex_cli::app::run(&mut stdout, &mut stderr)
}
