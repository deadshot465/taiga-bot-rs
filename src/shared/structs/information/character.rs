use serde::{Deserialize, Serialize};

use crate::shared::constants::ASSET_DIRECTORY;

const ROUTES_FILE_NAME: &str = "/json/routes.json";
const VALENTINES_FILE_NAME: &str = "/json/valentines.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub age: u8,
    pub birthday: String,
    pub animal: String,
    pub color: String,
    pub emote_id: String,
}

pub fn initialize_routes() -> Vec<Character> {
    let routes_path = String::from(ASSET_DIRECTORY) + ROUTES_FILE_NAME;
    let json = std::fs::read(routes_path).expect("Failed to read routes from local file.");
    serde_json::from_slice(&json).expect("Failed to deserialize routes.")
}

pub fn initialize_valentines() -> Vec<Character> {
    let valentines_path = String::from(ASSET_DIRECTORY) + VALENTINES_FILE_NAME;
    let json = std::fs::read(valentines_path).expect("Failed to read valentines from local file.");
    serde_json::from_slice(&json).expect("Failed to deserialize valentines.")
}
