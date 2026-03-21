use figex_cli::utils::font::{generate_font_name_variants, normalize_font_name};

#[test]
fn normalize_font_name_preserves_existing_extensions() {
    assert_eq!(normalize_font_name("Standard.flf"), "Standard.flf");
    assert_eq!(normalize_font_name("Standard.flc"), "Standard.flc");
    assert_eq!(normalize_font_name("Standard.tlf"), "Standard.tlf");
}

#[test]
fn normalize_font_name_adds_default_extension() {
    assert_eq!(normalize_font_name("Standard"), "Standard.flf");
    assert_eq!(normalize_font_name("ANSI Shadow"), "ANSI Shadow.flf");
}

#[test]
fn generate_font_name_variants_supports_underscores_and_spaces() {
    let underscore_variants = generate_font_name_variants("ANSI_Shadow");
    let space_variants = generate_font_name_variants("ANSI Shadow");

    assert!(underscore_variants.contains(&"ANSI_Shadow.flf".to_string()));
    assert!(underscore_variants.contains(&"ANSI Shadow.flf".to_string()));
    assert!(space_variants.contains(&"ANSI Shadow.flf".to_string()));
    assert!(space_variants.contains(&"ANSI_Shadow.flf".to_string()));
}
