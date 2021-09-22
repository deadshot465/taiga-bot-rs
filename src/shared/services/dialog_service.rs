#![allow(clippy::ptr_arg)]
use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::authentication::AUTHENTICATION;
use crate::shared::structs::config::configuration::CONFIGURATION;
use crate::shared::structs::fun::dialog::Dialog;
use once_cell::sync::Lazy;
use rand::prelude::*;
use regex::Regex;
use std::collections::HashMap;

const DIALOG_PATH: &str = "/dialog";
const DIALOG_TEXT_LIMIT: usize = 180;

static EMOTE_MENTIONS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<(?:[^\d>]+|:[A-Za-z0-9]+:)\w+>").expect("Failed to build emote mentions regex.")
});
static NON_ASCII_AND_JAPANESE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[^\x00-\x7F\u4e00-\u9fbf\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u3000-\u303f\u2018-\u2019]")
        .expect("Failed to build non-ASCII and Japanese regex.")
});

pub async fn get_dialog(
    background: String,
    character: String,
    text: String,
) -> anyhow::Result<Vec<u8>> {
    let dialog = Dialog {
        background,
        character,
        text,
    };

    let server_endpoint = CONFIGURATION
        .get()
        .map(|c| c.server_endpoint.as_str())
        .unwrap_or_default();

    let dialog_path = format!("{}{}", server_endpoint, DIALOG_PATH);
    let token = AUTHENTICATION.read().await.token.clone();
    let response = HTTP_CLIENT
        .post(&dialog_path)
        .json(&dialog)
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .send()
        .await?;

    match response.bytes().await {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(e) => {
            log::error!("Error when getting bytes data of dialog.");
            Err(anyhow::anyhow!("{}", e))
        }
    }
}

/*pub async fn get_specialized_dialog(mut dialog: SpecializedDialog) -> anyhow::Result<Vec<u8>> {
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
}*/

pub async fn validate_dialog(
    background: &mut String,
    character: &String,
    text: &String,
    is_kou: bool,
) -> anyhow::Result<()> {
    let server_endpoint = CONFIGURATION
        .get()
        .map(|c| &c.server_endpoint)
        .expect("Failed to get server endpoint from configuration.");

    let dialog_path = format!("{}{}", server_endpoint, DIALOG_PATH);
    let dialog_options: HashMap<String, Vec<String>> =
        HTTP_CLIENT.get(&dialog_path).send().await?.json().await?;

    if let Some(backgrounds) = dialog_options.get("backgrounds") {
        if !backgrounds.contains(background) {
            *background = {
                let mut rng = rand::thread_rng();
                backgrounds.choose(&mut rng).cloned().unwrap_or_default()
            }
        }
    }

    if let Some(characters) = dialog_options.get("characters") {
        if !characters.contains(character) {
            let characters_text: String = characters
                .iter()
                .map(|s| format!("`{}`", s))
                .collect::<Vec<_>>()
                .join(", ");

            return Err(anyhow::anyhow!(format!(
                "Sorry, but I don't think that `{}` is a supported character.\n\
                Available characters are: {}.",
                character, characters_text
            )));
        }
    }

    if text.is_empty() {
        return Err(anyhow::anyhow!(if is_kou {
            "Uh...I don't know what to send if you don't tell me anything..."
        } else {
            "At least give me something to send, you dumbass."
        }));
    }

    if text.chars().count() > DIALOG_TEXT_LIMIT {
        return Err(anyhow::anyhow!(
            "Sorry, the message limit is 180 characters!"
        ));
    }

    if EMOTE_MENTIONS_REGEX.is_match(text) || NON_ASCII_AND_JAPANESE_REGEX.is_match(text) {
        return Err(anyhow::anyhow!(if is_kou {
            "I can't do emotes, mentions, non-latin and non-Japanese characters."
        } else {
            "I don't do emotes, mentions, non-latin and non-Japanese characters."
        }));
    }

    Ok(())
}
