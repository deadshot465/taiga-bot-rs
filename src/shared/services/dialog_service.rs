use std::collections::HashMap;
use crate::AUTHENTICATION_SERVICE;

pub async fn get_dialog(background: &str, character: &str, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut request_data = HashMap::new();
    request_data.insert("Background", background);
    request_data.insert("Character", character);
    request_data.insert("Text", text);

    let client = reqwest::Client::new();
    unsafe {
        AUTHENTICATION_SERVICE.login().await.unwrap();
        let response = client.post("https://tetsukizone.com/api/dialog")
            .json(&request_data)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", AUTHENTICATION_SERVICE.token.as_str()))
            .send()
            .await
            .unwrap();

        let bytes = response.bytes().await;
        if let Ok(res) = bytes {
            Ok(res.to_vec())
        }
        else {
            Ok(vec![])
        }
    }
}