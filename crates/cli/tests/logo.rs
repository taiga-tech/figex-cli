#[path = "support/failing_writer.rs"]
mod failing_writer;

use figex_cli::logo::{render_logo, write_rendered_logo, LogoRenderer};
use std::io::{self, Write};

#[test]
fn logo_renderer_rejects_invalid_utf8() {
    let error = LogoRenderer::from_font_data("inline-font", &[0xff])
        .expect_err("invalid UTF-8 should fail");

    assert_eq!(error.to_string(), "Font file is not valid UTF-8");
}

#[test]
fn logo_renderer_rejects_invalid_figlet_content() {
    let error = LogoRenderer::from_font_data("inline-font", b"not a figlet font")
        .expect_err("invalid figlet data should fail");

    assert!(error
        .to_string()
        .starts_with("Failed to parse FIGlet font: "));
}

#[test]
fn logo_renderer_supports_embedded_font_variants() {
    LogoRenderer::from_embedded_font("DOS Rebel")
        .expect("space-separated font name should resolve");
    LogoRenderer::from_embedded_font("DOS_Rebel").expect("underscore font name should resolve");
}

#[test]
fn logo_renderer_returns_error_for_missing_fonts() {
    let error =
        LogoRenderer::from_embedded_font("missing-font").expect_err("missing font should fail");

    assert!(error
        .to_string()
        .contains("Font 'missing-font' not found. Tried variants:"));
}

#[test]
fn render_logo_returns_ascii_art_for_valid_input() {
    let logo = render_logo("DOS Rebel", "FIGEX").expect("logo rendering should succeed");

    assert!(!logo.trim().is_empty());
    assert!(logo.lines().filter(|line| !line.trim().is_empty()).count() > 1);
}

#[test]
fn render_logo_adds_context_when_font_loading_fails() {
    let error = render_logo("missing-font", "FIGEX")
        .expect_err("missing embedded font should include context");

    assert_eq!(error.to_string(), "Failed to load font: missing-font");
}

#[test]
fn logo_renderer_returns_error_for_empty_text() {
    let renderer =
        LogoRenderer::from_embedded_font("DOS Rebel").expect("embedded font should load");
    let error = renderer.render("").expect_err("empty text should fail");

    assert_eq!(
        error.to_string(),
        "Failed to convert text '' using font 'DOS Rebel'"
    );
}

#[test]
fn write_rendered_logo_writes_multiline_output() {
    let rendered_logo = render_logo("DOS Rebel", "FIGEX").expect("logo rendering should succeed");
    let mut stdout = Vec::new();

    write_rendered_logo(&mut stdout, &rendered_logo).expect("writer output should succeed");

    let stdout = String::from_utf8(stdout).expect("stdout should be valid UTF-8");

    assert!(!stdout.trim().is_empty());
    assert!(stdout.lines().count() > 1);
}

#[test]
fn write_rendered_logo_propagates_writer_failures() {
    let mut writer = failing_writer::FailingWriter;

    let error = write_rendered_logo(&mut writer, "line 1\nline 2")
        .expect_err("writer failures should bubble up");

    assert_eq!(error.kind(), io::ErrorKind::Other);
    writer
        .flush()
        .expect("failing writer flush should still succeed");
}
