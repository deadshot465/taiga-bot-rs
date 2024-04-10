use serde::{Deserialize, Serialize};

use crate::shared::constants::CONFIG_DIRECTORY;

const CONFIG_FILE_NAME: &str = "/config.toml";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Configuration {
    pub prefix: String,
    pub token: String,
    pub log_level: String,
    pub google_api_key: String,
    pub login_name: String,
    pub login_pass: String,
    pub rapid_api_key: String,
    pub mention_reply_chance: i32,
    pub random_reply_chance: i32,
    pub application_id: u64,
    pub bot_id: u64,
    pub recreate_global_slash_commands: bool,
    pub version_number: String,
    pub update_date: String,
    pub unsplash_token: String,
    pub cat_token: String,
    pub server_endpoint: String,
    pub exchange_rate_api_key: String,
    pub general_channel_ids: Vec<u64>,
    pub openai_api_key: String,
    pub openai_reply_chance: i32,
    pub skip_user_ids: Vec<u64>,
}

impl Configuration {
    pub(self) fn new() -> Self {
        Configuration {
            prefix: "ta!".to_string(),
            token: "".to_string(),
            log_level: "DEBUG".to_string(),
            google_api_key: "".to_string(),
            login_name: "".to_string(),
            login_pass: "".to_string(),
            rapid_api_key: "".to_string(),
            mention_reply_chance: 25,
            random_reply_chance: 10,
            application_id: 0,
            bot_id: 0,
            recreate_global_slash_commands: true,
            version_number: "".to_string(),
            update_date: "".to_string(),
            unsplash_token: "".to_string(),
            cat_token: "".to_string(),
            server_endpoint: "http://localhost:8080".to_string(),
            exchange_rate_api_key: "".to_string(),
            general_channel_ids: vec![],
            openai_api_key: "".to_string(),
            openai_reply_chance: 25,
            skip_user_ids: vec![],
        }
    }

    pub fn write_config(&self) -> anyhow::Result<()> {
        let config_path = String::from(CONFIG_DIRECTORY) + CONFIG_FILE_NAME;
        let serialized_toml = toml::to_string_pretty(self)?;
        std::fs::write(config_path, serialized_toml)?;
        Ok(())
    }
}

pub fn initialize() -> anyhow::Result<Configuration> {
    if !std::path::Path::new(CONFIG_DIRECTORY).exists() {
        std::fs::create_dir(CONFIG_DIRECTORY)?;
    }

    let config_path = String::from(CONFIG_DIRECTORY) + CONFIG_FILE_NAME;
    if !std::path::Path::new(&config_path).exists() {
        let new_config = Configuration::new();
        new_config.write_config()?;
        Ok(new_config)
    } else {
        let toml = std::fs::read_to_string(config_path)?;
        let deserialized_toml: Configuration = toml::from_str(&toml)?;
        Ok(deserialized_toml)
    }
}
