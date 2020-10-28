use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRecords {
    pub route: HashMap<String, HashMap<String, u16>>,
    pub valentine: HashMap<String, u16>,
    #[serde(default)]
    pub replies: u32,
}

impl UserRecords {
    pub fn new() -> Self {
        UserRecords {
            route: HashMap::new(),
            valentine: HashMap::new(),
            replies: 0,
        }
    }
}

impl Default for UserRecords {
    fn default() -> Self {
        Self::new()
    }
}
