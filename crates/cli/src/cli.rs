use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

// ---------------------------------------------------------------------------
// Value enums
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, ValueEnum)]
pub enum Transport {
    Cdp,
    Mcp,
    Auto,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// ---------------------------------------------------------------------------
// Root CLI
// ---------------------------------------------------------------------------

#[derive(Parser)]
#[command(name = "figex", about = "Figma extraction CLI", version)]
pub struct Cli {
    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Pretty-print output
    #[arg(long, global = true)]
    pub pretty: bool,

    /// Connection timeout in milliseconds
    #[arg(long, global = true, value_name = "MS")]
    pub timeout_ms: Option<u64>,

    /// Transport type [cdp|mcp|auto]
    #[arg(long, global = true, value_enum, value_name = "TYPE")]
    pub transport: Option<Transport>,

    /// Host to connect to
    #[arg(long, global = true, value_name = "HOST")]
    pub host: Option<String>,

    /// Port to connect to
    #[arg(long, global = true, value_name = "PORT")]
    pub port: Option<u16>,

    /// Path to config file [default: figex.toml]
    #[arg(long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Log level [error|warn|info|debug|trace]
    #[arg(long, global = true, value_enum, value_name = "LEVEL")]
    pub log_level: Option<LogLevel>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

// ---------------------------------------------------------------------------
// Subcommands
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
pub enum Commands {
    /// Connect to Figma Desktop runtime and verify the session
    Attach,

    /// Diagnose the connection environment (port discovery, ping, snapshot check)
    Doctor,

    /// Preview frame metadata (node count, text count, etc.)
    Inspect {
        #[command(subcommand)]
        subcommand: InspectSubcommand,
    },

    /// Extract raw snapshot from a frame and save to raw.json
    Extract {
        #[command(subcommand)]
        subcommand: ExtractSubcommand,
    },

    /// Build features.json from raw.json
    Features {
        /// Input file [default: raw.json]
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,

        /// Output file [default: features.json]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },

    /// Build ui.ir.json from features.json
    Normalize {
        /// Input file [default: features.json]
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,

        /// Output file [default: ui.ir.json]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },

    /// Build report.md from ui.ir.json
    Report {
        /// Input file [default: ui.ir.json]
        #[arg(short, long, value_name = "PATH")]
        input: Option<PathBuf>,

        /// Output file [default: report.md]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
}

// ---------------------------------------------------------------------------
// Nested subcommands
// ---------------------------------------------------------------------------

#[derive(Subcommand)]
pub enum InspectSubcommand {
    /// Show metadata for a frame (node count, text count, layout count, etc.)
    Frame {
        /// Frame reference: id, deep link, or selection alias
        frame_ref: String,
    },
}

#[derive(Subcommand)]
pub enum ExtractSubcommand {
    /// Extract raw snapshot for a frame
    Frame {
        /// Frame reference: id, deep link, or selection alias
        frame_ref: String,

        /// Output file [default: raw.json]
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,
    },
}
