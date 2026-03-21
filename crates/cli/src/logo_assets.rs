use rust_embed::RustEmbed;

// RustEmbed が生成する補助コードは coverage 集計から除外する。
#[derive(RustEmbed)]
#[folder = "../../assets/fonts/"]
pub(crate) struct FontAssets;
