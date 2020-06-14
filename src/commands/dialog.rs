use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use crate::PERSISTENCE_STORAGE;
use crate::shared::validate_dialog;

const LENGTH_TOO_SHORT_MSG: &'static str = "This command requires two arguments: `dialog [background] <character> <text>` ([] is optional)";

#[command]
pub async fn dialog(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() || args.len() < 2 {
        msg.channel_id
            .say(&context.http, LENGTH_TOO_SHORT_MSG).await?;
        return Ok(());
    }

    let mut first_arg = args.single::<String>().unwrap();
    let mut background = String::new();
    let mut character = String::new();
    unsafe {
        let ref characters = PERSISTENCE_STORAGE.get_instance().await.dialog_characters;
        let ref backgrounds = PERSISTENCE_STORAGE.get_instance().await.dialog_backgrounds;
        if characters.contains(&first_arg) {
            let mut rng = thread_rng();
            background = backgrounds[rng.gen_range(0, backgrounds.len())].clone();
            character = first_arg;
        }
        else {
            background = first_arg;
            character = args.single::<String>().unwrap();
        }
    }
    let text = args.single::<String>().unwrap();
    let validation_result = validate_dialog(context, msg, &background, &character, &text);
    if let Err(e) = validation_result.await {
        eprintln!("An error occurred when validating the dialog: {}", e);
        return Ok(());
    }

    let mut request_data = HashMap::new();
    request_data.insert("Background", background.as_str());
    request_data.insert("Character", character.as_str());
    request_data.insert("Text", text.as_str());

    let client = reqwest::Client::new();
    let response = client.post("https://tetsukizone.com/api/dialog")
        .json(&request_data)
        .send()
        .await?;

    Ok(())
}