#[path = "support/failing_writer.rs"]
mod failing_writer;

use clap::Parser;
use figex_cli::app::{
    build_settings, log_level_to_str, run_io, run_io_with_logo, transport_to_str,
};
use figex_cli::cli::Cli;
use figex_cli::cli::{LogLevel, Transport};
use std::io;

fn make_cli(args: &[&str]) -> Cli {
    Cli::try_parse_from(std::iter::once("figex").chain(args.iter().copied()))
        .expect("args should parse successfully")
}

// ---------------------------------------------------------------------------
// No subcommand
// ---------------------------------------------------------------------------

#[test]
fn no_subcommand_writes_logo_and_help_hint() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    run_io(make_cli(&[]), &mut stdout, &mut stderr).expect("run should succeed");

    let stdout = String::from_utf8(stdout).expect("stdout should be valid UTF-8");
    assert!(stdout.contains("--help"));
    assert!(!stdout.trim().is_empty());
    assert!(stderr.is_empty());
}

// ---------------------------------------------------------------------------
// Logo error paths
// ---------------------------------------------------------------------------

#[test]
fn logo_error_reports_to_stderr_and_continues() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    run_io_with_logo(
        make_cli(&[]),
        "missing-font",
        "FIGEX CLI",
        &mut stdout,
        &mut stderr,
    )
    .expect("logo error should not stop execution");

    assert!(
        String::from_utf8(stderr)
            .unwrap()
            .contains("Error printing logo:"),
        "stderr should contain logo error"
    );
}

#[test]
fn logo_error_propagates_stderr_write_failure() {
    let mut stdout = Vec::new();
    let mut stderr = failing_writer::FailingWriter;

    let error = run_io_with_logo(
        make_cli(&[]),
        "missing-font",
        "FIGEX CLI",
        &mut stdout,
        &mut stderr,
    )
    .expect_err("stderr write failure should bubble up");

    assert!(error
        .downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::Other)
        .unwrap_or(false));
}

// ---------------------------------------------------------------------------
// stdout write failures
// ---------------------------------------------------------------------------

#[test]
fn logo_error_then_stdout_write_failure_propagates() {
    let mut stdout = failing_writer::FailingWriter;
    let mut stderr = Vec::new();

    let error = run_io_with_logo(
        make_cli(&[]),
        "missing-font",
        "FIGEX CLI",
        &mut stdout,
        &mut stderr,
    )
    .expect_err("stdout write failure should bubble up");

    assert!(error
        .downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::Other)
        .unwrap_or(false));
}

#[test]
fn run_io_propagates_stdout_write_failures() {
    let mut stdout = failing_writer::FailingWriter;
    let mut stderr = Vec::new();

    let error = run_io(make_cli(&[]), &mut stdout, &mut stderr)
        .expect_err("write failure should bubble up");

    assert!(error
        .downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::Other)
        .unwrap_or(false));
}

// ---------------------------------------------------------------------------
// Subcommands — dispatch coverage
// ---------------------------------------------------------------------------

#[test]
fn subcommand_attach_runs_without_error() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["attach"]), &mut stdout, &mut stderr).expect("attach should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_doctor_runs_without_error() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["doctor"]), &mut stdout, &mut stderr).expect("doctor should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_inspect_frame_runs_without_error() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["inspect", "frame", "my-frame"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("inspect frame should succeed");
    assert!(stderr.is_empty());
}

// extract: output None / Some
#[test]
fn subcommand_extract_frame_default_output() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["extract", "frame", "my-frame"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("extract frame should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_extract_frame_custom_output() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["extract", "frame", "my-frame", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("extract frame with output should succeed");
    assert!(stderr.is_empty());
}

// features: input None / Some, output None / Some
#[test]
fn subcommand_features_default_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["features"]), &mut stdout, &mut stderr)
        .expect("features default should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_features_custom_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["features", "--input", "in.json", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("features custom should succeed");
    assert!(stderr.is_empty());
}

// normalize: input None / Some, output None / Some
#[test]
fn subcommand_normalize_default_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["normalize"]), &mut stdout, &mut stderr)
        .expect("normalize default should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_normalize_custom_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["normalize", "--input", "in.json", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("normalize custom should succeed");
    assert!(stderr.is_empty());
}

// report: input None / Some, output None / Some
#[test]
fn subcommand_report_default_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(make_cli(&["report"]), &mut stdout, &mut stderr).expect("report default should succeed");
    assert!(stderr.is_empty());
}

#[test]
fn subcommand_report_custom_io() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    run_io(
        make_cli(&["report", "--input", "in.json", "--output", "out.json"]),
        &mut stdout,
        &mut stderr,
    )
    .expect("report custom should succeed");
    assert!(stderr.is_empty());
}

// ---------------------------------------------------------------------------
// Global options — parsing
// ---------------------------------------------------------------------------

#[test]
fn global_option_json_is_parsed() {
    let cli = make_cli(&["--json", "attach"]);
    assert!(cli.json);
}

#[test]
fn global_option_pretty_is_parsed() {
    let cli = make_cli(&["--pretty", "attach"]);
    assert!(cli.pretty);
}

#[test]
fn global_option_timeout_ms_is_parsed() {
    let cli = make_cli(&["--timeout-ms", "3000", "attach"]);
    assert_eq!(cli.timeout_ms, Some(3000));
}

#[test]
fn global_option_transport_cdp_is_parsed() {
    use figex_cli::cli::Transport;
    let cli = make_cli(&["--transport", "cdp", "attach"]);
    assert!(matches!(cli.transport, Some(Transport::Cdp)));
}

#[test]
fn global_option_transport_mcp_is_parsed() {
    use figex_cli::cli::Transport;
    let cli = make_cli(&["--transport", "mcp", "attach"]);
    assert!(matches!(cli.transport, Some(Transport::Mcp)));
}

#[test]
fn global_option_transport_auto_is_parsed() {
    use figex_cli::cli::Transport;
    let cli = make_cli(&["--transport", "auto", "attach"]);
    assert!(matches!(cli.transport, Some(Transport::Auto)));
}

#[test]
fn global_option_host_is_parsed() {
    let cli = make_cli(&["--host", "192.168.1.1", "attach"]);
    assert_eq!(cli.host.as_deref(), Some("192.168.1.1"));
}

#[test]
fn global_option_port_is_parsed() {
    let cli = make_cli(&["--port", "9999", "attach"]);
    assert_eq!(cli.port, Some(9999));
}

#[test]
fn global_option_log_level_debug_is_parsed() {
    use figex_cli::cli::LogLevel;
    let cli = make_cli(&["--log-level", "debug", "attach"]);
    assert!(matches!(cli.log_level, Some(LogLevel::Debug)));
}

#[test]
fn global_option_config_is_parsed() {
    let cli = make_cli(&["--config", "/tmp/figex.toml", "attach"]);
    assert_eq!(
        cli.config.as_deref(),
        Some(std::path::Path::new("/tmp/figex.toml"))
    );
}

#[test]
fn build_settings_maps_all_transport_variants() {
    let missing_config = std::env::temp_dir().join(format!(
        "figex-missing-config-{}-transport.toml",
        std::process::id()
    ));
    let missing_config = missing_config.to_string_lossy().into_owned();

    for (transport, expected) in [("cdp", "cdp"), ("mcp", "mcp"), ("auto", "auto")] {
        let cli = make_cli(&["--config", &missing_config, "--transport", transport]);
        let settings = build_settings(&cli);
        assert_eq!(settings.transport, expected);
    }
}

#[test]
fn build_settings_maps_all_log_levels() {
    let missing_config = std::env::temp_dir().join(format!(
        "figex-missing-config-{}-log-level.toml",
        std::process::id()
    ));
    let missing_config = missing_config.to_string_lossy().into_owned();

    for (level, expected) in [
        ("error", "error"),
        ("warn", "warn"),
        ("info", "info"),
        ("debug", "debug"),
        ("trace", "trace"),
    ] {
        let cli = make_cli(&["--config", &missing_config, "--log-level", level]);
        let settings = build_settings(&cli);
        assert_eq!(settings.log_level, expected);
    }
}

#[test]
fn transport_to_str_maps_all_variants() {
    for (transport, expected) in [
        (Transport::Cdp, "cdp"),
        (Transport::Mcp, "mcp"),
        (Transport::Auto, "auto"),
    ] {
        assert_eq!(transport_to_str(&transport), expected);
    }
}

#[test]
fn log_level_to_str_maps_all_variants() {
    for (level, expected) in [
        (LogLevel::Error, "error"),
        (LogLevel::Warn, "warn"),
        (LogLevel::Info, "info"),
        (LogLevel::Debug, "debug"),
        (LogLevel::Trace, "trace"),
    ] {
        assert_eq!(log_level_to_str(&level), expected);
    }
}
