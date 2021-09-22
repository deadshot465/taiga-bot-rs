use crate::shared::constants::RECORD_DIRECTORY;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub static QOTD_INFOS: Lazy<RwLock<QotdInfos>> =
    Lazy::new(|| RwLock::new(initialize().expect("Failed to initialize qotd infos.")));

const QOTD_INFOS_FILE_NAME: &str = "/qotd_infos.toml";

#[derive(Deserialize, Serialize, Clone)]
pub struct QotdInfos {
    pub qotd_infos: Vec<QotdInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct QotdInfo {
    pub thread_channel_id: u64,
    pub question: String,
    pub expiry: DateTime<Utc>,
    pub participated_members: Vec<u64>,
}

impl QotdInfos {
    pub(self) fn new() -> Self {
        QotdInfos { qotd_infos: vec![] }
    }

    pub fn purge_expired_qotds(&mut self) {
        let filtered_qotds = self
            .qotd_infos
            .iter()
            .filter(|info| Utc::now() < info.expiry)
            .cloned()
            .collect::<Vec<_>>();
        self.qotd_infos = filtered_qotds;
    }

    pub fn write_qotd_infos(&self) -> anyhow::Result<()> {
        let qotd_infos_path = String::from(RECORD_DIRECTORY) + QOTD_INFOS_FILE_NAME;
        let serialized_toml = toml::to_string_pretty(self)?;
        std::fs::write(qotd_infos_path, serialized_toml)?;
        Ok(())
    }
}

fn initialize() -> anyhow::Result<QotdInfos> {
    if !std::path::Path::new(RECORD_DIRECTORY).exists() {
        std::fs::create_dir(RECORD_DIRECTORY)?;
    }

    let qotd_infos_path = String::from(RECORD_DIRECTORY) + QOTD_INFOS_FILE_NAME;
    if !std::path::Path::new(&qotd_infos_path).exists() {
        let new_qotd_infos = QotdInfos::new();
        new_qotd_infos.write_qotd_infos()?;
        Ok(new_qotd_infos)
    } else {
        let toml = std::fs::read(qotd_infos_path)?;
        let mut deserialized_toml = toml::from_slice::<QotdInfos>(&toml)?;
        deserialized_toml.purge_expired_qotds();
        deserialized_toml.write_qotd_infos()?;
        Ok(deserialized_toml)
    }
}
