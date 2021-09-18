use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static CONVERSION_TABLE: Lazy<ConversionTable> = Lazy::new(|| {
    let conversion_table_path = String::from(ASSET_DIRECTORY) + CONVERSION_TABLE_FILE_NAME;
    let json = std::fs::read(&conversion_table_path)
        .expect("Failed to read conversion table from local disk.");
    serde_json::from_slice(&json).expect("Failed to deserialize conversion table.")
});

const CONVERSION_TABLE_FILE_NAME: &str = "/json/conversion_table.json";

#[derive(Deserialize, Serialize)]
pub struct ConversionTable {
    pub length: HashMap<String, HashMap<String, f32>>,
    pub weight: HashMap<String, HashMap<String, f32>>,
    pub temperature: HashMap<String, HashMap<String, f32>>,
}
