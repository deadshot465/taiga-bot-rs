use regex::Regex;
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::{PERSISTENCE_STORAGE, INTERFACE_SERVICE};
use crate::shared::CommandStrings;

pub enum TextError {
    NoMessage, LengthTooLong, WrongCharacterSet, None
}

lazy_static! {
    //static ref EMOJI_REGEX: Regex = Regex::new(r"(?:[\u2700-\u27bf]|(?:\ud83c[\udde6-\uddff]){2}|[\ud800-\udbff][\udc00-\udfff]|[\u0023-\u0039]\ufe0f?\u20e3|\u3299|\u3297|\u303d|\u3030|\u24c2|\ud83c[\udd70-\udd71]|\ud83c[\udd7e-\udd7f]|\ud83c\udd8e|\ud83c[\udd91-\udd9a]|\ud83c[\udde6-\uddff]|\ud83c[\ude01-\ude02]|\ud83c\ude1a|\ud83c\ude2f|\ud83c[\ude32-\ude3a]|\ud83c[\ude50-\ude51]|\u203c|\u2049|[\u25aa-\u25ab]|\u25b6|\u25c0|[\u25fb-\u25fe]|\u00a9|\u00ae|\u2122|\u2139|\ud83c\udc04|[\u2600-\u26FF]|\u2b05|\u2b06|\u2b07|\u2b1b|\u2b1c|\u2b50|\u2b55|\u231a|\u231b|\u2328|\u23cf|[\u23e9-\u23f3]|[\u23f8-\u23fa]|\ud83c\udccf|\u2934|\u2935|[\u2190-\u21ff])").unwrap();
    static ref EMOJI_REGEX: Regex = Regex::new(r"(?:[\u2700-\u27bf]|(?:[\u1f1e6-\u1f1ff]){2}|[\u10000-\u10ffff]|[\u0023-\u0039]\ufe0f?\u20e3|\u3299|\u3297|\u303d|\u3030|\u24c2|[\u1f170-\u1f171]|[\u1f17e-\u1f17f]|\u1f18e|[\u1f191-\u1f19a]|[\u1f1e6-\u1f1ff]|[\u1f201-\u1f202]|\u1f21a|\u1f22f|[\u1f232-\u1f23a]|[\u1f250-\u1f251]|\u203c|\u2049|[\u25aa-\u25ab]|\u25b6|\u25c0|[\u25fb-\u25fe]|\u00a9|\u00ae|\u2122|\u2139|\u1f004|[\u2600-\u26FF]|\u2b05|\u2b06|\u2b07|\u2b1b|\u2b1c|\u2b50|\u2b55|\u231a|\u231b|\u2328|\u23cf|[\u23e9-\u23f3]|[\u23f8-\u23fa]|\u1f0cf|\u2934|\u2935|[\u2190-\u21ff])").unwrap();
    static ref EMOTE_MENTIONS_REGEX: Regex = Regex::new(r"<(?:[^\d>]+|:[A-Za-z0-9]+:)\w+>").unwrap();
    static ref JAPANESE_REGEX: Regex = Regex::new(r"[\u4e00-\u9fbf\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u3000-\u303f]").unwrap();
    static ref NON_ASCII_AND_JAPANESE_REGEX: Regex = Regex::new(r"[^\x00-\x7F\u4e00-\u9fbf\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u3000-\u303f\u2018-\u2019]").unwrap();
}

pub async fn validate_dialog(context: &Context, msg: &Message, background: &String, character: &String, text: &String) -> Result<(), String> {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.dialog;

        let backgrounds = PERSISTENCE_STORAGE.dialog_backgrounds.as_ref().unwrap();
        let characters = PERSISTENCE_STORAGE.dialog_characters.as_ref().unwrap();
        let ref background_strings = PERSISTENCE_STORAGE.background_strings;
        let ref character_strings = PERSISTENCE_STORAGE.character_strings;

        if !backgrounds.contains(background) {
            let message = interface_string.errors["background_not_found"]
                .as_str()
                .replace("{background}", background)
                .replace("{backgrounds}", background_strings);
            msg.channel_id
                .say(&context.http, message.as_str()).await.unwrap();
            return Err("Background not found.".to_string());
        }

        if !characters.contains(character) {
            let message = interface_string.errors["character_not_found"]
                .as_str()
                .replace("{character}", character)
                .replace("{characters}", character_strings);
            msg.channel_id
                .say(&context.http, message).await.unwrap();
            return Err("Character not found.".to_string());
        }
    }

    let text_validation = validate_text(&text);

    if !text_validation.0 {
        match &text_validation.1 {
            TextError::NoMessage => {
                let message = interface_string.errors["no_message"].as_str();
                msg.channel_id
                    .say(&context.http, message).await.unwrap();
                return Err("Message not found.".to_string());
            },
            TextError::LengthTooLong => {
                let message = interface_string.errors["message_too_long"].as_str();
                msg.channel_id
                    .say(&context.http, message).await.unwrap();
                return Err("Message too long.".to_string());
            },
            TextError::WrongCharacterSet => {
                let message = interface_string.errors["wrong_character_set"].as_str();
                msg.channel_id
                    .say(&context.http, message).await.unwrap();
                return Err("Wrong character set.".to_string());
            },
            _ => ()
        }
    }
    Ok(())
}

pub fn validate_text(text: &str) -> (bool, TextError) {
    if text.len() == 0 || text.is_empty() {
        return (false, TextError::NoMessage);
    }
    if JAPANESE_REGEX.is_match(text) && text.len() > 78 || text.len() > 180 {
        return (false, TextError::LengthTooLong);
    }
    if EMOTE_MENTIONS_REGEX.is_match(text) || NON_ASCII_AND_JAPANESE_REGEX.is_match(text) {
        return (false, TextError::WrongCharacterSet);
    }
    (true, TextError::None)
}