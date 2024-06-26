use serde::{Deserialize, Serialize};

use crate::shared::constants::CONFIG_DIRECTORY;

const CHANNEL_CONTROL_FILE_NAME: &str = "/channel_control.toml";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChannelControl {
    pub enabled_channels: Vec<u64>,
    pub ignored_channels: Vec<u64>,
}

impl ChannelControl {
    pub fn new() -> Self {
        ChannelControl {
            enabled_channels: vec![],
            ignored_channels: vec![],
        }
    }

    pub fn write_channel_control(&self) -> anyhow::Result<()> {
        let channel_control_path = String::from(CONFIG_DIRECTORY) + CHANNEL_CONTROL_FILE_NAME;
        let serialized_toml = toml::to_string_pretty(self)?;
        std::fs::write(channel_control_path, serialized_toml)?;
        Ok(())
    }
}

pub fn initialize() -> anyhow::Result<ChannelControl> {
    if !std::path::Path::new(CONFIG_DIRECTORY).exists() {
        std::fs::create_dir(CONFIG_DIRECTORY)?;
    }

    let channel_control_path = String::from(CONFIG_DIRECTORY) + CHANNEL_CONTROL_FILE_NAME;
    if !std::path::Path::new(&channel_control_path).exists() {
        let new_channel_control = ChannelControl::new();
        new_channel_control.write_channel_control()?;
        Ok(new_channel_control)
    } else {
        let toml = std::fs::read_to_string(&channel_control_path)?;
        let deserialized_toml: ChannelControl = toml::from_str(&toml)?;
        Ok(deserialized_toml)
    }
}
