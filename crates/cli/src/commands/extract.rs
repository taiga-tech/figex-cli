use std::path::PathBuf;

use anyhow::Result;

use crate::app::AppContext;

pub fn run(_ctx: &AppContext, frame: &str, output: Option<PathBuf>) -> Result<()> {
    let output = output.unwrap_or_else(|| PathBuf::from("raw.json"));
    println!("extract {frame:?} -> {output:?}: not yet implemented");
    Ok(())
}
