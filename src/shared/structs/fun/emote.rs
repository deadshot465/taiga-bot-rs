use serde::{Deserialize, Serialize};

use crate::shared::constants::{ASSET_DIRECTORY, CONFIG_DIRECTORY};

const EMOTE_LIST_FILE_NAME: &str = "/emote_list.toml";

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct EmoteList {
    pub emotes: Vec<Emote>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Emote {
    pub name: String,
    pub id: u64,
    pub link: String,
    pub raw: String,
}

impl EmoteList {
    pub fn write_emote_list(&self) -> anyhow::Result<()> {
        let emote_list_path = String::from(CONFIG_DIRECTORY) + EMOTE_LIST_FILE_NAME;
        let serialized_toml = toml::to_string_pretty(self)?;
        std::fs::write(emote_list_path, serialized_toml)?;
        Ok(())
    }
}

pub fn initialize_emote_list() -> anyhow::Result<EmoteList> {
    if !std::path::Path::new(CONFIG_DIRECTORY).exists() {
        std::fs::create_dir(CONFIG_DIRECTORY)?;
    }

    let emote_list_path = String::from(CONFIG_DIRECTORY) + EMOTE_LIST_FILE_NAME;
    if !std::path::Path::new(&emote_list_path).exists() {
        // Read from json
        let json_path = String::from(ASSET_DIRECTORY) + "/json/backup/emotes.json";
        let json = std::fs::read(json_path).unwrap_or_default();
        let mut deserialized_json = serde_json::from_slice::<EmoteList>(&json).unwrap_or_default();
        deserialized_json
            .emotes
            .dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
        deserialized_json
            .emotes
            .sort_unstable_by(|a, b| a.name.cmp(&b.name));
        deserialized_json.write_emote_list()?;
        Ok(deserialized_json)
    } else {
        let toml = std::fs::read_to_string(&emote_list_path)?;
        let mut deserialized_toml = toml::from_str::<EmoteList>(&toml)?;
        deserialized_toml
            .emotes
            .sort_unstable_by(|a, b| a.name.cmp(&b.name));
        deserialized_toml.write_emote_list()?;
        Ok(deserialized_toml)
    }
}
