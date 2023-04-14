use crate::shared::constants::ASSET_DIRECTORY;
use crate::shared::structs::config::configuration::KOU;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static COMMON_SETTINGS: Lazy<CommonSettings> =
    Lazy::new(|| initialize().expect("Failed to initialize common settings."));

const COMMON_SETTINGS_FILE_NAME: &str = "/common_settings.toml";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CommonSettings {
    pub activities: Vec<String>,
    pub greetings: Vec<String>,
    pub common_responses: Vec<String>,
    pub failed_messages: Vec<String>,
}

fn initialize() -> anyhow::Result<CommonSettings> {
    if !std::path::Path::new(ASSET_DIRECTORY).exists() {
        std::fs::create_dir(ASSET_DIRECTORY)?;
    }

    let common_settings_path = String::from(ASSET_DIRECTORY) + COMMON_SETTINGS_FILE_NAME;
    let is_kou = KOU.get().copied().unwrap_or(false);
    if !std::path::Path::new(&common_settings_path).exists() {
        // Read from backup JSON files.
        let json_path = String::from(ASSET_DIRECTORY)
            + if is_kou {
                "/json/backup/common_kou.json"
            } else {
                "/json/backup/common_taiga.json"
            };

        let json = std::fs::read(json_path)?;
        let deserialized_json: CommonSettings = serde_json::from_slice(&json)?;
        let serialized_toml = toml::to_string_pretty(&deserialized_json)?;
        std::fs::write(&common_settings_path, serialized_toml)?;
        Ok(deserialized_json)
    } else {
        let toml = std::fs::read_to_string(&common_settings_path)?;
        let deserialized_toml: CommonSettings = toml::from_str(&toml)?;
        Ok(deserialized_toml)
    }
}
