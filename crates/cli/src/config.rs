//! Configuration resolution: CLI args > env vars > figex.toml > defaults.
//!
//! Priority (highest to lowest):
//!   1. CLI arguments
//!   2. Environment variables (`FIGEX_*`)
//!   3. `figex.toml`
//!   4. Built-in defaults

use std::path::{Path, PathBuf};

use serde::Deserialize;

// ---------------------------------------------------------------------------
// figex.toml schema
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
pub struct FileConfig {
    pub runtime: Option<RuntimeFileConfig>,
    pub output: Option<OutputFileConfig>,
    pub classifier: Option<ClassifierFileConfig>,
    pub normalizer: Option<NormalizerFileConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RuntimeFileConfig {
    pub transport: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub timeout_ms: Option<u64>,
    pub snapshot_max_depth: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
pub struct OutputFileConfig {
    pub pretty: Option<bool>,
    pub include_diagnostics: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ClassifierFileConfig {
    pub enabled: Option<bool>,
    pub max_component_candidates: Option<u32>,
    pub max_atomic_candidates: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
pub struct NormalizerFileConfig {
    pub collapse_wrappers: Option<bool>,
    pub boundary_threshold: Option<f64>,
    pub preserve_repetition_candidates: Option<bool>,
}

// ---------------------------------------------------------------------------
// CLI overrides (populated from clap args)
// ---------------------------------------------------------------------------

/// Subset of CLI flags that participate in settings resolution.
#[derive(Debug, Default)]
pub struct CliOverrides {
    pub json: bool,
    pub pretty: bool,
    pub timeout_ms: Option<u64>,
    pub transport: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub log_level: Option<String>,
}

// ---------------------------------------------------------------------------
// Resolved settings
// ---------------------------------------------------------------------------

/// Final, fully-resolved settings used at runtime.
#[derive(Debug, Clone)]
pub struct Settings {
    // Global output
    pub json: bool,
    pub pretty: bool,
    pub log_level: String,

    // Runtime connection
    pub transport: String,
    pub host: String,
    pub port: u16,
    pub timeout_ms: u64,
    pub snapshot_max_depth: u32,

    // Output
    pub include_diagnostics: bool,

    // Normalizer
    pub collapse_wrappers: bool,
    pub boundary_threshold: f64,
    pub preserve_repetition_candidates: bool,

    // Classifier
    pub classifier_enabled: bool,
    pub max_component_candidates: u32,
    pub max_atomic_candidates: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            json: false,
            pretty: true,
            log_level: "warn".to_string(),
            transport: "auto".to_string(),
            host: "127.0.0.1".to_string(),
            port: 9222,
            timeout_ms: 5000,
            snapshot_max_depth: 128,
            include_diagnostics: true,
            collapse_wrappers: true,
            boundary_threshold: 0.62,
            preserve_repetition_candidates: true,
            classifier_enabled: true,
            max_component_candidates: 3,
            max_atomic_candidates: 3,
        }
    }
}

// ---------------------------------------------------------------------------
// Config file lookup
// ---------------------------------------------------------------------------

/// Walk from `start` toward the filesystem root, returning the first
/// `figex.toml` found. Returns `None` if none exists.
pub fn find_config_file(start: &Path) -> Option<PathBuf> {
    let mut dir = start;
    loop {
        let candidate = dir.join("figex.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return None,
        }
    }
}

/// Load and parse a `figex.toml`. Returns `FileConfig::default()` on any
/// read/parse error (missing file is not treated as fatal here).
pub fn load_file_config(path: &Path) -> FileConfig {
    match std::fs::read_to_string(path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("warning: failed to parse {}: {e}", path.display());
                FileConfig::default()
            }
        },
        Err(_) => FileConfig::default(),
    }
}

// ---------------------------------------------------------------------------
// Resolution
// ---------------------------------------------------------------------------

fn parse_bool_str(s: &str) -> bool {
    matches!(s, "1" | "true")
}

fn parse_bool_env(v: Option<String>) -> bool {
    v.is_some_and(|s| parse_bool_str(&s))
}

/// Resolve settings from all sources using the defined priority order.
///
/// `env_fn` is `Fn(&str) -> Option<String>` so that tests can inject a fake
/// environment without touching real env vars.
pub fn resolve_settings<F>(cli: &CliOverrides, env_fn: F, file: Option<&FileConfig>) -> Settings
where
    F: Fn(&str) -> Option<String>,
{
    let defaults = Settings::default();
    let rt = file.and_then(|f| f.runtime.as_ref());
    let out = file.and_then(|f| f.output.as_ref());
    let clf = file.and_then(|f| f.classifier.as_ref());
    let nrm = file.and_then(|f| f.normalizer.as_ref());

    Settings {
        // json: CLI > env > default
        json: cli.json || parse_bool_env(env_fn("FIGEX_JSON")),

        // pretty: CLI > env > file > default
        pretty: {
            let env_pretty = env_fn("FIGEX_PRETTY");
            if cli.pretty {
                true
            } else if let Some(v) = env_pretty {
                parse_bool_str(&v)
            } else {
                out.and_then(|o| o.pretty).unwrap_or(defaults.pretty)
            }
        },

        // log_level: CLI > env > default
        log_level: cli
            .log_level
            .clone()
            .or_else(|| env_fn("FIGEX_LOG_LEVEL"))
            .unwrap_or(defaults.log_level),

        // transport: CLI > env > file > default
        transport: cli
            .transport
            .clone()
            .or_else(|| env_fn("FIGEX_TRANSPORT"))
            .or_else(|| rt.and_then(|r| r.transport.clone()))
            .unwrap_or(defaults.transport),

        // host: CLI > env > file > default
        host: cli
            .host
            .clone()
            .or_else(|| env_fn("FIGEX_HOST"))
            .or_else(|| rt.and_then(|r| r.host.clone()))
            .unwrap_or(defaults.host),

        // port: CLI > env > file > default
        port: cli
            .port
            .or_else(|| env_fn("FIGEX_PORT").and_then(|v| v.parse::<u16>().ok()))
            .or_else(|| rt.and_then(|r| r.port))
            .unwrap_or(defaults.port),

        // timeout_ms: CLI > env > file > default
        timeout_ms: cli
            .timeout_ms
            .or_else(|| env_fn("FIGEX_TIMEOUT_MS").and_then(|v| v.parse::<u64>().ok()))
            .or_else(|| rt.and_then(|r| r.timeout_ms))
            .unwrap_or(defaults.timeout_ms),

        // snapshot_max_depth: file > default
        snapshot_max_depth: rt
            .and_then(|r| r.snapshot_max_depth)
            .unwrap_or(defaults.snapshot_max_depth),

        // include_diagnostics: file > default
        include_diagnostics: out
            .and_then(|o| o.include_diagnostics)
            .unwrap_or(defaults.include_diagnostics),

        // collapse_wrappers: file > default
        collapse_wrappers: nrm
            .and_then(|n| n.collapse_wrappers)
            .unwrap_or(defaults.collapse_wrappers),

        // boundary_threshold: file > default
        boundary_threshold: nrm
            .and_then(|n| n.boundary_threshold)
            .unwrap_or(defaults.boundary_threshold),

        // preserve_repetition_candidates: file > default
        preserve_repetition_candidates: nrm
            .and_then(|n| n.preserve_repetition_candidates)
            .unwrap_or(defaults.preserve_repetition_candidates),

        // classifier_enabled: file > default
        classifier_enabled: clf
            .and_then(|c| c.enabled)
            .unwrap_or(defaults.classifier_enabled),

        // max_component_candidates: file > default
        max_component_candidates: clf
            .and_then(|c| c.max_component_candidates)
            .unwrap_or(defaults.max_component_candidates),

        // max_atomic_candidates: file > default
        max_atomic_candidates: clf
            .and_then(|c| c.max_atomic_candidates)
            .unwrap_or(defaults.max_atomic_candidates),
    }
}
