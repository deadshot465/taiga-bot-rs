use regex::Regex;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::{CommandStrings, INTERFACE_SERVICE, Emote, PERSISTENCE_STORAGE};

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"\w").unwrap();
    static ref EMOTE_REGEX: Regex = Regex::new(r"(<a?:\w+:\d+>)").unwrap();
    static ref EMOTE_ID_REGEX: Regex = Regex::new(r"(:\w+:)(\d+)").unwrap();
    static ref EMOTE_IS_ANIMATED_REGEX: Regex = Regex::new(r"(<a)").unwrap();
}

const VALID_COMMANDS: [&'static str; 2] = ["register", "deregister"];
const EMOTE_BASE_LINK: &'static str = "https://cdn.discordapp.com/emojis/";

#[command]
#[description = "Register or deregister an emote from the bot. Note that emotes from servers which the bot is not in won't work as expected."]
#[usage = "<register/deregister> <name> <emote>"]
#[example = "register kek :kek:"]
#[bucket = "fun"]
pub async fn emote(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get interface strings.
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.emote;
    }

    let cmd = args.single::<String>();
    let name = args.single::<String>();
    let emote = args.single::<String>();
    // If the command lacks any single argument, abort.
    if cmd.is_err() || name.is_err() {
        msg.channel_id.say(&context.http, interface_string.errors["length_too_short"].as_str())
            .await?;
        return Ok(())
    }
    // Unwrap the value by shadowing the variable.
    let cmd = cmd.unwrap().to_lowercase();
    // If the command is neither `register` nor `deregister,` abort.
    if !VALID_COMMANDS.contains(&cmd.as_str()) {
        msg.channel_id.say(&context.http, interface_string.errors["unsupported_operation"].as_str())
            .await?;
        return Ok(())
    }
    let name = name.unwrap().to_lowercase();
    let name_regex = &*NAME_REGEX;
    // If the name is invalid, abort.
    if !name_regex.is_match(&name) {
        msg.channel_id.say(&context.http, interface_string.errors["invalid_name"].as_str())
            .await?;
        return Ok(())
    }

    match cmd.as_str() {
        "register" => {
            unsafe {
                if emote.is_err() {
                    msg.channel_id.say(&context.http, interface_string.errors["invalid_emote"].as_str())
                        .await?;
                    return Ok(())
                }
                let emote = emote.unwrap();
                let emote_regex = &*EMOTE_REGEX;
                // If it's not an emote, abort.
                if !emote_regex.is_match(&emote) {
                    msg.channel_id.say(&context.http, interface_string.errors["invalid_emote"].as_str())
                        .await?;
                    return Ok(())
                }
                let id_regex = &*EMOTE_ID_REGEX;
                let id = id_regex.captures(&emote)
                    .unwrap()
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .unwrap();
                let file_extension = if (&*EMOTE_IS_ANIMATED_REGEX).is_match(&emote) {
                    ".gif"
                }
                else {
                    ".png"
                };
                let link = String::from(EMOTE_BASE_LINK) + &id.to_string() + file_extension;
                let emote_entity = Emote::new(name.as_str(), id, link.as_str(), emote.as_str());
                let persistence = &mut PERSISTENCE_STORAGE;
                let config = persistence.config.as_mut().unwrap();
                config.emotes.push(emote_entity);
                persistence.write();
                msg.channel_id.say(&context.http, "Successfully added the emote!").await?;
            }
        },
        "deregister" => {
            unsafe {
                let persistence = &mut PERSISTENCE_STORAGE;
                let config = persistence.config.as_mut().unwrap();
                let mut index: usize = 0;
                for emote in config.emotes.iter().enumerate() {
                    if emote.1.name.as_str() == name.as_str() {
                        index = emote.0;
                        break;
                    }
                }
                config.emotes.remove(index);
                persistence.write();
                msg.channel_id.say(&context.http, "Successfully removed the emote!").await?;
            }
        },
        _ => ()
    }
    Ok(())
}