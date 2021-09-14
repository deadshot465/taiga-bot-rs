use crate::shared::constants::RECORD_DIRECTORY;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub static USER_RECORDS: OnceCell<RwLock<HashMap<String, UserRecord>>> = OnceCell::new();

const USER_RECORDS_FILE_NAME: &str = "/user_records.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct UserRecord {
    pub route: HashMap<String, HashMap<String, u16>>,
    pub valentine: HashMap<String, u16>,
    #[serde(default)]
    pub replies: u32,
}

impl UserRecord {
    pub fn new() -> Self {
        UserRecord {
            route: HashMap::new(),
            valentine: HashMap::new(),
            replies: 0,
        }
    }
}

impl Default for UserRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub fn initialize() -> anyhow::Result<()> {
    if !std::path::Path::new(RECORD_DIRECTORY).exists() {
        std::fs::create_dir(RECORD_DIRECTORY)?
    }

    let user_records_path = String::from(RECORD_DIRECTORY) + USER_RECORDS_FILE_NAME;
    if !std::path::Path::new(&user_records_path).exists() {
        let new_user_records: HashMap<String, UserRecord> = HashMap::new();
        write_user_records(&new_user_records)?;
        USER_RECORDS.get_or_init(|| RwLock::new(new_user_records));
    } else {
        let json = std::fs::read(&user_records_path)?;
        let deserialized_user_records: HashMap<String, UserRecord> = serde_json::from_slice(&json)?;
        USER_RECORDS.get_or_init(|| RwLock::new(deserialized_user_records));
    }

    Ok(())
}

pub fn write_user_records(user_records: &HashMap<String, UserRecord>) -> anyhow::Result<()> {
    let user_records_path = String::from(RECORD_DIRECTORY) + USER_RECORDS_FILE_NAME;
    let serialized_user_records = serde_json::to_string_pretty(user_records)?;
    std::fs::write(&user_records_path, &serialized_user_records)?;
    Ok(())
}
