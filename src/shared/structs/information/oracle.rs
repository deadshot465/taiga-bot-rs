use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const ORACLES_FILE_NAME: &str = "/json/oracles.json";

pub static ORACLES: Lazy<Vec<Oracle>> = Lazy::new(|| {
    let oracles_path = String::from(ASSET_DIRECTORY) + ORACLES_FILE_NAME;
    let oracles = std::fs::read(&oracles_path).expect("Failed to read oracles from local file.");
    serde_json::from_slice(&oracles).expect("Failed to deserialize oracles.")
});

#[derive(Deserialize, Serialize, Clone)]
pub struct Oracle {
    pub no: u8,
    pub fortune: String,
    pub meaning: String,
    pub content: String,
}
