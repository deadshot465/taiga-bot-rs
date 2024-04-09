use crate::shared::constants::{
    ASSET_DIRECTORY, CONFIG_DIRECTORY, KOU_SERVER_SMOTE_ROLE_ID, TAIGA_SERVER_SMOTE_ROLE_ID,
};
use crate::shared::structs::ContextData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, UserId};
use serenity::model::prelude::RoleId;
use std::sync::Arc;
use tokio::sync::RwLock;

const SMITE_GIF_LINKS_FILE_NAME: &str = "/json/smite_links.json";
const SMOTE_USER_LIST_FILE_NAME: &str = "/smote_users.toml";

#[derive(Debug, Clone)]
pub struct Smite {
    pub smite_gif_links: Vec<String>,
    pub smote_user_list: Arc<RwLock<SmoteUserList>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SmoteUserList {
    pub smote_users: Vec<SmoteUser>,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub struct SmoteUser {
    pub user_id: u64,
    pub due_time: DateTime<Utc>,
    pub guild_id: u64,
}

impl SmoteUserList {
    pub(self) fn new() -> Self {
        SmoteUserList {
            smote_users: vec![],
        }
    }

    pub fn write_smote_user_list(&self) -> anyhow::Result<()> {
        let smote_user_list_path = String::from(CONFIG_DIRECTORY) + SMOTE_USER_LIST_FILE_NAME;
        let serialized_toml = toml::to_string_pretty(self)?;
        std::fs::write(smote_user_list_path, serialized_toml)?;
        Ok(())
    }
}

pub fn initialize_smite() -> anyhow::Result<Smite> {
    Ok(Smite {
        smite_gif_links: initialize_smite_gif_links(),
        smote_user_list: Arc::new(RwLock::new(initialize_smote_user_list()?)),
    })
}

pub async fn schedule_unsmite(ctx: &serenity::prelude::Context, data: &ContextData) {
    let smote_user_list = data.smite.smote_user_list.clone();
    let smote_users = smote_user_list.read().await.smote_users.clone();

    for smote_user in smote_users.into_iter() {
        let context = ctx.clone();
        let smote_user_list = data.smite.smote_user_list.clone();
        tokio::spawn(async move {
            let time_remained = smote_user.due_time - Utc::now();

            if time_remained.num_seconds() < 0 {
                if let Err(e) = remove_smote_user(context, smote_user_list, smote_user).await {
                    tracing::error!("Error occurred when removing smote user: {}", e);
                }
            } else {
                let std_duration = time_remained
                    .to_std()
                    .expect("Failed to cast chrono duration to std duration.");
                tokio::time::sleep(std_duration).await;
                if let Err(e) = remove_smote_user(context, smote_user_list, smote_user).await {
                    tracing::error!("Error occurred when removing smote user: {}", e);
                }
            }
        });
    }
}

fn initialize_smite_gif_links() -> Vec<String> {
    let smite_gif_links_path = String::from(ASSET_DIRECTORY) + SMITE_GIF_LINKS_FILE_NAME;
    let json = std::fs::read(smite_gif_links_path)
        .expect("Failed to read smite gif links from local disk.");
    serde_json::from_slice(&json).expect("Failed to deserialize smite gif links.")
}

fn initialize_smote_user_list() -> anyhow::Result<SmoteUserList> {
    if !std::path::Path::new(CONFIG_DIRECTORY).exists() {
        std::fs::create_dir(CONFIG_DIRECTORY)?;
    }

    let smote_user_list_path = String::from(CONFIG_DIRECTORY) + SMOTE_USER_LIST_FILE_NAME;
    if !std::path::Path::new(&smote_user_list_path).exists() {
        let new_smote_user_list = SmoteUserList::new();
        new_smote_user_list.write_smote_user_list()?;
        Ok(new_smote_user_list)
    } else {
        let toml = std::fs::read_to_string(&smote_user_list_path)?;
        Ok(toml::from_str(&toml)?)
    }
}

async fn remove_smote_user(
    ctx: serenity::prelude::Context,
    smote_user_list: Arc<RwLock<SmoteUserList>>,
    smote_user: SmoteUser,
) -> anyhow::Result<()> {
    if let Ok(member) = ctx
        .http
        .get_member(
            GuildId::new(smote_user.guild_id),
            UserId::new(smote_user.user_id),
        )
        .await
    {
        let smote_role_ids = vec![TAIGA_SERVER_SMOTE_ROLE_ID, KOU_SERVER_SMOTE_ROLE_ID];
        for role_id in smote_role_ids.into_iter() {
            if !member.roles.contains(&RoleId::new(role_id)) {
                continue;
            }

            match member
                .remove_role(ctx.http.clone(), RoleId::new(role_id))
                .await
            {
                Ok(_) => {
                    let mut smote_users_write_lock = smote_user_list.write().await;
                    let smote_users = smote_users_write_lock.smote_users.clone();
                    let filtered_smote_users = smote_users
                        .into_iter()
                        .filter(|u| u.user_id != smote_user.user_id)
                        .collect::<Vec<_>>();
                    smote_users_write_lock.smote_users = filtered_smote_users;
                    smote_users_write_lock.write_smote_user_list()?;
                    break;
                }
                Err(e) => {
                    tracing::error!("Error when removing smote role from user: {}", e);
                }
            }
        }
    }
    Ok(())
}
