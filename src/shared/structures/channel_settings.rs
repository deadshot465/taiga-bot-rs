use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Deserialize, Serialize)]
pub struct ChannelSettings {
    pub enabled_channels: HashSet<u64>,
    pub ignored_channels: HashSet<u64>,
}

impl ChannelSettings {
    pub fn new() -> Self {
        ChannelSettings {
            enabled_channels: HashSet::new(),
            ignored_channels: HashSet::new(),
        }
    }
}

impl Default for ChannelSettings {
    fn default() -> Self {
        Self::new()
    }
}
