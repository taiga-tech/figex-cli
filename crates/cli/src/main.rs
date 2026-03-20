mod logo;
mod utils;

use logo::print_logo;

fn main() {
    // ロゴを表示
    print_logo("DOS Rebel", "FIGEX CLI").unwrap_or_else(|e| {
        eprintln!("Error printing logo: {}", e);
    });

    let message = figex_cli_core::greet();
    println!("{}", message);
}
