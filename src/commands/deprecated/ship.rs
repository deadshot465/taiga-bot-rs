use crate::shared::{search_user, ShipMessage};
use crate::{InterfaceService, PersistenceService};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::prelude::Context;
use std::borrow::{Borrow, Cow};

const KOU_EMOTE_URL: &str = "https://cdn.discordapp.com/emojis/700119260394946620.png";
const HIRO_EMOTE_URL: &str = "https://cdn.discordapp.com/emojis/704022326412443658.png";
const KOU_NAME: &str = "Minamoto Kou";
const HIRO_NAME: &str = "Akiba Hiro";

#[command]
#[aliases("lover", "love")]
#[description = "Ship two users."]
#[usage = "<user1> <user2>"]
#[example = "Taiga Keitaro"]
#[bucket = "fun"]
pub async fn ship(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg1 = args.single::<String>()?;
    let arg2 = args.single::<String>()?;
    let lower_arg1 = arg1.to_lowercase();
    let lower_arg2 = arg2.to_lowercase();

    if lower_arg1.contains("kou") && lower_arg2.contains("hiro") {
        ship_secret_romance(context, msg, KOU_NAME, HIRO_NAME).await?;
        return Ok(());
    } else if lower_arg1.contains("hiro") && lower_arg2.contains("kou") {
        ship_secret_romance(context, msg, HIRO_NAME, KOU_NAME).await?;
        return Ok(());
    }

    let guild = context.cache.guild(msg.guild_id.unwrap()).await.unwrap();
    let target1: &Member;
    let search_result = search_user(context, &guild, arg1.as_str()).await?;
    if !search_result.is_empty() {
        target1 = &search_result[0];
    } else {
        msg.channel_id
            .say(
                &context.http,
                format!("No user found for {}", arg1.as_str()),
            )
            .await?;
        return Ok(());
    }

    let target2: Option<&Member>;
    let search_result_2 = search_user(context, &guild, arg2.as_str()).await?;
    target2 = find_next_user(target1, search_result_2.borrow());
    if target2.is_none() {
        msg.channel_id
            .say(
                &context.http,
                format!("No user found for {}", arg2.as_str()),
            )
            .await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let interface = data.get::<InterfaceService>().unwrap();
    let persistence_lock = persistence.read().await;
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    let ship_messages = persistence_lock.ship_messages.as_ref().unwrap();
    let (score, message) = calculate_score(target1, target2.unwrap(), is_kou, ship_messages).await;
    let img_url1 = target1.user.avatar_url().unwrap_or_default();
    let img_url2 = target2.unwrap().user.avatar_url().unwrap_or_default();

    let client = reqwest::Client::new();
    let response = client
        .get(
            format!(
                "https://api.alexflipnote.dev/ship?user={}&user2={}",
                img_url1.as_str(),
                img_url2.as_str()
            )
            .as_str(),
        )
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();

    let name1 = target1
        .nick
        .clone()
        .unwrap_or_else(|| target1.user.name.clone());
    let name2 = target2
        .unwrap()
        .nick
        .clone()
        .unwrap_or_else(|| target2.unwrap().user.name.clone());
    let build_message = message
        .replace("{name}", name1.as_str())
        .replace("{name2}", name2.as_str());
    drop(persistence_lock);
    drop(data);
    let files: Vec<(&[u8], &str)> = vec![(response.borrow(), "result.png")];

    msg.channel_id
        .send_files(&context.http, files, |m| {
            m.embed(|e| {
                e.title(format!("{} and {}", name1, name2));
                e.field(
                    format!("Your love score is {}", score),
                    build_message.as_str(),
                    false,
                );
                e.attachment("result.png");
                e
            })
        })
        .await?;
    Ok(())
}

async fn ship_secret_romance<'a>(
    context: &Context,
    msg: &Message,
    first: &'a str,
    second: &'a str,
) -> CommandResult {
    let (score, message) = (
        10000,
        format!(
            "What are you talking about? {} and {} of course are the cutest two!",
            first, second
        ),
    );
    let title = format!("{} and {}", first, second);

    let client = reqwest::Client::new();
    let response = client
        .get(
            format!(
                "https://api.alexflipnote.dev/ship?user={}&user2={}",
                if first == KOU_NAME {
                    KOU_EMOTE_URL
                } else {
                    HIRO_EMOTE_URL
                },
                if second == HIRO_NAME {
                    HIRO_EMOTE_URL
                } else {
                    KOU_EMOTE_URL
                }
            )
            .as_str(),
        )
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();

    let files: Vec<(&[u8], &str)> = vec![(response.borrow(), "result.png")];

    msg.channel_id
        .send_message(&context.http, |m| {
            m.embed(|e| {
                e.title(title).field(
                    format!("Your love score is {}", score),
                    message.as_str(),
                    false,
                )
            })
        })
        .await?;
    msg.channel_id
        .send_files(&context.http, files, |m| m.content(""))
        .await?;

    Ok(())
}

fn find_next_user<'a>(first_user: &'a Member, seconds: &'a [Member]) -> Option<&'a Member> {
    if seconds.is_empty() {
        return None;
    }
    if seconds.len() == 1 {
        return Some(&seconds[0]);
    }

    for member in seconds.iter() {
        if member.user.id.0 == first_user.user.id.0 {
            continue;
        }
        return Some(member);
    }
    None
}

async fn calculate_score<'a>(
    first_user: &'a Member,
    second_user: &'a Member,
    is_kou: bool,
    ship_messages: &'a [ShipMessage],
) -> (u64, Cow<'a, str>) {
    let first_id = first_user.user.id.0;
    let second_id = second_user.user.id.0;
    let t_id = std::env::var("T_ID").unwrap();
    let k_id = std::env::var("K_ID").unwrap();
    let t_id = t_id.parse::<u64>().unwrap();
    let k_id = k_id.parse::<u64>().unwrap();

    if first_id == second_id {
        (
            100_u64,
            Cow::Borrowed("You're a perfect match... for yourself!"),
        )
    } else if ((first_id == t_id && second_id == k_id) || (first_id == k_id && second_id == t_id))
        && is_kou
    {
        (u64::MAX, Cow::Borrowed("Oops...You found us..."))
    } else {
        let score = ((first_id + second_id) / 7 % 100) as u64;
        (score, Cow::Owned(find_message(score, ship_messages).await))
    }
}

async fn find_message(score: u64, ship_messages: &[ShipMessage]) -> String {
    let msg = &ship_messages
        .iter()
        .filter(|m| score <= m.max_score as u64)
        .collect::<Vec<&ShipMessage>>();
    msg[0].message.clone()
}