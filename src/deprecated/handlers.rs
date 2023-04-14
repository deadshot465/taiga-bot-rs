use crate::commands::guide::build_embed;
use crate::{
    get_dialog, get_image, search_user, CommandGroupCollection, Emote, InterfaceService,
    PersistenceService, UserRecords,
};
use chrono::{Duration, Local, Utc};
use rand::{prelude::*, thread_rng, Rng};
use regex::Regex;
use serenity::framework::standard::DispatchError;
use serenity::utils::Color;
use serenity::{async_trait, framework::standard::macros::hook, model::prelude::*, prelude::*};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;

async fn handle_user_replies(context: &Context, msg: &Message) {
    let lock = context.data.read().await;
    let persistence = lock
        .get::<PersistenceService>()
        .expect("Failed to retrieve assets service.");
    let interface = lock
        .get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let _persistence = Arc::clone(persistence);
    let persistence_lock = _persistence.read().await;
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(lock);
    if persistence_lock
        .channel_settings
        .as_ref()
        .expect("Failed to retrieve channel settings.")
        .ignored_channels
        .contains(&msg.channel_id.0)
    {
        drop(persistence_lock);
        return;
    }
    if msg.author.bot {
        return;
    }
    if is_kou {
        return;
    }
    let user_replies = persistence_lock
        .user_replies
        .as_ref()
        .expect("Failed to retrieve user replies.");
    if !user_replies
        .iter()
        .map(|r| r.user)
        .any(|x| x == msg.author.id.0)
    {
        return;
    }

    let reply_user_chance: u8 = env::var("RDM_REPLY_USER_CHANCE")
        .expect("Failed to retrieve random user reply chance.")
        .parse::<u8>()
        .expect("Failed to parse u8 from string.");
    if hit_or_miss(reply_user_chance) {
        let messages = user_replies
            .iter()
            .find_map(|r| {
                if r.user == msg.author.id.0 {
                    Some(&r.messages)
                } else {
                    None
                }
            })
            .expect("Failed to find and map user reply messages.");
        if msg.author.id.0 == 677249244842950684 {
            let average_probability = 1_f64 / (messages.len() as f64);
            let specialized_chance = (average_probability / 2_f64).floor();
            let reply = messages
                .iter()
                .find(|s| s.contains("You know what") && s.contains("moderate"))
                .expect("Failed to find specialized messages.");
            let hit = hit_or_miss(specialized_chance as u8);
            if hit {
                msg.reply(&context.http, reply)
                    .await
                    .expect("Failed to reply to the user.");
            } else {
                let _messages = messages
                    .iter()
                    .filter(|s| !s.starts_with("You know what"))
                    .collect::<Vec<&String>>();
                let index = thread_rng().gen_range(0.._messages.len());
                msg.reply(&context.http, _messages[index])
                    .await
                    .expect("Failed to reply to the user.");
            }
        } else {
            let index = thread_rng().gen_range(0..messages.len());
            msg.reply(&context.http, messages[index].as_str())
                .await
                .expect("Failed to reply to the user.");
        }
        drop(persistence_lock);
        let mut persistence_lock = _persistence.write().await;
        let user_id = msg.author.id.0.to_string();
        let user_records = persistence_lock
            .user_records
            .as_mut()
            .expect("Failed to get user records.")
            .entry(user_id)
            .or_insert_with(UserRecords::new);
        let reply_count = &mut user_records.replies;
        *reply_count += 1;
        drop(persistence_lock);
    } else {
        drop(persistence_lock);
    }
}

async fn emote_command(context: &Context, msg: &Message, emote: &Emote) {
    let mut cmd = std::env::var("PREFIX").expect("Failed to retrieve bot's prefix.");
    cmd += emote.name.as_str();
    let remains: &str;
    if msg.content.len() > cmd.len() {
        remains = &msg.content[cmd.len() + 1..];
    } else {
        remains = "";
    }
    let count = remains.parse::<u8>();
    if let Ok(c) = count {
        let mut message = String::from(&emote.raw);
        for _ in 1..c {
            message += " ";
            message += &emote.raw;
        }
        msg.channel_id
            .say(&context.http, &message)
            .await
            .expect("Failed to send emote messages.");
    } else {
        msg.channel_id
            .say(&context.http, &emote.link)
            .await
            .expect("Failed to send the emote link.");
    }
}

#[hook]
pub async fn unknown_command(context: &Context, msg: &Message, cmd: &str) {
    let data_lock = context.data.read().await;
    let persistence = data_lock
        .get::<PersistenceService>()
        .expect("Failed to retrieve assets service.");
    let interface = data_lock
        .get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let persistence_clone = Arc::clone(persistence);
    let interface_clone = Arc::clone(interface);
    drop(data_lock);
    let persistence_lock = persistence_clone.read().await;
    let interface_lock = interface_clone.read().await;
    let config = persistence_lock
        .config
        .as_ref()
        .expect("Failed to retrieve configuration.");
    let emote_exist = config.emotes.iter().find(|e| e.name == cmd);
    if let Some(e) = emote_exist {
        emote_command(context, msg, e).await;
        return;
    }
    drop(persistence_lock);

    let failed_messages: &Vec<String>;
    failed_messages = &interface_lock
        .interface_strings
        .as_ref()
        .expect("Failed to retrieve interface strings.")
        .failed_messages;
    let index = thread_rng().gen_range(0..failed_messages.len());
    let response = failed_messages[index].replace("{command}", cmd);
    msg.channel_id
        .say(&context.http, &response)
        .await
        .expect("Failed to show failed messages.");
    drop(interface_lock);
}

#[hook]
pub async fn message_received(context: &Context, msg: &Message) {
    handle_user_replies(context, msg).await;

    let mut is_persistence_changed = false;
    let reminders = persistence_lock
        .reminders
        .as_mut()
        .expect("Failed to retrieve reminders.");
    let mut user_id_to_remove = HashSet::new();
    // Remind users
    for (user_id, reminder) in reminders.iter() {
        if !(Local::now() > reminder.datetime) {
            continue;
        }
        let user = context.cache.user(UserId(*user_id)).await;
        if let Some(u) = user {
            let msg = reminder.message.clone();
            user_id_to_remove.insert(*user_id);
            u.direct_message(&context.http, |m| m.content(msg.as_str()))
                .await
                .expect("Failed to dm the user.");
            break;
        }
    }
    for id in user_id_to_remove.iter() {
        reminders.remove(id);
        is_persistence_changed = true;
    }

    // Revoke Smote role
    let smote_users = &mut persistence_lock.smote_users;
    let mut smote_users_to_remove = HashSet::new();
    for (user_id, due_time) in smote_users.iter_mut() {
        if Local::now() > *due_time {
            smote_users_to_remove.insert(*user_id);
        }
    }
    for id in smote_users_to_remove.iter() {
        let mut guild = guild.clone().expect("Failed to get guild from cache.");
        let member = guild.members.get_mut(&UserId::from(*id));
        let member = match member {
            Some(m) => m,
            None => {
                continue;
            }
        };
        for role_id in SMITE_ROLE_IDS.iter() {
            let result: serenity::Result<()> = member
                .remove_role(&context.http, RoleId::from(*role_id))
                .await;
            if result.is_ok() {
                break;
            }
        }
        smote_users.remove(id);
        is_persistence_changed = true;
    }
    if is_persistence_changed {
        persistence_lock.write();
    }
    drop(persistence_lock);
}

#[hook]
pub async fn before(context: &Context, msg: &Message, command_name: &str) -> bool {
    let lock = context.data.read().await;
    let persistence = lock
        .get::<PersistenceService>()
        .expect("Failed to retrieve assets service.");
    let _persistence = Arc::clone(persistence);
    drop(lock);
    let persistence_lock = _persistence.read().await;
    let channel = msg
        .channel(&context.cache)
        .await
        .expect("Failed to retrieve message channel.");
    let guild_channel = channel.guild();
    let enabled_channels = &persistence_lock
        .channel_settings
        .as_ref()
        .expect("Failed to retrieve channel settings.")
        .enabled_channels;
    if ADMIN_COMMANDS.contains(&command_name) {
        drop(persistence_lock);
        return true;
    }
    if !enabled_channels.contains(&msg.channel_id.0) && guild_channel.is_some() {
        drop(persistence_lock);
        return false;
    }
    true
}

#[hook]
pub async fn dispatch_error(context: &Context, msg: &Message, error: DispatchError) {
    let http = &context.http;
    match error {
        DispatchError::LackingRole => {
            msg.reply(
                http,
                "You don't have the required role to perform such operation.",
            )
            .await
            .expect(
                "Failed to indicate that the user needs proper roles to perform the operation.",
            );
        }
        DispatchError::OnlyForGuilds => {
            msg.reply(http, "This command can only be used in a guild.")
                .await
                .expect("Failed to indicate that the command can only be used in a guild.");
        }
        DispatchError::OnlyForOwners => {
            msg.reply(http, "This command can only be used by the owners.")
                .await
                .expect("Failed to indicate that the command can only be used by the owners.");
        }
        DispatchError::Ratelimited(time) => {
            let lock = context.data.read().await;
            let interface = lock
                .get::<InterfaceService>()
                .expect("Failed to retrieve interface service.");
            let interface_lock = interface.read().await;
            let error_msg = interface_lock
                .interface_strings
                .as_ref()
                .expect("Failed to retrieve interface strings.")
                .cool_down
                .clone();
            drop(interface_lock);
            drop(lock);
            let seconds = time.as_secs().to_string();
            let error_msg = error_msg.replace("{timeLeft}", seconds.as_str());
            msg.reply(http, &error_msg)
                .await
                .expect("Failed to show cool down message.");
        }
        _ => {
            msg.reply(http, "There was an error trying to execute that command...")
                .await
                .expect("Failed to reply to a generic error.");
        }
    }
}
