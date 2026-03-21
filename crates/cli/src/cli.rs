use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "figex", about = "Figma extraction CLI", version)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Path to config file [default: ~/.config/figex/config.json]
    #[arg(short, long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Connect to Figma Desktop runtime and establish a session
    Attach,

    /// Diagnose the connection environment (Figma running, patch applied, etc.)
    Doctor,

    /// Preview frame overview (node count, complexity, etc.)
    Inspect {
        /// Frame ID or name to inspect
        frame: String,
    },

    /// Extract raw frame info and save as raw.json
    Extract {
        /// Frame ID or name to extract
        frame: String,

        /// Output file path [default: raw.json]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },

    /// Generate features.json from extracted raw info
    Features {
        /// Input raw.json file [default: raw.json]
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,

        /// Output file path [default: features.json]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },

    /// Normalize to UI IR and generate ui.ir.json
    Normalize {
        /// Input features.json file [default: features.json]
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,

        /// Output file path [default: ui.ir.json]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },

    /// Output a human-readable analysis report (report.md)
    Report {
        /// Input ui.ir.json file [default: ui.ir.json]
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,

        /// Output file path [default: report.md]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
}
