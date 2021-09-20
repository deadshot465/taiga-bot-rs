use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;

pub static SMITE_GIF_LINKS: Lazy<Vec<String>> = Lazy::new(|| {
    let smite_gif_links_path = String::from(ASSET_DIRECTORY) + SMITE_GIF_LINKS_FILE_NAME;
    let json = std::fs::read(&smite_gif_links_path)
        .expect("Failed to read smite gif links from local disk.");
    serde_json::from_slice(&json).expect("Failed to deserialize smite gif links.")
});

const SMITE_GIF_LINKS_FILE_NAME: &str = "/json/smite_links.json";
