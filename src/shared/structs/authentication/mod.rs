use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::shared::structs::ContextData;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub(self) async fn login(&mut self, data: &ContextData) -> anyhow::Result<()> {
        if let Some(expiry) = self.expiry {
            if expiry < Utc::now() {
                self.inner_login(data).await?;
            }
        } else {
            self.inner_login(data).await?;
        }
        Ok(())
    }

    async fn inner_login(&mut self, data: &ContextData) -> anyhow::Result<()> {
        let (request_data, server_endpoint) = {
            let configuration = &data.config;

            let mut data = HashMap::new();
            data.insert("user_name".to_string(), configuration.login_name.clone());
            data.insert("password".to_string(), configuration.login_pass.clone());
            (data, configuration.server_endpoint.clone())
        };

        let login_path = server_endpoint + LOGIN_PATH;
        let response = data
            .http_client
            .post(&login_path)
            .json(&request_data)
            .send()
            .await?;

        let mut response: HashMap<String, String> = response.json().await?;
        self.token = response
            .remove("token")
            .expect("Failed to get authentication token.");
        self.expiry = response
            .remove("expiry")
            .and_then(|s| s.parse::<DateTime<Utc>>().ok());
        Ok(())
    }
}

impl Default for Authentication {
    fn default() -> Self {
        Authentication::new()
    }
}

pub async fn login(data: &ContextData) -> anyhow::Result<()> {
    let auth = data.authentication.clone();
    let mut authentication_write_lock = auth.write().await;
    authentication_write_lock.login(data).await
}
