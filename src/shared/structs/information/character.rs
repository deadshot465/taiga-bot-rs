use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const ROUTES_FILE_NAME: &str = "/json/routes.json";
const VALENTINES_FILE_NAME: &str = "/json/valentines.json";

pub static ROUTES: Lazy<Vec<Character>> = Lazy::new(|| {
    let routes_path = String::from(ASSET_DIRECTORY) + ROUTES_FILE_NAME;
    let json = std::fs::read(&routes_path).expect("Failed to read routes from local file.");
    serde_json::from_slice(&json).expect("Failed to deserialize routes.")
});

pub static VALENTINES: Lazy<Vec<Character>> = Lazy::new(|| {
    let valentines_path = String::from(ASSET_DIRECTORY) + VALENTINES_FILE_NAME;
    let json = std::fs::read(&valentines_path).expect("Failed to read valentines from local file.");
    serde_json::from_slice(&json).expect("Failed to deserialize valentines.")
});

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
