use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::{CommandStrings, SpecializedInfo};
use crate::{INTERFACE_SERVICE, PERSISTENCE_STORAGE, TextError, validate_text, AUTHENTICATION_SERVICE, SpecializedDialog};
use std::collections::HashMap;
use std::time::Duration;
use rand::{thread_rng, Rng};
use std::borrow::Borrow;

#[command]
#[aliases("hiro")]
#[description = "Returns an image of Hiro saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn hirosay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "hiro", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("mhiro", "maturehiro", "maturehirosay")]
#[description = "Returns an image of mature Hiro saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn mhirosay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "hiro", true).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("taiga")]
#[description = "Returns an image of Taiga saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn taigasay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "taiga", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("keitaro")]
#[description = "Returns an image of Keitaro saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn keitarosay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "keitaro", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("yoichi")]
#[description = "Returns an image of Yoichi saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn yoichisay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "yoichi", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("yuri")]
#[description = "Returns an image of Yuri saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn yurisay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "yuri", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("kieran")]
#[description = "Returns an image of Kieran saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn kieransay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "kieran", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

#[command]
#[aliases("natsumi")]
#[description = "Returns an image of Natsumi saying anything you want."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "say"]
pub async fn natsumisay(context: &Context, msg: &Message) -> CommandResult {
    let result = say(context, msg, "natsumi", false).await?;
    if result.len() > 0 {
        let file: Vec<(&[u8], &str)> = vec![(result.borrow(), "result.png")];
        msg.channel_id.send_files(&context.http, file, |f| f.content("Here you go~"))
            .await?;
    }
    Ok(())
}

async fn say(context: &Context, msg: &Message, character: &str, is_hidden: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.say[character];
    }

    let backgrounds: &Vec<String>;
    unsafe {
        backgrounds = PERSISTENCE_STORAGE.dialog_backgrounds.as_ref().unwrap();
    }
    msg.reply(&context.http, "Please specify a background in 10 seconds, or specify nothing or anything to use a random background.")
        .await?;
    let background: String;
    if let Some(bg) = &msg.author
        .await_reply(&context)
        .timeout(Duration::from_secs(10)).await {
        let lower_case = bg.content.to_lowercase();
        if backgrounds.contains(&lower_case) {
            background = lower_case;
        }
        else {
            background = backgrounds[thread_rng().gen_range(0, backgrounds.len())].clone();
        }
    }
    else {
        background = backgrounds[thread_rng().gen_range(0, backgrounds.len())].clone();
    }

    let character_available_options: &SpecializedInfo;
    unsafe {
        let options = PERSISTENCE_STORAGE.specialized_info.as_ref().unwrap();
        character_available_options = &options[character];
    }

    msg.reply(&context.http, "Please specify a pose number in 10 seconds.")
        .await?;
    let mut pose = String::new();
    if let Some(p) = &msg.author.await_reply(&context)
        .timeout(Duration::from_secs(10)).await {
        if character_available_options.poses.contains_key(&p.content) {
            pose = p.content.clone();
        }
        else {
            let error_msg = interface_string.errors["pose_not_exist"].as_str();
            msg.reply(&context.http, error_msg).await?;
            return Ok(vec![]);
        }
    }

    msg.reply(&context.http, "Please specify a cloth in 10 seconds.")
        .await?;
    let mut cloth = String::new();
    if let Some(c) = &msg.author.await_reply(&context)
        .timeout(Duration::from_secs(10)).await {
        if character_available_options.poses[&pose].clothes.contains(&c.content) {
            cloth = c.content.clone();
        }
        else {
            let error_msg = interface_string.errors["clothes_not_exist"].as_str();
            msg.reply(&context.http, error_msg).await?;
            return Ok(vec![]);
        }
    }

    msg.reply(&context.http, "Please specify a face/expression in 10 seconds.")
        .await?;
    let mut face = String::new();
    if let Some(e) = &msg.author.await_reply(&context)
        .timeout(Duration::from_secs(10)).await {
        if character_available_options.poses[&pose].faces.contains(&e.content) {
            face = e.content.clone();
        }
        else {
            let error_msg = interface_string.errors["face_not_exist"].as_str();
            msg.reply(&context.http, error_msg).await?;
            return Ok(vec![]);
        }
    }

    msg.reply(&context.http, "Please specify the message in 30 seconds.")
        .await?;
    let mut text = String::new();
    if let Some(t) = &msg.author.await_reply(&context)
        .timeout(Duration::from_secs(30)).await {
        let text_validation = validate_text(t.content.as_str());
        if !text_validation.0 {
            match &text_validation.1 {
                TextError::NoMessage => {
                    let error_msg = interface_string.errors["no_message"].as_str();
                    msg.reply(&context.http, error_msg).await?;
                    return Ok(vec![]);
                },
                TextError::LengthTooLong => {
                    let error_msg = interface_string.errors["message_too_long"].as_str();
                    msg.reply(&context.http, error_msg).await?;
                    return Ok(vec![]);
                },
                TextError::WrongCharacterSet => {
                    let error_msg = interface_string.errors["wrong_character_set"].as_str();
                    msg.reply(&context.http, error_msg).await?;
                    return Ok(vec![]);
                }
                _ => ()
            }
        }
        else {
            text = t.content.clone();
            let client = reqwest::Client::new();
            unsafe {
                AUTHENTICATION_SERVICE.login().await.unwrap();
                let request_data = SpecializedDialog {
                    background,
                    clothes: cloth,
                    face,
                    is_hidden_character: is_hidden,
                    pose: pose.as_str().parse::<u8>().unwrap(),
                    text
                };

                let response = client.post(format!("https://tetsukizone.com/api/dialog/{}", character).as_str())
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", AUTHENTICATION_SERVICE.token.as_str()))
                    .json(&request_data)
                    .send()
                    .await?;

                let data = response.bytes().await?;
                return Ok(data.to_vec());
            }
        }
    }
    else {
        let error_msg = interface_string.errors["no_message"].as_str();
        msg.reply(&context.http, error_msg).await?;
        return Ok(vec![]);
    }
    Ok(vec![])
}