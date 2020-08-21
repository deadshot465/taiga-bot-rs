use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::{UserRecords, InterfaceService, PersistenceService, PersistenceStorage};
use crate::shared::Character;
use std::sync::Arc;
use tokio::sync::{MutexGuard, RwLockReadGuard};

#[command]
#[aliases("v")]
#[description = "Tells you your next valentine."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
pub async fn valentine(context: &Context, msg: &Message) -> CommandResult {
    let data = context.data.read().await;
    let interface = data.get::<InterfaceService>().unwrap();
    let persistence = data.get::<PersistenceService>().unwrap();
    let _interface = Arc::clone(interface);
    let _persistence = Arc::clone(persistence);
    drop(data);
    let interface_lock = _interface.read().await;
    let interface_strings = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface_strings.valentine;
    let persistence_lock = _persistence.read().await;
    let valentine = get_valentine(&persistence_lock).await;
    drop(persistence_lock);
    let is_keitaro = get_first_name(valentine.name.as_str()) == "Keitaro";
    let prefix_suffix = if is_keitaro {
        "~~"
    }
    else {
        ""
    };

    let footer = if is_keitaro {
        interface_string.infos["keitaro_footer"].clone()
            .replace("{firstName}", get_first_name(valentine.name.as_str()))
    }
    else {
        interface_string.infos["normal_footer"].clone()
            .replace("{firstName}", get_first_name(valentine.name.as_str()))
    };

    let valentine_name = format!("{}Your valentine is {}{}", prefix_suffix, valentine.name.as_str(), prefix_suffix);
    let color = u32::from_str_radix(&valentine.color.as_str(), 16).unwrap() as i32;

    if is_keitaro {
        let message = interface_string.infos["keitaro_header"].as_str();
        msg.channel_id.say(&context.http, message).await?;
    }

    msg.channel_id.send_message(&context.http, |m| {
        m.embed(|e| {
            e.author(|a| {
                if let Some(url) = msg.author.avatar_url().as_ref() {
                    a.icon_url(url);
                }
                a.name(&msg.author.name)
            })
                .color(Color::from(color))
                .description(format!("{}{}{}", prefix_suffix, valentine.description.as_str(), prefix_suffix))
                .field("Age", valentine.age, true)
                .field("Birthday", valentine.birthday.as_str(), true)
                .field("Animal Motif", valentine.animal.as_str(), true)
                .footer(|f| f.text(&footer))
                .thumbnail(get_emote_url(valentine.emote_id.as_str()))
                .title(valentine_name.as_str())
        })
    }).await?;

    let mut persistence_lock = _persistence.write().await;
    let user_records = persistence_lock.user_records.as_mut().unwrap();
    let user_record = user_records.entry(msg.author.id.0.to_string())
        .or_insert(UserRecords::new());
    *user_record.valentine.entry(valentine.name.clone())
        .or_insert(0) += 1;
    drop(persistence_lock);
    drop(interface_lock);
    Ok(())
}

async fn get_valentine(persistence: &RwLockReadGuard<'_, PersistenceStorage>) -> Character {
    let valentines = persistence.valentines.as_ref().unwrap();
    valentines[thread_rng().gen_range(0, valentines.len())].clone()
}

fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[0]
}

fn get_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.png?v=1", emote_id)
}