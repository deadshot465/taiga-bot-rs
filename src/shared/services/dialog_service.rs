use crate::protos::discord_bot_service::discord_bot_service_client::DiscordBotServiceClient;
use crate::protos::discord_bot_service::{DialogReply, DialogRequest, SpecializedDialogRequest};
use crate::protos::jwt_token_service::access_reply::User;
use crate::protos::jwt_token_service::jwt_token_service_client::JwtTokenServiceClient;
use crate::protos::jwt_token_service::AccessRequest;
use crate::shared::structures::dialog::Comic;
use crate::{AuthenticationService, SpecializedDialog};
use chrono::{TimeZone, Utc};
use once_cell::sync::OnceCell;
use serenity::client::Context;
use std::collections::HashMap;
use std::env;
use tokio::sync::Mutex;
use tonic::{Response, Streaming};

static mut DISCORD_SERVICE_CLIENT: OnceCell<DiscordBotServiceClient<tonic::transport::Channel>> =
    OnceCell::new();
static DISCORD_SERVICE_CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();
static mut JWT_TOKEN_CLIENT: OnceCell<JwtTokenServiceClient<tonic::transport::Channel>> =
    OnceCell::new();
static JWT_TOKEN_CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

struct JwtToken {
    pub token: String,
    pub user_details: Option<User>,
    pub expiry: String,
}

static mut JWT_TOKEN: OnceCell<JwtToken> = OnceCell::new();

pub async fn get_dialog(
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
}

pub async fn get_specialized_dialog(
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
}

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

async fn initialize_clients() -> Result<(), Box<dyn std::error::Error>> {
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
}

async fn update_token() -> Result<(), Box<dyn std::error::Error>> {
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
        unsafe {
            JWT_TOKEN.set(JwtToken {
                token: response.token,
                user_details: response.user_details,
                expiry: response.expiry,
            })?;
        }
    }

    Ok(())
}
