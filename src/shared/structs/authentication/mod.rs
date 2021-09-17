use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::config::configuration::CONFIGURATION;
use chrono::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub static AUTHENTICATION: Lazy<RwLock<Authentication>> =
    Lazy::new(|| RwLock::new(Authentication::new()));

#[derive(Deserialize, Serialize, Clone)]
pub struct Authentication {
    pub token: String,
    pub expiry: Option<DateTime<Utc>>,
}

const LOGIN_PATH: &str = "/login";

impl Authentication {
    pub fn new() -> Self {
        Authentication {
            token: String::new(),
            expiry: None,
        }
    }

    pub(self) async fn login(&mut self) -> anyhow::Result<()> {
        if let Some(expiry) = self.expiry {
            if expiry < Utc::now() {
                self.inner_login().await?;
            }
        } else {
            self.inner_login().await?;
        }
        Ok(())
    }

    async fn inner_login(&mut self) -> anyhow::Result<()> {
        let (request_data, server_endpoint) = {
            let configuration = CONFIGURATION
                .get()
                .expect("Configuration is not initialized.");

            let mut data = HashMap::new();
            data.insert("user_name".to_string(), configuration.login_name.clone());
            data.insert("password".to_string(), configuration.login_pass.clone());
            (data, configuration.server_endpoint.clone())
        };

        let login_path = server_endpoint + LOGIN_PATH;
        let response = HTTP_CLIENT
            .post(&login_path)
            .json(&request_data)
            .send()
            .await?;

        let mut response: HashMap<String, String> = response.json().await?;
        self.token = response
            .remove(&"token".to_string())
            .expect("Failed to get authentication token.");
        self.expiry = response
            .remove(&"expiry".to_string())
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());
        Ok(())
    }
}

pub async fn login() -> anyhow::Result<()> {
    let mut authentication_write_lock = AUTHENTICATION.write().await;
    authentication_write_lock.login().await
}
