use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const SHIP_MESSAGES_FILE_NAME: &str = "/json/ship_messages.json";

pub static SHIP_MESSAGES: Lazy<Vec<ShipMessage>> = Lazy::new(|| {
    let ship_messages_path = String::from(ASSET_DIRECTORY) + SHIP_MESSAGES_FILE_NAME;
    let json = std::fs::read(ship_messages_path).expect("Failed to read ship messages from disk.");
    serde_json::from_slice(&json).expect("Failed to deserialize ship messages.")
});

#[derive(Deserialize, Serialize, Clone)]
pub struct ShipMessage {
    pub max_score: u8,
    pub message: String,
}
