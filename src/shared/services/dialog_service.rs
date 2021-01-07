/*use crate::protos::discord_bot_service::discord_bot_service_client::DiscordBotServiceClient;
use crate::protos::discord_bot_service::{DialogReply, DialogRequest, SpecializedDialogRequest};
use crate::protos::jwt_token_service::access_reply::User;
use crate::protos::jwt_token_service::jwt_token_service_client::JwtTokenServiceClient;
use crate::protos::jwt_token_service::AccessRequest;*/
use crate::shared::structures::dialog::Comic;
use crate::{AuthenticationService, SpecializedDialog};
use chrono::{DateTime, TimeZone, Utc};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serenity::client::Context;
use std::collections::HashMap;
use std::env;
use tokio::sync::Mutex;
//use tonic::{Response, Streaming};

/*static mut DISCORD_SERVICE_CLIENT: OnceCell<DiscordBotServiceClient<tonic::transport::Channel>> =
    OnceCell::new();
static DISCORD_SERVICE_CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();
static mut JWT_TOKEN_CLIENT: OnceCell<JwtTokenServiceClient<tonic::transport::Channel>> =
    OnceCell::new();*/
static REST_CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

// Via REST API
static mut REST_CLIENT: OnceCell<reqwest::Client> = OnceCell::new();

/*struct JwtToken {
    pub token: String,
    pub user_details: Option<User>,
    pub expiry: String,
}*/

#[derive(Deserialize, Serialize)]
pub struct JwtToken {
    pub token: String,
    #[serde(rename = "userDetails")]
    pub user_details: UserDetails,
    pub expiry: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserDetails {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userRole")]
    pub user_role: String,
    #[serde(rename = "type")]
    pub user_type: u8,
}

static mut JWT_TOKEN: OnceCell<JwtToken> = OnceCell::new();

pub async fn get_dialog(background: &str, character: &str, text: &str) -> anyhow::Result<Vec<u8>> {
    initialize_clients().await?;
    update_token().await?;

    let token = unsafe { JWT_TOKEN.get().expect("Failed to get JWT token.") };
    let client = unsafe { REST_CLIENT.get().expect("Failed to get REST client.") };

    let mut request_data = HashMap::new();
    request_data.insert("Background", background);
    request_data.insert("Character", character);
    request_data.insert("Text", text);

    let response = client
        .post("https://tetsukizone.com/api/dialog")
        .json(&request_data)
        .header("Content-Type", "application/json")
        .bearer_auth(token.token.clone())
        .send()
        .await?;

    match response.bytes().await {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(_) => Ok(vec![]),
    }
}

/*pub async fn get_dialog(
    background: &str,
    character: &str,
    text: &str,
    _context: &Context,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    initialize_clients().await?;
    update_token().await?;

    let token = unsafe { JWT_TOKEN.get().expect("Failed to get JWT token.") };

    let discord_client = unsafe {
        DISCORD_SERVICE_CLIENT
            .get_mut()
            .expect("Failed to get gRPC client for Discord bot.")
    };

    let request = tonic::Request::new(DialogRequest {
        background: background.into(),
        character: character.into(),
        text: text.into(),
        jwt_token: token.token.clone(),
    });
    let response: Response<Streaming<DialogReply>> = discord_client.post_dialog(request).await?;
    let mut response: Streaming<DialogReply> = response.into_inner();
    if let Some(message) = response.message().await? {
        if message.status {
            Ok(message.image)
        } else {
            Ok(vec![])
        }
    } else {
        Ok(vec![])
    }
}*/

pub async fn get_specialized_dialog(mut dialog: SpecializedDialog) -> anyhow::Result<Vec<u8>> {
    initialize_clients().await?;
    update_token().await?;

    let token = unsafe { JWT_TOKEN.get().expect("Failed to get JWT token.") };
    let client = unsafe { REST_CLIENT.get().expect("Failed to get REST client.") };

    let character = dialog
        .character
        .take()
        .expect("Failed to get character from dialog input.");
    let response = client
        .post(&format!("https://tetsukizone.com/api/dialog/{}", character))
        .json(&dialog)
        .header("Content-Type", "application/json")
        .bearer_auth(token.token.clone())
        .send()
        .await?;

    match response.bytes().await {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(_) => Ok(vec![]),
    }
}

/*pub async fn get_specialized_dialog(
    dialog: SpecializedDialog,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    initialize_clients().await?;
    update_token().await?;

    let token = unsafe { JWT_TOKEN.get().expect("Failed to get JWT token.") };

    let discord_client = unsafe {
        DISCORD_SERVICE_CLIENT
            .get_mut()
            .expect("Failed to get gRPC client for Discord bot.")
    };

    let request = tonic::Request::new(SpecializedDialogRequest {
        background: dialog.background,
        character: dialog
            .character
            .expect("Failed to get character information from input."),
        pose: dialog.pose as i32,
        clothes: dialog.clothes,
        face: dialog.face,
        is_hidden_character: dialog.is_hidden_character,
        text: dialog.text,
        jwt_token: token.token.clone(),
    });

    let response = discord_client.post_specialized_dialog(request).await?;
    let mut response = response.into_inner();
    if let Some(message) = response.message().await? {
        if message.status {
            Ok(message.image)
        } else {
            Ok(vec![])
        }
    } else {
        Ok(vec![])
    }
}*/

pub async fn get_comic(
    comic_data: Vec<Comic>,
    context: &Context,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut dummy = HashMap::new();
    dummy.insert("Hello", "world");

    let data = context.data.read().await;
    let authentication = data.get::<AuthenticationService>().unwrap();
    let mut authentication_lock = authentication.lock().await;
    authentication_lock.login().await.unwrap();

    let response = client
        .post("https://tetsukizone.com/api/comic")
        .json(&comic_data)
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("Bearer {}", authentication_lock.token.as_str()),
        )
        .send()
        .await
        .unwrap();
    drop(authentication_lock);
    drop(data);

    let bytes = response.bytes().await;
    if let Ok(res) = bytes {
        Ok(res.to_vec())
    } else {
        Ok(vec![])
    }
}

async fn initialize_clients() -> anyhow::Result<()> {
    unsafe {
        if REST_CLIENT.get().is_none() {
            let client_initialized = REST_CLIENT_INITIALIZED.get_or_init(|| Mutex::new(false));
            let mut initialized = client_initialized.lock().await;
            if !*initialized {
                let client = reqwest::Client::new();
                if let Ok(_) = REST_CLIENT.set(client) {
                    *initialized = true;
                }
            }
        }
    }
    Ok(())
}

/*async fn initialize_clients() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if JWT_TOKEN_CLIENT.get_mut().is_none() {
            let client_initialized = JWT_TOKEN_CLIENT_INITIALIZED.get_or_init(|| Mutex::new(false));
            let mut initialized = client_initialized.lock().await;
            if !*initialized {
                let client = JwtTokenServiceClient::connect("http://64.227.99.31:26361").await?;
                if let Ok(_) = JWT_TOKEN_CLIENT.set(client) {
                    *initialized = true;
                }
            }
        }

        if DISCORD_SERVICE_CLIENT.get_mut().is_none() {
            let client_initialized =
                DISCORD_SERVICE_CLIENT_INITIALIZED.get_or_init(|| Mutex::new(false));
            let mut initialized = client_initialized.lock().await;
            if !*initialized {
                let client = DiscordBotServiceClient::connect("http://64.227.99.31:26361").await?;
                if let Ok(_) = DISCORD_SERVICE_CLIENT.set(client) {
                    *initialized = true;
                }
            }
        }
    }

    Ok(())
}*/

async fn update_token() -> anyhow::Result<()> {
    let token = unsafe { JWT_TOKEN.get_mut() };
    let client = unsafe {
        REST_CLIENT
            .get()
            .expect("Failed to get REST client for JWT token.")
    };

    if let Some(t) = token {
        let expiry = t.expiry.parse::<DateTime<Utc>>()?;
        if expiry < Utc::now() {
            let mut request_data = HashMap::new();
            request_data.insert("UserName", env::var("LOGIN_NAME")?);
            request_data.insert("Password", env::var("LOGIN_PASS")?);
            let request = client
                .post("https://tetsukizone.com/api/login")
                .json(&request_data)
                .send()
                .await?;

            let response: JwtToken = request.json().await?;
            *t = response;
        }
    } else {
        let mut request_data = HashMap::new();
        request_data.insert("UserName", env::var("LOGIN_NAME")?);
        request_data.insert("Password", env::var("LOGIN_PASS")?);
        let request = client
            .post("https://tetsukizone.com/api/login")
            .json(&request_data)
            .send()
            .await?;

        let response: JwtToken = request.json().await?;
        let jwt_token_set_result = unsafe { JWT_TOKEN.set(response) };
        if jwt_token_set_result.is_err() {
            panic!("Failed to set OnceCell for JWT token.");
        }
    }

    Ok(())
}

/*async fn update_token() -> Result<(), Box<dyn std::error::Error>> {
    let token = unsafe { JWT_TOKEN.get_mut() };

    let client = unsafe {
        JWT_TOKEN_CLIENT
            .get_mut()
            .expect("Failed to get gRPC client for JWT token.")
    };

    if let Some(t) = token {
        let expiry = Utc.datetime_from_str(&t.expiry, "%m/%d/%Y %H:%M:%S")?;
        if expiry < Utc::now() {
            let request = tonic::Request::new(AccessRequest {
                user_name: env::var("LOGIN_NAME")?,
                password: env::var("LOGIN_PASS")?,
            });
            let response = client.access(request).await?;
            let response = response.into_inner();
            *t = JwtToken {
                token: response.token,
                user_details: response.user_details,
                expiry: response.expiry,
            };
        }
    } else {
        let request = tonic::Request::new(AccessRequest {
            user_name: env::var("LOGIN_NAME")?,
            password: env::var("LOGIN_PASS")?,
        });
        let response = client.access(request).await?;
        let response = response.into_inner();
        let token_set_result = unsafe {
            JWT_TOKEN.set(JwtToken {
                token: response.token,
                user_details: response.user_details,
                expiry: response.expiry,
            })
        };
        if token_set_result.is_err() {
            panic!("Failed to set OnceCell for JWT token.");
        }
    }

    Ok(())
}*/
