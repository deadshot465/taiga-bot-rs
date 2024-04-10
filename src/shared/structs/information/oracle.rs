use serde::{Deserialize, Serialize};

use crate::shared::constants::ASSET_DIRECTORY;

const ORACLES_FILE_NAME: &str = "/json/oracles.json";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Oracle {
    pub no: u8,
    pub fortune: String,
    pub meaning: String,
    pub content: String,
}

pub fn initialize_oracles() -> Vec<Oracle> {
    let oracles_path = String::from(ASSET_DIRECTORY) + ORACLES_FILE_NAME;
    let oracles = std::fs::read(oracles_path).expect("Failed to read oracles from local file.");
    serde_json::from_slice(&oracles).expect("Failed to deserialize oracles.")
}
