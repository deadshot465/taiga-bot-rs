use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::{UserRecords, PersistenceService};
use crate::shared::Character;
use std::collections::HashMap;

const MATURE_HIRO_EMOTE_IDS: [&str; 5] = [
    "703591584305774662",
    "710951618576908289",
    "711192310767157248",
    "710957588237385789",
    "711227408933453844"
];

const ENDINGS: [&str; 4] = [
    "Perfect", "Good", "Bad", "Worst"
];

const KOU_GIFS: [&str; 5] = [
    "https://tetsukizone.com/images/kou.gif",
    "https://tetsukizone.com/images/kou2.gif",
    "https://tetsukizone.com/images/kou3.gif",
    "https://tetsukizone.com/images/kou4.gif",
    "https://cdn.discordapp.com/emojis/705279783340212265.gif"
];

#[command]
#[aliases("r")]
#[description = "Tells you what route to play next."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
pub async fn route(context: &Context, msg: &Message) -> CommandResult {
    let route = get_route(context).await;
    let footer = format!("Play {}'s route next. All bois are best bois.", get_first_name(route.name.as_str()));
    let color = u32::from_str_radix(&route.color.as_str(), 16).unwrap() as i32;
    let mut ending = ENDINGS[thread_rng().gen_range(0, ENDINGS.len())];
    if route.name == "Hiro Akiba (Mature)" || route.name == "Minamoto Kou" {
        ending = "Perfect";
    }

    msg.channel_id.send_message(&context.http, |m| m
        .embed(|e| e
            .author(|author| {
                if let Some(url) = msg.author.avatar_url().as_ref() {
                    author.icon_url(url);
                }
                author.name(&msg.author.name)
            })
            .color(Color::from(color))
            .description(&route.description)
            .field("Age", route.age, true)
            .field("Birthday", &route.birthday, true)
            .field("Animal Motif", &route.animal, true)
            .footer(|f| f.text(footer))
            .thumbnail(if route.name == "Hiro Akiba (Mature)" {
                let id = MATURE_HIRO_EMOTE_IDS[thread_rng().gen_range(0, MATURE_HIRO_EMOTE_IDS.len())];
                get_emote_url(id)
            }
            else if route.name == "Minamoto Kou" {
                KOU_GIFS[thread_rng().gen_range(0, KOU_GIFS.len())].to_string()
            }
            else {
                get_emote_url(route.emote_id.as_str())
            })
            .title(format!("Next: {}, {} Ending", &route.name, ending)))).await?;

    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let mut persistence_lock = persistence.write().await;
    let user_records = persistence_lock.user_records.as_mut().unwrap();
    let user_record = user_records.entry(msg.author.id.0.to_string())
        .or_insert(UserRecords::new());
    let r = user_record.route.entry(route.name.clone())
        .or_insert(HashMap::new());
    *r.entry(format!("{} Ending", ending))
        .or_insert(0) += 1;

    match route.name.as_str() {
        "Taiga Akatora" => {
            persistence_lock.update_credits(context, msg.author.id.0, msg.channel_id.0, 2, "plus")
                .await;
        },
        "Minamoto Kou" => {
            persistence_lock.update_credits(context, msg.author.id.0, msg.channel_id.0, 10, "plus")
                .await;
        }
        _ => ()
    }

    drop(persistence_lock);
    Ok(())
}

async fn get_route(context: &Context) -> Character {
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let persistence_lock = persistence.read().await;
    let res = thread_rng().gen_range(0, 100);
    let routes = persistence_lock.routes.as_ref().unwrap();
    match res {
        x if x >= 0 && x <= 14 => routes[0].clone(),
        x if x >= 15 && x <= 19 => routes[1].clone(),
        x if x >= 20 && x <= 22 => routes[7].clone(),
        x if x >= 23 && x <= 38 => routes[2].clone(),
        x if x >= 39 && x <= 53 => routes[3].clone(),
        x if x >= 54 && x <= 68 => routes[4].clone(),
        x if x >= 69 && x <= 83 => routes[5].clone(),
        x if x >= 84 && x <= 99 => routes[6].clone(),
        _ => routes[0].clone()
    }
}

fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[0]
}

fn get_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.gif?v=1", emote_id)
}