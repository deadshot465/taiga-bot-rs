use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::constants::ASSET_DIRECTORY;

const CONVERSION_TABLE_FILE_NAME: &str = "/json/conversion_table.json";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConversionTable {
    pub length: HashMap<String, HashMap<String, f32>>,
    pub weight: HashMap<String, HashMap<String, f32>>,
    pub temperature: HashMap<String, HashMap<String, f32>>,
}

pub fn initialize_conversion_table() -> ConversionTable {
    let conversion_table_path = String::from(ASSET_DIRECTORY) + CONVERSION_TABLE_FILE_NAME;
    let json = std::fs::read(conversion_table_path)
        .expect("Failed to read conversion table from local disk.");
    serde_json::from_slice(&json).expect("Failed to deserialize conversion table.")
}
