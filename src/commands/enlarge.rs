use regex::Regex;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;

const EMOTE_BASE_LINK: &'static str = "https://cdn.discordapp.com/emojis/";

#[command]
pub fn enlarge(context: &mut Context, msg: &Message) -> CommandResult {

    lazy_static! {
        static ref EMOTE_REGEX: Regex = Regex::new(r"(<a?:\w+:\d+>)").unwrap();
        // Looking ahead and looking back currently is not supported.
        //static ref EMOTE_ID_REGEX: Regex = Regex::new(r"[^:]+(?=>)").unwrap();
        static ref EMOTE_ID_REGEX: Regex = Regex::new(r"(:\w+:)(\d+)").unwrap();
        static ref EMOTE_IS_ANIMATED_REGEX: Regex = Regex::new(r"(<a)").unwrap();
    }

    if !EMOTE_ID_REGEX.is_match(msg.content.as_str()) {
        msg.channel_id.say(&context.http, "What do you think I could do if you don't even give me an emote?")?;
        return Ok(());
    }
    let splits: Vec<&str> = msg.content.split(' ').collect();
    let mut emote_links: Vec<String> = vec![];

    for item in splits.iter() {
        if EMOTE_REGEX.is_match(item) {
            for capture in EMOTE_REGEX.captures_iter(item) {
                let emote = capture.get(1).unwrap().as_str();
                let id_capture = EMOTE_ID_REGEX.captures(emote).unwrap();
                let id = id_capture.get(2).unwrap().as_str();

                emote_links.push(format!("{}{}{}", EMOTE_BASE_LINK, id, if EMOTE_IS_ANIMATED_REGEX.is_match(emote) {
                    ".gif"
                }
                else {
                    ".png"
                }));
            }
        }
    }

    for link in emote_links.iter() {
        msg.channel_id.say(&context.http, link)?;
    }

    Ok(())
}