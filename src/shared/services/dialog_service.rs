use std::collections::HashMap;
use crate::AuthenticationService;
use crate::shared::structures::dialog::Comic;
use serenity::client::Context;

pub async fn get_dialog(background: &str, character: &str, text: &str, context: &Context) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut request_data = HashMap::new();
    request_data.insert("Background", background);
    request_data.insert("Character", character);
    request_data.insert("Text", text);

    let client = reqwest::Client::new();
    let data = context.data.read().await;
    let authentication = data.get::<AuthenticationService>().unwrap();
    let mut authentication_lock = authentication.lock().await;
    authentication_lock.login().await.unwrap();
    authentication_lock.login().await.unwrap();
    let response = client.post("https://tetsukizone.com/api/dialog")
        .json(&request_data)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", authentication_lock.token.as_str()))
        .send()
        .await
        .unwrap();
    drop(authentication_lock);
    drop(data);

    let bytes = response.bytes().await;
    if let Ok(res) = bytes {
        Ok(res.to_vec())
    }
    else {
        Ok(vec![])
    }
}

pub async fn get_comic(comic_data: Vec<Comic>, context: &Context) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut dummy = HashMap::new();
    dummy.insert("Hello", "world");

    let data = context.data.read().await;
    let authentication = data.get::<AuthenticationService>().unwrap();
    let mut authentication_lock = authentication.lock().await;
    authentication_lock.login().await.unwrap();

    let response = client.post("https://tetsukizone.com/api/comic")
        .json(&comic_data)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", authentication_lock.token.as_str()))
        .send()
        .await
        .unwrap();
    drop(authentication_lock);
    drop(data);

    let bytes = response.bytes().await;
    if let Ok(res) = bytes {
        Ok(res.to_vec())
    }
    else {
        Ok(vec![])
    }
}