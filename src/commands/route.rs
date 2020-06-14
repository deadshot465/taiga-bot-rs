use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::utils::Color;
use crate::PERSISTENCE_STORAGE;
use crate::shared::Character;

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
pub async fn route(context: &Context, msg: &Message) -> CommandResult {
    let route = get_route().await;
    let footer = format!("Play {}'s route next. All bois are best bois.", get_first_name(route.name.as_str()));
    let color = u32::from_str_radix(&route.color.as_str(), 16).unwrap() as i32;
    let mut ending = ENDINGS[thread_rng().gen_range(0, ENDINGS.len())];
    if route.name == "Hiro Akiba (Mature)" || route.name == "Minamoto Kou" {
        ending = "Perfect";
    }

    msg.channel_id.send_message(&context.http, |m| m
        .embed(|e| e
            .author(|author| {
                if let Some(url) = msg.author.avatar.as_ref() {
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

    Ok(())
}

async fn get_route() -> &'static Character {
    let res = thread_rng().gen_range(0, 100);
    unsafe {
        let routes = &PERSISTENCE_STORAGE.get_instance().await.routes;
        match res {
            x if x >= 0 && x <= 14 => &routes[0],
            x if x >= 15 && x <= 19 => &routes[1],
            x if x >= 20 && x <= 22 => &routes[7],
            x if x >= 23 && x <= 38 => &routes[2],
            x if x >= 39 && x <= 53 => &routes[3],
            x if x >= 54 && x <= 68 => &routes[4],
            x if x >= 69 && x <= 83 => &routes[5],
            x if x >= 84 && x <= 99 => &routes[6],
            _ => &routes[0]
        }
    }
}

fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[0]
}

fn get_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.gif?v=1", emote_id)
}