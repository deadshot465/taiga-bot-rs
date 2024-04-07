use serde::{Deserialize, Serialize};

use crate::shared::constants::ASSET_DIRECTORY;

const SHIP_MESSAGES_FILE_NAME: &str = "/json/ship_messages.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShipMessage {
    pub max_score: u8,
    pub message: String,
}

pub fn initialize_ship_messages() -> Vec<ShipMessage> {
    let ship_messages_path = String::from(ASSET_DIRECTORY) + SHIP_MESSAGES_FILE_NAME;
    let json = std::fs::read(ship_messages_path).expect("Failed to read ship messages from disk.");
    serde_json::from_slice(&json).expect("Failed to deserialize ship messages.")
}
