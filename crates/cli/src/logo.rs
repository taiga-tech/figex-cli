use crate::{logo_assets::FontAssets, utils::font::generate_font_name_variants};
use anyhow::{Context, Result};
use figlet_rs::FIGlet;
use owo_colors::OwoColorize;
use std::io::{self, Write};

const LOGO_COLORS: &[(u8, u8, u8)] = &[
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

#[derive(Debug, Clone)]
pub struct LogoRenderer {
    font: FIGlet,
    font_name: String,
}

impl LogoRenderer {
    pub fn from_embedded_font(font_name: &str) -> Result<Self> {
        let variants = generate_font_name_variants(font_name);

        for variant in &variants {
            if let Some(font_data) = FontAssets::get(variant) {
                return Self::from_font_data(font_name, font_data.data.as_ref());
            }
        }

        anyhow::bail!(
            "Font '{}' not found. Tried variants: {:?}",
            font_name,
            variants
        )
    }

    pub fn from_font_data(font_name: &str, font_data: &[u8]) -> Result<Self> {
        let font_content =
            std::str::from_utf8(font_data).context("Font file is not valid UTF-8")?;
        let font = FIGlet::from_content(font_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse FIGlet font: {}", e))?;

        Ok(Self {
            font,
            font_name: font_name.to_owned(),
        })
    }

    pub fn render(&self, text: &str) -> Result<String> {
        self.font
            .convert(text)
            .map(|figure| figure.to_string())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Failed to convert text '{}' using font '{}'",
                    text,
                    self.font_name
                )
            })
    }
}

pub fn render_logo(font_name: &str, text: &str) -> Result<String> {
    LogoRenderer::from_embedded_font(font_name)
        .with_context(|| format!("Failed to load font: {}", font_name))?
        .render(text)
}

pub fn write_rendered_logo(writer: &mut dyn Write, rendered_logo: &str) -> io::Result<()> {
    for (index, line) in rendered_logo.lines().enumerate() {
        let (red, green, blue) = LOGO_COLORS[index % LOGO_COLORS.len()];
        writeln!(writer, "{}", line.truecolor(red, green, blue).bold())?;
    }

    Ok(())
}
