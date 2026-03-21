use std::path::PathBuf;

use anyhow::Result;

use crate::app::AppContext;

pub fn run(_ctx: &AppContext, input: Option<PathBuf>, output: Option<PathBuf>) -> Result<()> {
    let input = input.unwrap_or_else(|| PathBuf::from("raw.json"));
    let output = output.unwrap_or_else(|| PathBuf::from("features.json"));
    println!("features {input:?} -> {output:?}: not yet implemented");
    Ok(())
}
