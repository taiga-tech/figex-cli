#[path = "support/failing_writer.rs"]
mod failing_writer;

use figex_cli::app::AppContext;
use figex_cli::commands::doctor::{
    fallback_health, format_health_json, render_health, write_health_human,
};
use figex_cli::config::Settings;
use figex_cli_core::RuntimeError;
use figex_cli_runtime::RuntimeHealth;

fn make_context(json: bool, pretty: bool) -> AppContext {
    AppContext::new(Settings {
        json,
        pretty,
        log_level: "info".to_string(),
        transport: "cdp".to_string(),
        host: "127.0.0.1".to_string(),
        port: 9222,
        timeout_ms: 2_000,
        snapshot_max_depth: 3,
        include_diagnostics: false,
        collapse_wrappers: true,
        boundary_threshold: 0.5,
        preserve_repetition_candidates: true,
        classifier_enabled: true,
        max_component_candidates: 20,
        max_atomic_candidates: 50,
    })
}

fn make_health() -> RuntimeHealth {
    RuntimeHealth {
        transport: "cdp".to_string(),
        host: "127.0.0.1".to_string(),
        port: 9222,
        target_id: None,
        target_title: Some("Figma".to_string()),
        target_url: Some("https://www.figma.com/file/abc".to_string()),
        target_score: Some(42),
        ping_ok: true,
        snapshot_ok: false,
        latency_ms: Some(17),
        ping_script_version: "ping-v1".to_string(),
        snapshot_script_version: "snapshot-v1".to_string(),
        warnings: vec!["snapshot check failed".to_string()],
        errors: vec!["Runtime.enable failed".to_string()],
    }
}

#[test]
fn fallback_health_uses_context_and_versions() {
    let ctx = make_context(false, false);

    let health = fallback_health(&ctx, RuntimeError::AttachFailed);
    let (ping_script_version, snapshot_script_version) = figex_cli_runtime::transport_versions();

    assert_eq!(health.transport, "cdp");
    assert_eq!(health.host, ctx.settings.host);
    assert_eq!(health.port, ctx.settings.port);
    assert_eq!(health.ping_script_version, ping_script_version);
    assert_eq!(health.snapshot_script_version, snapshot_script_version);
    assert_eq!(health.errors, vec!["AttachFailed"]);
}

#[test]
fn format_health_json_supports_pretty_and_compact_output() {
    let health = make_health();

    let compact = format_health_json(&health, false);
    let pretty = format_health_json(&health, true);

    assert!(!compact.contains('\n'));
    assert!(pretty.contains('\n'));
    assert!(compact.contains("\"ping\":true"));
    assert!(pretty.contains("\"snapshot\": false"));
}

#[test]
fn write_health_human_includes_optional_fields_warnings_and_errors() {
    let health = make_health();
    let mut output = Vec::new();

    write_health_human(&mut output, &health).expect("human health output should write");

    let output = String::from_utf8(output).expect("output should be valid UTF-8");
    assert!(output.contains("transport:  cdp"));
    assert!(output.contains("target_id:  -"));
    assert!(output.contains("title:      Figma"));
    assert!(output.contains("latency:    17ms"));
    assert!(output.contains("warning:    snapshot check failed"));
    assert!(output.contains("error:      Runtime.enable failed"));
}

#[test]
fn write_health_human_formats_missing_optional_fields_and_inverse_statuses() {
    let health = RuntimeHealth {
        transport: "cdp".to_string(),
        host: "127.0.0.1".to_string(),
        port: 9222,
        target_id: Some("t-human".to_string()),
        target_title: None,
        target_url: None,
        target_score: None,
        ping_ok: false,
        snapshot_ok: true,
        latency_ms: None,
        ping_script_version: "ping-v1".to_string(),
        snapshot_script_version: "snapshot-v1".to_string(),
        warnings: vec![],
        errors: vec![],
    };
    let mut output = Vec::new();

    write_health_human(&mut output, &health).expect("human health output should write");

    let output = String::from_utf8(output).expect("output should be valid UTF-8");
    assert!(output.contains("target_id:  t-human"));
    assert!(output.contains("title:      -"));
    assert!(output.contains("url:        -"));
    assert!(output.contains("ping:       fail"));
    assert!(output.contains("snapshot:   ok"));
    assert!(!output.contains("latency:"));
    assert!(!output.contains("warning:"));
    assert!(!output.contains("error:"));
}

#[test]
fn render_health_formats_fallback_json_output() {
    let ctx = make_context(true, true);
    let mut output = Vec::new();

    render_health(&ctx, Err(RuntimeError::AttachFailed), &mut output)
        .expect("fallback health should render");

    let output = String::from_utf8(output).expect("output should be valid UTF-8");
    assert!(output.contains("\"errors\": ["));
    assert!(output.contains("\"AttachFailed\""));
}

#[test]
fn render_health_formats_human_output() {
    let ctx = make_context(false, false);
    let mut output = Vec::new();

    render_health(&ctx, Ok(make_health()), &mut output).expect("health should render");

    let output = String::from_utf8(output).expect("output should be valid UTF-8");
    assert!(output.contains("ping:       ok"));
    assert!(output.contains("snapshot:   fail"));
}

#[test]
fn render_health_propagates_json_writer_failures() {
    let ctx = make_context(true, false);
    let mut writer = failing_writer::FailAfterNWrites::new(0);

    let error = render_health(&ctx, Ok(make_health()), &mut writer)
        .expect_err("JSON rendering should propagate write failures");

    assert!(error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|io_error| io_error.kind() == std::io::ErrorKind::Other));
}

#[test]
fn render_health_propagates_human_writer_failures() {
    let ctx = make_context(false, false);
    let mut writer = failing_writer::FailAfterNWrites::new(0);

    let error = render_health(&ctx, Ok(make_health()), &mut writer)
        .expect_err("human rendering should propagate write failures");

    assert!(error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|io_error| io_error.kind() == std::io::ErrorKind::Other));
}

#[test]
fn write_health_human_propagates_writer_failures_for_each_output_line() {
    for successful_writes in 0..=10 {
        let mut writer = failing_writer::FailAfterNWrites::new(successful_writes);
        let error = write_health_human(&mut writer, &make_health())
            .expect_err("human rendering should stop on writer failure");

        assert!(error
            .downcast_ref::<std::io::Error>()
            .is_some_and(|io_error| io_error.kind() == std::io::ErrorKind::Other));
    }
}
