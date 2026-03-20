use crate::utils::font::generate_font_name_variants;
use anyhow::{Context, Ok, Result};
use figlet_rs::FIGlet;
use owo_colors::OwoColorize;
use rust_embed::RustEmbed;

// フォントファイルをバイナリに埋め込む（バイナリ配布対応）
#[derive(RustEmbed)]
#[folder = "../../assets/fonts/"]
struct FontAssets;

/// 指定されたフォント名に基づいてFIGletフォントを選択する
fn select_font(font_name: &str) -> Result<FIGlet> {
    // フォント名のバリエーションを生成（拡張子追加、スペース/アンダーバー変換）
    let variants = generate_font_name_variants(font_name);

    // バリエーションを順に試す
    for variant in &variants {
        if let Some(font_data) = FontAssets::get(variant) {
            // バイト列を文字列に変換してFIGfontを生成
            let font_content = std::str::from_utf8(font_data.data.as_ref())
                .context("Font file is not valid UTF-8")?;

            return FIGlet::from_content(font_content)
                .map_err(|e| anyhow::anyhow!("Failed to parse FIGlet font: {}", e));
        }
    }

    // すべてのバリエーションで見つからなかった場合
    anyhow::bail!(
        "Font '{}' not found. Tried variants: {:?}",
        font_name,
        variants
    )
}

/// 指定されたフォントとテキストを使ってロゴを生成し、カラーで出力する
pub fn print_logo(font_name: &str, text: &str) -> Result<()> {
    let font =
        select_font(font_name).with_context(|| format!("Failed to load font: {}", font_name))?;

    let figure = font.convert(text).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to convert text '{}' using font '{}'",
            text,
            font_name
        )
    })?;

    let colors = [
        (255, 255, 255),
        // (242, 78, 30),  // オレンジ
        // (242, 78, 30),  // オレンジ
        // (10, 207, 131), // グリーン
        // (10, 207, 131), // グリーン
        // (26, 188, 254), // ブルー
        // (26, 188, 254), // ブルー
        // (162, 89, 255), // パープル
        // (162, 89, 255), // パープル
    ];

    for (i, line) in figure.to_string().lines().enumerate() {
        let (r, g, b) = colors[i % colors.len()];
        println!("{}", line.truecolor(r, g, b).bold());
    }

    Ok(())
}
