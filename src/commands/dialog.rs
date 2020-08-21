use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::{get_dialog, InterfaceService, PersistenceService};
use crate::shared::validate_dialog;
use std::borrow::Borrow;
use std::sync::Arc;

#[command]
#[description = "Returns an image of a character in Camp Buddy saying anything you want."]
#[usage = "<background> <character> <message> or dialog <character> <message>"]
#[example = "hiro Welcome to Camp Buddy!"]
#[bucket = "fun"]
pub async fn dialog(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let lock = context.data.read().await;
    let interface = lock.get::<InterfaceService>().unwrap();
    let persistence = lock.get::<PersistenceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    let _interface = Arc::clone(interface);
    drop(lock);
    let interface_lock = _interface.read().await;
    let interface_strings = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface_strings.dialog;

    if args.is_empty() || args.len() < 2 {
        msg.channel_id
            .say(&context.http, interface_string.errors["length_too_short"].as_str()).await?;
        return Ok(());
    }
    let first_arg = args.single::<String>().unwrap();
    let background: String;
    let character: String;
    let persistence_lock = _persistence.read().await;
    let characters = persistence_lock.dialog_characters.as_ref().unwrap();
    let backgrounds = persistence_lock.dialog_backgrounds.as_ref().unwrap();
    if characters.contains(&first_arg) {
        let mut rng = thread_rng();
        background = backgrounds.choose(&mut rng).unwrap().clone();
        character = first_arg;
    }
    else {
        background = first_arg;
        character = args.single::<String>().unwrap();
    }
    drop(persistence_lock);
    let text = args.remains();
    if text.is_none() {
        let error_msg = interface_string.errors["no_message"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }
    let text_content = String::from(text.unwrap());
    let validation_result = validate_dialog(context, msg, &background, &character, &text_content, &interface_lock);
    if let Err(e) = validation_result.await {
        eprintln!("An error occurred when validating the dialog: {}", e);
        return Ok(());
    }
    let bytes = get_dialog(background.as_str(), character.as_str(), text_content.as_str(), context)
        .await
        .unwrap();
    let files: Vec<(&[u8], &str)> = vec![(bytes.borrow(), "result.png")];
    msg.channel_id.send_files(&context.http, files, |m| m.content("Here you go~")).await?;
    drop(interface_lock);
    Ok(())
}