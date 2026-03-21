use anyhow::Result;

use crate::app::AppContext;

pub fn run(_ctx: &AppContext, frame: &str) -> Result<()> {
    println!("inspect {frame:?}: not yet implemented");
    Ok(())
}
