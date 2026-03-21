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
