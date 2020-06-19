use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::collections::HashMap;
use crate::{PERSISTENCE_STORAGE, AUTHENTICATION_SERVICE, INTERFACE_SERVICE};
use crate::shared::{validate_dialog, CommandStrings};
use std::borrow::Borrow;

#[command]
pub async fn dialog(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.dialog;
    }

    if args.is_empty() || args.len() < 2 {
        msg.channel_id
            .say(&context.http, interface_string.errors["length_too_short"].as_str()).await?;
        return Ok(());
    }

    let first_arg = args.single::<String>().unwrap();
    let background: String;
    let character: String;
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

    unsafe {
        AUTHENTICATION_SERVICE.login().await.unwrap();
        let response = client.post("https://tetsukizone.com/api/dialog")
            .json(&request_data)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", AUTHENTICATION_SERVICE.token.as_str()))
            .send()
            .await
            .unwrap();

        let bytes: Vec<u8> = response.bytes().await.unwrap().to_vec();
        let files: Vec<(&[u8], &str)> = vec![(bytes.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, files, |m| m.content("Here you go~")).await?;
    }

    Ok(())
}