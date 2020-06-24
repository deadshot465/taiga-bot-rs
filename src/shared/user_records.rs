use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct UserRecords {
    pub route: HashMap<String, HashMap<String, u16>>,
    pub valentine: HashMap<String, u16>
}

impl UserRecords {
    pub fn new() -> Self {
        UserRecords {
            route: HashMap::new(),
            valentine: HashMap::new()
        }
    }
}