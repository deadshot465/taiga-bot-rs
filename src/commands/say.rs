use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::{CommandStrings, SpecializedInfo};
use crate::{INTERFACE_SERVICE, PERSISTENCE_STORAGE, TextError, validate_text, AUTHENTICATION_SERVICE, SpecializedDialog, AvailableSpecializedOptions};
use std::collections::HashMap;
use std::time::Duration;
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use serenity::utils::Color;

#[command]
#[aliases("hiro")]
#[description = "Returns an image of Hiro saying anything you want."]
#[usage = ""]
#[example = ""]
#[bucket = "say"]
pub async fn hirosay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "hiro").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn mhirosay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "hiro").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn taigasay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "taiga").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn keitarosay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "keitaro").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn yoichisay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "yoichi").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn yurisay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "yuri").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn kieransay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "kieran").await?;
            return Ok(());
        }
    }
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
#[example = ""]
#[bucket = "say"]
pub async fn natsumisay(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<String>();
    if let Ok(s) = arg {
        if s.to_lowercase() == "help" {
            say_help(context, msg, "natsumi").await?;
            return Ok(());
        }
    }
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

async fn say_help(context: &Context, msg: &Message, character: &str) -> CommandResult {
    let character_available_options: &SpecializedInfo;
    let available_backgrounds: &Vec<String>;
    unsafe {
        let options = PERSISTENCE_STORAGE.specialized_info.as_ref().unwrap();
        character_available_options = &options[character];
        available_backgrounds = PERSISTENCE_STORAGE.dialog_backgrounds.as_ref().unwrap();
    }

    let member = msg.member(&context.cache).await.unwrap();
    let color = u32::from_str_radix("ff6600", 16).unwrap();

    unsafe {
        if INTERFACE_SERVICE.is_kou {
            msg.channel_id.say(&context.http, "Check your DM <:KouConfident:705182851754360912>")
                .await?;
        }
        else {
            msg.channel_id.say(&context.http, "Check your DM <:chibitaiga:697893400891883531>")
                .await?;
        }
    }

    msg.author.direct_message(&context.http, |m| m
        .embed(|e| {
            e.author(|a| {
               if let Some(nick) = member.nick.as_ref() {
                   a.name(&nick);
               }
                else {
                    a.name(&msg.author.name);
                }
                if let Some(url) = msg.author.avatar_url() {
                    a.icon_url(&url);
                }
                a
            });
            e.color(Color::from(color));
            e.description(format!("Details usage for `{}`", character));
            let mut background_strings: String = available_backgrounds.iter()
                .map(|s| format!("`{}`, ", s))
                .collect();
            e.field("**Backgrounds**", &background_strings[..background_strings.len() - 2], false);
            e.field("**Total Available Poses (0-indexed)**", character_available_options.poses.len(), false);

            let mut available_options = character_available_options.poses.iter()
                .map(|p| (p.0, p.1))
                .collect::<Vec<(&String, &AvailableSpecializedOptions)>>();
            available_options
                .sort_by(|a, b| a.0.parse::<u8>().unwrap().cmp(&b.0.parse::<u8>().unwrap()));

            for pair in available_options.iter() {
                let cloth_title = format!("**Available Clothes for Pose {}**", pair.0);
                let clothes: String = pair.1.clothes.iter()
                    .map(|s| format!("`{}`, ", s))
                    .collect();
                let face_title = format!("**Available Faces for Pose {}**", pair.0);
                let faces: String = pair.1.faces.iter()
                    .map(|s| format!("`{}`, ", s))
                    .collect();

                e.field(&cloth_title, &clothes[..clothes.len() - 2], false);

                if faces.len() > 1024 {
                    let mut face_msg_list: Vec<&str> = vec![];
                    let mut last_start = 0;
                    let stride = 1000;
                    let mut last_period_index = 0;

                    loop {
                        if last_start + stride > faces.len() {
                            face_msg_list.push(&faces[last_start..]);
                            break;
                        }
                        last_period_index = faces[last_start..last_start + stride]
                            .rfind(',')
                            .unwrap();
                        let string = &faces[last_start..last_period_index];
                        face_msg_list.push(string);
                        last_start = last_period_index + 1;
                    }

                    for s in face_msg_list.iter() {
                        e.field(&face_title, *s, false);
                    }
                }
                else {
                    e.field(&face_title, &faces[..faces.len() - 2], false);
                }
            }
            e
        })).await?;

    Ok(())
}