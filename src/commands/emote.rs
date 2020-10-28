use crate::{Emote, InterfaceService, PersistenceService};
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::utils::Color;
use std::sync::Arc;

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"\w").unwrap();
    static ref EMOTE_REGEX: Regex = Regex::new(r"(<a?:\w+:\d+>)").unwrap();
    static ref EMOTE_ID_REGEX: Regex = Regex::new(r"(:\w+:)(\d+)").unwrap();
    static ref EMOTE_IS_ANIMATED_REGEX: Regex = Regex::new(r"(<a)").unwrap();
}

const VALID_COMMANDS: [&str; 2] = ["register", "deregister"];
const EMOTE_BASE_LINK: &str = "https://cdn.discordapp.com/emojis/";

#[command]
#[description = "Register or deregister an emote from the bot. Note that emotes from servers which the bot is not in won't work as expected."]
#[usage = "<register/deregister> <name> <emote>"]
#[example = "register kek :kek:"]
#[bucket = "fun"]
pub async fn emote(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get interface strings.
    let lock = context.data.read().await;
    let interface = lock.get::<InterfaceService>().unwrap();
    let persistence = lock.get::<PersistenceService>().unwrap();
    let _interface = Arc::clone(interface);
    let _persistence = Arc::clone(persistence);
    drop(lock);
    let interface_lock = _interface.read().await;
    let interface_strings = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface_strings.emote;

    if args.is_empty() {
        let color = u32::from_str_radix("93B986", 16).unwrap();
        let persistence_lock = _persistence.read().await;
        msg.channel_id
            .send_message(&context.http, |m| {
                m.embed(|e| {
                    e.thumbnail("https://cdn.discordapp.com/emojis/730239295155077251.png");
                    e.title("Registered Emotes");
                    e.description("The following is a list of currently registered emotes.");
                    e.color(Color::new(color));
                    let config = persistence_lock.config.as_ref().unwrap();
                    let emotes = &config.emotes;
                    let emote_names = emotes
                        .iter()
                        .map(|e| format!("`{}`, ", e.name.as_str()))
                        .collect::<String>();
                    e.field(
                        "Emotes",
                        &emote_names.trim()[0..emote_names.len() - 2],
                        false,
                    );
                    drop(persistence_lock);
                    e
                })
            })
            .await?;
        return Ok(());
    }

    let cmd = args.single::<String>();
    let name = args.single::<String>();
    let emote = args.single::<String>();
    // If the command lacks any single argument, abort.
    if cmd.is_err() || name.is_err() {
        msg.channel_id
            .say(
                &context.http,
                interface_string.errors["length_too_short"].as_str(),
            )
            .await?;
        return Ok(());
    }
    // Unwrap the value by shadowing the variable.
    let cmd = cmd.unwrap().to_lowercase();
    // If the command is neither `register` nor `deregister,` abort.
    if !VALID_COMMANDS.contains(&cmd.as_str()) {
        msg.channel_id
            .say(
                &context.http,
                interface_string.errors["unsupported_operation"].as_str(),
            )
            .await?;
        return Ok(());
    }
    let name = name.unwrap().to_lowercase();
    let name_regex = &*NAME_REGEX;
    // If the name is invalid, abort.
    if !name_regex.is_match(&name) {
        msg.channel_id
            .say(
                &context.http,
                interface_string.errors["invalid_name"].as_str(),
            )
            .await?;
        return Ok(());
    }

    match cmd.as_str() {
        "register" => {
            if emote.is_err() {
                msg.channel_id
                    .say(
                        &context.http,
                        interface_string.errors["invalid_emote"].as_str(),
                    )
                    .await?;
                return Ok(());
            }
            let emote = emote.unwrap();
            let emote_regex = &*EMOTE_REGEX;
            // If it's not an emote, abort.
            if !emote_regex.is_match(&emote) {
                msg.channel_id
                    .say(
                        &context.http,
                        interface_string.errors["invalid_emote"].as_str(),
                    )
                    .await?;
                return Ok(());
            }
            let id_regex = &*EMOTE_ID_REGEX;
            let id = id_regex
                .captures(&emote)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            let file_extension = if (&*EMOTE_IS_ANIMATED_REGEX).is_match(&emote) {
                ".gif"
            } else {
                ".png"
            };
            let link = String::from(EMOTE_BASE_LINK) + &id.to_string() + file_extension;
            let emote_entity = Emote::new(name.as_str(), id, link.as_str(), emote.as_str());
            let mut persistence_lock = _persistence.write().await;
            let config = persistence_lock.config.as_mut().unwrap();
            config.emotes.push(emote_entity);
            persistence_lock.write();
            drop(persistence_lock);
            msg.channel_id
                .say(&context.http, "Successfully added the emote!")
                .await?;
        }
        "deregister" => {
            let mut persistence_lock = _persistence.write().await;
            let config = persistence_lock.config.as_mut().unwrap();
            let mut index: usize = 0;
            for emote in config.emotes.iter().enumerate() {
                if emote.1.name.as_str() == name.as_str() {
                    index = emote.0;
                    break;
                }
            }
            config.emotes.remove(index);
            persistence_lock.write();
            drop(persistence_lock);
            msg.channel_id
                .say(&context.http, "Successfully removed the emote!")
                .await?;
        }
        _ => (),
    }
    Ok(())
}
