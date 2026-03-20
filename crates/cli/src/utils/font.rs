/// フォント名を正規化する
///
/// - 拡張子 (.flf, .flc, .tlf) がない場合は .flf を追加
/// - アンダーバー(_)をスペースに変換してマッチングを試みる
/// - 元の名前でもマッチングを試みる
pub fn normalize_font_name(input: &str) -> String {
    // すでに拡張子がある場合はそのまま返す
    if input.ends_with(".flf") || input.ends_with(".flc") || input.ends_with(".tlf") {
        return input.to_string();
    }

    // 拡張子がない場合は .flf を追加
    format!("{}.flf", input)
}

/// フォント名のバリエーションを生成する
///
/// スペースとアンダーバーの両方のパターンを返す
pub fn generate_font_name_variants(input: &str) -> Vec<String> {
    let normalized = normalize_font_name(input);
    let mut variants = vec![normalized.clone()];

    // アンダーバーをスペースに変換したバージョン
    if normalized.contains('_') {
        variants.push(normalized.replace('_', " "));
    }

    // スペースをアンダーバーに変換したバージョン
    if normalized.contains(' ') {
        variants.push(normalized.replace(' ', "_"));
    }

    variants
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_font_name() {
        assert_eq!(normalize_font_name("Standard"), "Standard.flf");
        assert_eq!(normalize_font_name("Standard.flf"), "Standard.flf");
        assert_eq!(normalize_font_name("ANSI Shadow"), "ANSI Shadow.flf");
    }

    #[test]
    fn test_generate_font_name_variants() {
        let variants = generate_font_name_variants("ANSI_Shadow");
        assert!(variants.contains(&"ANSI_Shadow.flf".to_string()));
        assert!(variants.contains(&"ANSI Shadow.flf".to_string()));

        let variants = generate_font_name_variants("ANSI Shadow");
        assert!(variants.contains(&"ANSI Shadow.flf".to_string()));
        assert!(variants.contains(&"ANSI_Shadow.flf".to_string()));
    }
}
