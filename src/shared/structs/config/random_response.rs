use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

pub static RANDOM_RESPONSES: OnceCell<Responses> = OnceCell::new();

const RANDOM_RESPONSES_FILE_NAME: &str = "/random_responses.toml";

#[derive(Deserialize, Serialize, Clone)]
pub struct Responses {
    pub random_responses: Vec<RandomResponse>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RandomResponse {
    pub keyword: String,
    pub messages: Vec<String>,
    pub reactions: Vec<String>,
}

impl Responses {
    pub(self) fn new(random_responses: Vec<RandomResponse>) -> Self {
        Responses { random_responses }
    }
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
        let responses = Responses::new(deserialized_json);
        let serialized_toml = toml::to_string_pretty(&responses)?;
        std::fs::write(&random_responses_path, serialized_toml)?;
        RANDOM_RESPONSES.get_or_init(|| responses);
    } else {
        let toml = std::fs::read(&random_responses_path)?;
        let deserialized_toml: Responses = toml::from_slice(&toml)?;
        RANDOM_RESPONSES.get_or_init(|| deserialized_toml);
    }
    Ok(())
}
