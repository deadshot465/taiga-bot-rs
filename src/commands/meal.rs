use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;

#[derive(Deserialize, Serialize)]
struct Meal {

}

#[command]
pub async fn meal(context: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let response = client.get("http://www.themealdb.com/api/json/v1/1/random.php")
        .send()
        .await?
        .json()
        .await?;
    let color = u32::from_str_radix("fd9b3b", 16).unwrap();

    Ok(())
}