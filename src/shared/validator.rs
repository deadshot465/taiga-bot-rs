use regex::Regex;
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::PERSISTENCE_STORAGE;

pub fn validate_dialog(context: &mut Context, msg: &Message, background: &String, character: &String, text: &String) -> Result<(), String> {

    lazy_static! {
        static ref EMOJI_REGEX: Regex = Regex::new(r"(?:[\u2700-\u27bf]|(?:\ud83c[\udde6-\uddff]){2}|[\ud800-\udbff][\udc00-\udfff]|[\u0023-\u0039]\ufe0f?\u20e3|\u3299|\u3297|\u303d|\u3030|\u24c2|\ud83c[\udd70-\udd71]|\ud83c[\udd7e-\udd7f]|\ud83c\udd8e|\ud83c[\udd91-\udd9a]|\ud83c[\udde6-\uddff]|\ud83c[\ude01-\ude02]|\ud83c\ude1a|\ud83c\ude2f|\ud83c[\ude32-\ude3a]|\ud83c[\ude50-\ude51]|\u203c|\u2049|[\u25aa-\u25ab]|\u25b6|\u25c0|[\u25fb-\u25fe]|\u00a9|\u00ae|\u2122|\u2139|\ud83c\udc04|[\u2600-\u26FF]|\u2b05|\u2b06|\u2b07|\u2b1b|\u2b1c|\u2b50|\u2b55|\u231a|\u231b|\u2328|\u23cf|[\u23e9-\u23f3]|[\u23f8-\u23fa]|\ud83c\udccf|\u2934|\u2935|[\u2190-\u21ff])").unwrap();
        static ref EMOTE_MENTIONS_REGEX: Regex = Regex::new(r"<(?:[^\d>]+|:[A-Za-z0-9]+:)\w+>").unwrap();
        static ref JAPANESE_REGEX: Regex = Regex::new(r"[\u4e00-\u9fbf\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u3000-\u303f]").unwrap();
        static ref NON_ASCII_AND_JAPANESE_REGEX: Regex = Regex::new(r"[^\x00-\x7F\u4e00-\u9fbf\u3040-\u309f\u30a0-\u30ff\uff00-\uff9f\u3000-\u303f\u2018-\u2019]").unwrap();
    }

    unsafe {
        let ref backgrounds = PERSISTENCE_STORAGE.get_instance().dialog_backgrounds;
        let ref characters = PERSISTENCE_STORAGE.get_instance().dialog_characters;
        if !backgrounds.contains(background) {
            msg.channel_id
                .say(&context.http, format!("Sorry, but I couldn't find `{}` as a location\nAvailable backgrounds are: `{}`.", background, &PERSISTENCE_STORAGE.get_instance().background_strings)).unwrap();
            return Err("Background not found.".to_string());
        }
        if !characters.contains(character) {
            msg.channel_id
                .say(&context.http, format!("Sorry, but I don't think that `{}` is a character in Camp Buddy\nAvailable characters are: `{}`.", character, &PERSISTENCE_STORAGE.get_instance().character_strings)).unwrap();
            return Err("Character not found.".to_string());
        }
    }
    if text.len() == 0 {
        msg.channel_id
            .say(&context.http, "At least give me something to send, you dumbass.").unwrap();
        return Err("Message not found.".to_string());
    }
    if JAPANESE_REGEX.is_match(text) && text.len() > 78 || text.len() > 180 {
        msg.channel_id
            .say(&context.http, "Sorry, the message limit is 180 characters for latin characters, 78 characters for Japanese <:TaigaAck2:700006264507465778>").unwrap();
        return Err("Message too long.".to_string());
    }
    if EMOJI_REGEX.is_match(text) || EMOTE_MENTIONS_REGEX.is_match(text) || NON_ASCII_AND_JAPANESE_REGEX.is_match(text) {
        msg.channel_id
            .say(&context.http, "I don't do emotes, mentions, non-latin and non-Japanese characters").unwrap();
        return Err("Wrong character set.".to_string());
    }

    Ok(())
}