use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub static RANDOM_RESPONSES: Lazy<Responses> =
    Lazy::new(|| initialize().expect("Failed to initialize random responses."));
pub static RANDOM_RESPONSES_KEYWORDS: Lazy<Vec<String>> =
    Lazy::new(|| initialize_random_responses_keywords());

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

pub fn get_random_message(keyword: &str) -> String {
    let keyword = keyword.trim().to_lowercase();
    RANDOM_RESPONSES
        .random_responses
        .iter()
        .find(|m| m.keyword.as_str() == &keyword)
        .and_then(|res| {
            let mut rng = rand::thread_rng();
            res.messages.choose(&mut rng)
        })
        .cloned()
        .unwrap_or("Oops...".into())
}

pub fn get_random_reaction(keyword: &str) -> String {
    let keyword = keyword.trim().to_lowercase();
    RANDOM_RESPONSES
        .random_responses
        .iter()
        .find(|m| m.keyword.as_str() == &keyword)
        .and_then(|res| {
            let mut rng = rand::thread_rng();
            res.reactions.choose(&mut rng)
        })
        .cloned()
        .unwrap_or("Oops...".into())
}

fn initialize() -> anyhow::Result<Responses> {
    if !std::path::Path::new(ASSET_DIRECTORY).exists() {
        std::fs::create_dir(ASSET_DIRECTORY)?;
    }

    let random_responses_path = String::from(ASSET_DIRECTORY) + RANDOM_RESPONSES_FILE_NAME;
    if !std::path::Path::new(&random_responses_path).exists() {
        let json_path = String::from(ASSET_DIRECTORY) + "/json/backup/random_responses.json";
        let json = std::fs::read(&json_path)?;
        let deserialized_json: Vec<RandomResponse> = serde_json::from_slice(&json)?;
        let responses = Responses::new(deserialized_json);
        let serialized_toml = toml::to_string_pretty(&responses)?;
        std::fs::write(&random_responses_path, serialized_toml)?;
        Ok(responses)
    } else {
        let toml = std::fs::read(&random_responses_path)?;
        let deserialized_toml: Responses = toml::from_slice(&toml)?;
        Ok(deserialized_toml)
    }
}

fn initialize_random_responses_keywords() -> Vec<String> {
    RANDOM_RESPONSES
        .random_responses
        .iter()
        .map(|res| &res.keyword)
        .map(|s| format!(" {} ", s))
        .collect::<Vec<_>>()
}
