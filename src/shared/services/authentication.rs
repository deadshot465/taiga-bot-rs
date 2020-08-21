use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AuthenticationService;
impl TypeMapKey for AuthenticationService {
    type Value = Arc<Mutex<Authentication>>;
}

#[derive(Deserialize, Serialize)]
pub struct Authentication {
    pub token: String,
    #[serde(rename = "userDetails")]
    pub user_details: Option<UserDetails>,
    pub expiry: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct UserDetails {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userRole")]
    pub user_role: String,
    #[serde(rename = "type")]
    pub user_type: u8
}

impl Authentication {
    pub async fn new() -> Self {
        let mut entity = Authentication {
            token: String::new(),
            user_details: None,
            expiry: None
        };
        entity.login().await.expect("Failed to authenticate with the server.");
        entity
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.expiry.is_some() {
            let expiry_date = self
                .expiry
                .as_ref()
                .unwrap()
                .parse::<DateTime<Utc>>()
                .unwrap();
            if expiry_date > Utc::now() {
                return Ok(());
            }
        }

        let mut request_data: HashMap<&str, String> = HashMap::new();
        request_data.insert("UserName", env::var("LOGIN_NAME").unwrap());
        request_data.insert("Password", env::var("LOGIN_PASS").unwrap());

        let client = reqwest::Client::new();
        let response = client.post("https://tetsukizone.com/api/login")
            .json(&request_data)
            .send()
            .await?;

        let resp: Authentication = response.json().await?;
        *self = resp;
        Ok(())
    }
}