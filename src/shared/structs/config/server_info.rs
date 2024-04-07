use serde::{Deserialize, Serialize};

use crate::shared::constants::{
    CONFIG_DIRECTORY, KOU_SERVER_ADMIN_ROLE_ID, KOU_SERVER_ID, KOU_SERVER_QOTD_CHANNEL_ID,
    TAIGA_SERVER_ADMIN_ROLE_ID, TAIGA_SERVER_ID, TAIGA_SERVER_WINTER_SPLENDOR_ROLE_ID,
};

const SERVER_INFOS_FILE_NAME: &str = "/server_infos.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerInfos {
    pub server_infos: Vec<ServerInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerInfo {
    pub server_id: u64,
    pub admin_role_ids: Vec<u64>,
    pub qotd_channel_ids: Vec<u64>,
}

impl ServerInfos {
    pub fn write_server_infos(&self) -> anyhow::Result<()> {
        let server_infos_path = String::from(CONFIG_DIRECTORY) + SERVER_INFOS_FILE_NAME;
        let serialized_toml = toml::to_string_pretty(self)?;
        std::fs::write(server_infos_path, serialized_toml)?;
        Ok(())
    }
}

pub fn initialize_server_infos() -> anyhow::Result<ServerInfos> {
    if !std::path::Path::new(CONFIG_DIRECTORY).exists() {
        std::fs::create_dir(CONFIG_DIRECTORY)?;
    }

    let server_infos_path = String::from(CONFIG_DIRECTORY) + SERVER_INFOS_FILE_NAME;
    if !std::path::Path::new(&server_infos_path).exists() {
        let new_server_infos = ServerInfos {
            server_infos: vec![
                ServerInfo {
                    server_id: KOU_SERVER_ID,
                    admin_role_ids: vec![KOU_SERVER_ADMIN_ROLE_ID],
                    qotd_channel_ids: vec![KOU_SERVER_QOTD_CHANNEL_ID],
                },
                ServerInfo {
                    server_id: TAIGA_SERVER_ID,
                    admin_role_ids: vec![
                        TAIGA_SERVER_ADMIN_ROLE_ID,
                        TAIGA_SERVER_WINTER_SPLENDOR_ROLE_ID,
                    ],
                    qotd_channel_ids: vec![],
                },
            ],
        };
        new_server_infos.write_server_infos()?;
        Ok(new_server_infos)
    } else {
        let toml = std::fs::read_to_string(&server_infos_path)?;
        Ok(toml::from_str(&toml)?)
    }
}
