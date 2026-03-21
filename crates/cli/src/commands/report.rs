use std::path::PathBuf;

use anyhow::Result;

use crate::app::AppContext;

pub fn run(_ctx: &AppContext, input: Option<PathBuf>, output: Option<PathBuf>) -> Result<()> {
    let input = input.unwrap_or_else(|| PathBuf::from("ui.ir.json"));
    let output = output.unwrap_or_else(|| PathBuf::from("report.md"));
    println!("report {input:?} -> {output:?}: not yet implemented");
    Ok(())
}
