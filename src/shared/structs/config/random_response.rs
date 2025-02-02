use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::shared::constants::ASSET_DIRECTORY;

const RANDOM_RESPONSES_FILE_NAME: &str = "/random_responses.toml";

#[derive(Debug, Clone)]
pub struct RandomResponse {
    pub responses: Responses,
    pub keywords: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Responses {
    pub random_responses: Vec<Response>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Response {
    pub keyword: String,
    pub messages: Vec<String>,
    pub reactions: Vec<String>,
}

impl Responses {
    pub(self) fn new(random_responses: Vec<Response>) -> Self {
        Responses { random_responses }
    }
}

pub fn initialize_random_response() -> anyhow::Result<RandomResponse> {
    let responses = initialize_responses()?;
    let keywords = initialize_random_responses_keywords(&responses);
    Ok(RandomResponse {
        responses,
        keywords,
    })
}

pub fn get_random_message(random_response: &RandomResponse, keyword: &str) -> String {
    let keyword = keyword.trim().to_lowercase();
    random_response
        .responses
        .random_responses
        .iter()
        .find(|m| m.keyword.as_str() == keyword)
        .and_then(|res| {
            let mut rng = rand::rng();
            res.messages.choose(&mut rng)
        })
        .cloned()
        .unwrap_or_else(|| "Oops...".into())
}

pub fn get_random_reaction(random_response: &RandomResponse, keyword: &str) -> String {
    let keyword = keyword.trim().to_lowercase();
    random_response
        .responses
        .random_responses
        .iter()
        .find(|m| m.keyword.as_str() == keyword)
        .and_then(|res| {
            let mut rng = rand::rng();
            res.reactions.choose(&mut rng)
        })
        .cloned()
        .unwrap_or_else(|| "Oops...".into())
}

pub fn get_shuffled_keywords(random_response: &RandomResponse) -> Vec<String> {
    let mut keywords = random_response.keywords.to_vec();
    let mut rng = rand::rng();
    keywords.shuffle(&mut rng);
    keywords
}

fn initialize_responses() -> anyhow::Result<Responses> {
    if !std::path::Path::new(ASSET_DIRECTORY).exists() {
        std::fs::create_dir(ASSET_DIRECTORY)?;
    }

    let random_responses_path = String::from(ASSET_DIRECTORY) + RANDOM_RESPONSES_FILE_NAME;
    if !std::path::Path::new(&random_responses_path).exists() {
        let json_path = String::from(ASSET_DIRECTORY) + "/json/backup/random_responses.json";
        let json = std::fs::read(json_path)?;
        let deserialized_json: Vec<Response> = serde_json::from_slice(&json)?;
        let responses = Responses::new(deserialized_json);
        let serialized_toml = toml::to_string_pretty(&responses)?;
        std::fs::write(&random_responses_path, serialized_toml)?;
        Ok(responses)
    } else {
        let toml = std::fs::read_to_string(&random_responses_path)?;
        let deserialized_toml: Responses = toml::from_str(&toml)?;
        Ok(deserialized_toml)
    }
}

fn initialize_random_responses_keywords(responses: &Responses) -> Vec<String> {
    responses
        .random_responses
        .iter()
        .map(|res| &res.keyword)
        .map(|s| format!(" {} ", s))
        .collect::<Vec<_>>()
}
