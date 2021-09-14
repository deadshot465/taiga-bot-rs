use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub static RANDOM_RESPONSES: OnceCell<Vec<RandomResponse>> = OnceCell::new();

const RANDOM_RESPONSES_FILE_NAME: &str = "/random_responses.dhall";

#[derive(Deserialize, Serialize, Clone)]
pub struct RandomResponse {
    pub keyword: String,
    pub messages: Vec<String>,
    pub reactions: Vec<String>,
}

pub fn initialize() -> anyhow::Result<()> {
    if !std::path::Path::new(ASSET_DIRECTORY).exists() {
        std::fs::create_dir(ASSET_DIRECTORY)?;
    }

    let random_responses_path = String::from(ASSET_DIRECTORY) + RANDOM_RESPONSES_FILE_NAME;
    if !std::path::Path::new(&random_responses_path).exists() {
        let json_path = String::from(ASSET_DIRECTORY) + "/json/random_responses.json";
        let json = std::fs::read(&json_path)?;
        let deserialized_json: Vec<RandomResponse> = serde_json::from_slice(&json)?;
        let serialized_dhall = serde_dhall::serialize(&deserialized_json).to_string()?;
        std::fs::write(&random_responses_path, serialized_dhall)?;
        RANDOM_RESPONSES.get_or_init(|| deserialized_json);
    } else {
        let deserialized_dhall: Vec<RandomResponse> =
            serde_dhall::from_file(&random_responses_path).parse()?;
        RANDOM_RESPONSES.get_or_init(|| deserialized_dhall);
    }
    Ok(())
}
