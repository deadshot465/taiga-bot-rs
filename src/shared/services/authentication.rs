use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static mut AUTHENTICATION_SERVICE: Authentication = Authentication{
    token: String::new(),
    user_details: None,
    expiry: None
};

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
    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            if AUTHENTICATION_SERVICE.expiry.is_some() {
                let expiry_date = AUTHENTICATION_SERVICE
                    .expiry
                    .as_ref()
                    .unwrap()
                    .parse::<DateTime<Utc>>()
                    .unwrap();
                if expiry_date > Utc::now() {
                    return Ok(());
                }
            }
        }

        let mut request_data: HashMap<&str, &str> = HashMap::new();
        request_data.insert("UserName", dotenv!("LOGIN_NAME"));
        request_data.insert("Password", dotenv!("LOGIN_PASS"));

        let client = reqwest::Client::new();
        let response = client.post("https://tetsukizone.com/api/login")
            .json(&request_data)
            .send()
            .await?;

        let resp: Authentication = response.json().await?;

        unsafe {
            AUTHENTICATION_SERVICE = resp;
        }

        Ok(())
    }
}