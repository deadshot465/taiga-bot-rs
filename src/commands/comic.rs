use regex::Regex;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::str;
use crate::{SpecializedInfo, validate_text, TextError, SpecializedDialog, get_comic, PersistenceService};
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::shared::structures::dialog::{Comic, Dialog};
use std::sync::Arc;

lazy_static! {
    static ref SPECIAL_DIALOG_REGEX: Regex = Regex::new(r"([a-z]+)\s{1}(\w+)\s{1}(\d{1})\s{1}([\w]+)\s{1}([\w]+)\s{1}(.*)").unwrap();
    static ref DIALOG_REGEX: Regex = Regex::new(r"([a-z]+)\s{1}(\w+)\s{1}([a-z0-9]+)\s{1}(.*)").unwrap();
    static ref DELIMITER_REGEX: Regex = Regex::new(r"[\r\n]").unwrap();
}

#[command]
#[description = "Make a simple and short Camp Buddy comic. Use the same command as dialog or any say, except that all arguments are required. All commands must be written in a plain text file."]
#[usage = ""]
#[example = ""]
#[bucket = "fun"]
pub async fn comic(context: &Context, msg: &Message) -> CommandResult {
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    drop(lock);
    let http = &context.http;
    let mut available_specializations = HashMap::new();
    available_specializations.insert("hirosay", "hiro");
    available_specializations.insert("mhirosay", "hiro");
    available_specializations.insert("taigasay", "taiga");
    available_specializations.insert("keitarosay", "keitaro");
    available_specializations.insert("yoichisay", "yoichi");
    available_specializations.insert("yurisay", "yuri");
    available_specializations.insert("kieransay", "kieran");
    available_specializations.insert("natsumisay", "natsumi");
    available_specializations.insert("huntersay", "hunter");

    if msg.attachments.is_empty() || msg.attachments.len() == 0 {
        msg.channel_id.say(http, "The command has to be called with an attachment.")
            .await?;
        return Ok(());
    }

    let attachment = msg.attachments.first().unwrap();
    msg.channel_id.say(http, format!("Your file name is: {}", &attachment.filename)).await?;
    let file = attachment.download().await?;
    let raw_strings = str::from_utf8(file.borrow()).unwrap();
    let dialog_regex = &*DIALOG_REGEX;
    let special_regex = &*SPECIAL_DIALOG_REGEX;
    let delimiter_regex = &*DELIMITER_REGEX;
    let strings = delimiter_regex.split(raw_strings)
        .into_iter()
        .filter(|s| s.len() > 0 && !s.is_empty())
        .collect::<Vec<&str>>();

    if strings.len() > 5 {
        msg.channel_id.say(http, "The maximum number of images allowed is 5.")
            .await?;
        return Ok(());
    }

    let mut images: Vec<Comic> = vec![];
    for s in strings.iter() {
        if !special_regex.is_match(*s) && !dialog_regex.is_match(*s) {
            msg.channel_id.say(http, "One of the commands is incorrect.")
                .await?;
            return Ok(());
        }

        if special_regex.is_match(*s) {
            let m = special_regex.captures(*s).unwrap();
            let specialization = m.get(1).unwrap().as_str();
            if !available_specializations.contains_key(specialization) {
                msg.channel_id.say(http, "One of the commands contains nonexistent specialization.")
                    .await?;
                return Ok(());
            }

            let persistence_lock = _persistence.read().await;
            let character = available_specializations[specialization];
            let background = m.get(2).unwrap().as_str();
            if !(persistence_lock.dialog_backgrounds.as_ref().unwrap().contains(&background.to_string())) {
                msg.channel_id.say(http, "One of the commands contains a background that is not available.")
                    .await?;
                return Ok(());
            }

            let pose = m.get(3).unwrap().as_str();
            let character_available_options: &SpecializedInfo;
            let options = persistence_lock.specialized_info.as_ref().unwrap();
            character_available_options = &options[character];
            if !character_available_options.poses.contains_key(&pose.to_string()) {
                msg.channel_id.say(http, "One of the commands contains an invalid pose.")
                    .await?;
                return Ok(());
            }

            let cloth = m.get(4).unwrap().as_str();
            if !character_available_options.poses[pose].clothes.contains(&cloth.to_string()) {
                msg.channel_id.say(http, "One of the commands contains an invalid cloth.")
                    .await?;
                return Ok(());
            }

            let face = m.get(5).unwrap().as_str();
            if !character_available_options.poses[pose].faces.contains(&face.to_string()) {
                msg.channel_id.say(http, "One of the commands contains an invalid face.")
                    .await?;
                return Ok(());
            }

            let text = m.get(6).unwrap().as_str();
            let text_validation = validate_text(text);
            if !text_validation.0 {
                match &text_validation.1 {
                    TextError::NoMessage => {
                        msg.channel_id.say(http, "One of the commands doesn't have any content.")
                            .await?;
                        return Ok(());
                    },
                    TextError::LengthTooLong => {
                        msg.channel_id.say(http, "The maximum length of text is 180 for English/ASCII characters, 78 for Japanese characters.")
                            .await?;
                        return Ok(());
                    },
                    TextError::WrongCharacterSet => {
                        msg.channel_id.say(http, "This command cannot be used with non-English or non-Japanese text.")
                            .await?;
                        return Ok(());
                    },
                    _ => ()
                }
            }
            drop(persistence_lock);

            let request_data = SpecializedDialog {
                background: background.trim().to_lowercase(),
                character: Some(character.to_string()),
                clothes: cloth.trim().to_lowercase(),
                face: face.trim().to_lowercase(),
                is_hidden_character: if specialization == "mhirosay" {
                    true
                } else {
                    false
                },
                pose: pose.parse::<u8>().unwrap(),
                text: text.to_string()
            };

            images.push(Comic::SpecializedDialog(request_data));
        }
        else if dialog_regex.is_match(*s) {
            let m = dialog_regex.captures(*s).unwrap();
            let cmd = m.get(1).unwrap().as_str();
            if cmd.trim().to_lowercase() != "dialog" {
                msg.channel_id.say(http, "One of the commands is incorrect.")
                    .await?;
                return Ok(());
            }

            let persistence_lock = _persistence.read().await;
            let background = m.get(2).unwrap().as_str();
            if !(persistence_lock.dialog_backgrounds.as_ref().unwrap().contains(&background.to_string())) {
                msg.channel_id.say(http, "One of the commands contains a background that is not available.")
                    .await?;
                return Ok(());
            }

            let character = m.get(3).unwrap().as_str();
            if !(persistence_lock.dialog_characters.as_ref().unwrap().contains(&character.to_string())) {
                msg.channel_id.say(http, "One of the dialog commands contains an invalid character.")
                    .await?;
                return Ok(());
            }

            let text = m.get(4).unwrap().as_str();
            let text_validation = validate_text(text);
            if !text_validation.0 {
                match &text_validation.1 {
                    TextError::NoMessage => {
                        msg.channel_id.say(http, "One of the commands doesn't have any content.")
                            .await?;
                        return Ok(());
                    },
                    TextError::LengthTooLong => {
                        msg.channel_id.say(http, "The maximum length of text is 180 for English/ASCII characters, 78 for Japanese characters.")
                            .await?;
                        return Ok(());
                    },
                    TextError::WrongCharacterSet => {
                        msg.channel_id.say(http, "This command cannot be used with non-English or non-Japanese text.")
                            .await?;
                        return Ok(());
                    },
                    _ => ()
                }
            }
            drop(persistence_lock);
            images.push(Comic::Dialog(Dialog {
                background: background.trim().to_lowercase(),
                character: character.trim().to_lowercase(),
                text: text.trim().to_lowercase()
            }));
        }
        else {
            msg.channel_id.say(http, "One of the commands is incorrect.")
                .await?;
            return Ok(());
        }
    }

    let bytes = get_comic(images, context).await.unwrap();
    let files: Vec<(&[u8], &str)> = vec![(bytes.borrow(), "result.png")];
    msg.channel_id.send_files(&context.http, files, |m| m.content("Here you go~")).await?;

    Ok(())
}