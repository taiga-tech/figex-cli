//! Tests for configuration priority: CLI > env > figex.toml > defaults.

use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use figex_cli::config::{
    find_config_file, load_file_config, resolve_settings, ClassifierFileConfig, CliOverrides,
    FileConfig, NormalizerFileConfig, OutputFileConfig, RuntimeFileConfig,
};

fn no_env(_: &str) -> Option<String> {
    None
}

fn env_map<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
    move |key: &str| {
        pairs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.to_string())
    }
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&path).expect("temp dir should be created");
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

// ---------------------------------------------------------------------------
// Defaults
// ---------------------------------------------------------------------------

#[test]
fn defaults_are_applied_when_nothing_is_set() {
    let s = resolve_settings(&CliOverrides::default(), no_env, None);
    assert_eq!(s.transport, "auto");
    assert_eq!(s.host, "127.0.0.1");
    assert_eq!(s.port, 9222);
    assert_eq!(s.timeout_ms, 5000);
    assert_eq!(s.snapshot_max_depth, 128);
    assert!(s.pretty);
    assert!(!s.json);
    assert_eq!(s.log_level, "warn");
    assert!(s.classifier_enabled);
    assert_eq!(s.max_component_candidates, 3);
    assert_eq!(s.max_atomic_candidates, 3);
    assert!(s.collapse_wrappers);
    assert!((s.boundary_threshold - 0.62).abs() < f64::EPSILON);
    assert!(s.preserve_repetition_candidates);
    assert!(s.include_diagnostics);
}

// ---------------------------------------------------------------------------
// File config overrides defaults
// ---------------------------------------------------------------------------

#[test]
fn file_config_overrides_defaults() {
    let file = FileConfig {
        runtime: Some(RuntimeFileConfig {
            transport: Some("cdp".to_string()),
            host: Some("10.0.0.1".to_string()),
            port: Some(1234),
            timeout_ms: Some(8000),
            snapshot_max_depth: Some(64),
        }),
        output: Some(OutputFileConfig {
            pretty: Some(false),
            include_diagnostics: Some(false),
        }),
        classifier: Some(ClassifierFileConfig {
            enabled: Some(false),
            max_component_candidates: Some(5),
            max_atomic_candidates: Some(5),
        }),
        normalizer: Some(NormalizerFileConfig {
            collapse_wrappers: Some(false),
            boundary_threshold: Some(0.75),
            preserve_repetition_candidates: Some(false),
        }),
    };

    let s = resolve_settings(&CliOverrides::default(), no_env, Some(&file));

    assert_eq!(s.transport, "cdp");
    assert_eq!(s.host, "10.0.0.1");
    assert_eq!(s.port, 1234);
    assert_eq!(s.timeout_ms, 8000);
    assert_eq!(s.snapshot_max_depth, 64);
    assert!(!s.pretty);
    assert!(!s.include_diagnostics);
    assert!(!s.classifier_enabled);
    assert_eq!(s.max_component_candidates, 5);
    assert_eq!(s.max_atomic_candidates, 5);
    assert!(!s.collapse_wrappers);
    assert!((s.boundary_threshold - 0.75).abs() < f64::EPSILON);
    assert!(!s.preserve_repetition_candidates);
}

// ---------------------------------------------------------------------------
// Env vars override file config
// ---------------------------------------------------------------------------

#[test]
fn env_vars_override_file_config() {
    let file = FileConfig {
        runtime: Some(RuntimeFileConfig {
            transport: Some("cdp".to_string()),
            host: Some("file-host".to_string()),
            port: Some(1111),
            timeout_ms: Some(1000),
            snapshot_max_depth: None,
        }),
        ..FileConfig::default()
    };

    let env = env_map(&[
        ("FIGEX_TRANSPORT", "mcp"),
        ("FIGEX_HOST", "env-host"),
        ("FIGEX_PORT", "5555"),
        ("FIGEX_TIMEOUT_MS", "9999"),
        ("FIGEX_LOG_LEVEL", "info"),
    ]);

    let s = resolve_settings(&CliOverrides::default(), env, Some(&file));

    assert_eq!(s.transport, "mcp");
    assert_eq!(s.host, "env-host");
    assert_eq!(s.port, 5555);
    assert_eq!(s.timeout_ms, 9999);
    assert_eq!(s.log_level, "info");
}

// ---------------------------------------------------------------------------
// CLI overrides override env vars and file config
// ---------------------------------------------------------------------------

#[test]
fn cli_overrides_take_highest_priority() {
    let file = FileConfig {
        runtime: Some(RuntimeFileConfig {
            transport: Some("cdp".to_string()),
            host: Some("file-host".to_string()),
            port: Some(1111),
            timeout_ms: Some(1000),
            snapshot_max_depth: None,
        }),
        ..FileConfig::default()
    };

    let env = env_map(&[
        ("FIGEX_TRANSPORT", "mcp"),
        ("FIGEX_HOST", "env-host"),
        ("FIGEX_PORT", "5555"),
        ("FIGEX_TIMEOUT_MS", "9999"),
        ("FIGEX_LOG_LEVEL", "info"),
    ]);

    let cli = CliOverrides {
        transport: Some("auto".to_string()),
        host: Some("cli-host".to_string()),
        port: Some(7777),
        timeout_ms: Some(42),
        log_level: Some("trace".to_string()),
        json: true,
        pretty: true,
    };

    let s = resolve_settings(&cli, env, Some(&file));

    assert_eq!(s.transport, "auto");
    assert_eq!(s.host, "cli-host");
    assert_eq!(s.port, 7777);
    assert_eq!(s.timeout_ms, 42);
    assert_eq!(s.log_level, "trace");
    assert!(s.json);
    assert!(s.pretty);
}

// ---------------------------------------------------------------------------
// Partial env vars — only some keys set
// ---------------------------------------------------------------------------

#[test]
fn partial_env_vars_fall_through_to_file_or_default() {
    let file = FileConfig {
        runtime: Some(RuntimeFileConfig {
            host: Some("file-host".to_string()),
            ..RuntimeFileConfig::default()
        }),
        ..FileConfig::default()
    };

    // Only FIGEX_PORT is set in env
    let env = env_map(&[("FIGEX_PORT", "8888")]);

    let s = resolve_settings(&CliOverrides::default(), env, Some(&file));

    // host comes from file (env didn't set it)
    assert_eq!(s.host, "file-host");
    // port comes from env
    assert_eq!(s.port, 8888);
    // transport falls through to default
    assert_eq!(s.transport, "auto");
}

// ---------------------------------------------------------------------------
// Invalid env var values are ignored (fall through to lower priority)
// ---------------------------------------------------------------------------

#[test]
fn invalid_port_env_var_falls_through_to_file() {
    let file = FileConfig {
        runtime: Some(RuntimeFileConfig {
            port: Some(3000),
            ..RuntimeFileConfig::default()
        }),
        ..FileConfig::default()
    };

    let env = env_map(&[("FIGEX_PORT", "not-a-number")]);

    let s = resolve_settings(&CliOverrides::default(), env, Some(&file));

    // env parse failed → use file value
    assert_eq!(s.port, 3000);
}

// ---------------------------------------------------------------------------
// Config file lookup / loading
// ---------------------------------------------------------------------------

#[test]
fn find_config_file_discovers_nearest_parent_config() {
    let temp_dir = TempDir::new("figex-config-discovery");
    let workspace_root = temp_dir.path.join("workspace");
    let nested_dir = workspace_root.join("apps/web");
    fs::create_dir_all(&nested_dir).expect("nested dir should be created");

    let config_path = workspace_root.join("figex.toml");
    fs::write(&config_path, "transport = 'auto'\n").expect("config file should be written");

    let discovered = find_config_file(&nested_dir);

    assert_eq!(discovered, Some(config_path));
}

#[test]
fn load_file_config_reads_valid_toml() {
    let temp_dir = TempDir::new("figex-config-load");
    let config_path = temp_dir.path.join("figex.toml");
    fs::write(
        &config_path,
        r#"
[runtime]
transport = "mcp"
port = 9911

[output]
pretty = false
"#,
    )
    .expect("config file should be written");

    let config = load_file_config(&config_path);

    assert_eq!(
        config.runtime.and_then(|runtime| runtime.transport),
        Some("mcp".to_string())
    );
    assert_eq!(config.output.and_then(|output| output.pretty), Some(false));
}

#[test]
fn load_file_config_returns_default_for_missing_file() {
    let temp_dir = TempDir::new("figex-config-missing");
    let missing_path = temp_dir.path.join("missing.toml");

    let config = load_file_config(&missing_path);

    assert!(config.runtime.is_none());
    assert!(config.output.is_none());
    assert!(config.classifier.is_none());
    assert!(config.normalizer.is_none());
}
