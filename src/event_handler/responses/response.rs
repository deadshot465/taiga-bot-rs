use crate::event_handler::hit_or_miss;
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use crate::shared::structs::config::random_response::{get_random_message, get_shuffled_keywords};
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_responses(ctx: &Context, new_message: &Message) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    let random_reply_chance = CONFIGURATION
        .get()
        .map(|c| c.random_reply_chance)
        .unwrap_or_default();

    if !hit_or_miss(random_reply_chance) {
        return Ok(());
    }

    let is_kou = KOU.get().copied().unwrap_or(false);
    let message_content = new_message.content.to_lowercase();
    let shuffled_keywords = get_shuffled_keywords();
    for keyword in shuffled_keywords.into_iter() {
        if !message_content.contains(&keyword) {
            continue;
        }

        let trimmed_keyword = keyword.trim();
        if !is_kou && trimmed_keyword == "kou" {
            continue;
        } else if is_kou && trimmed_keyword != "kou" {
            continue;
        }

        let response = get_random_message(trimmed_keyword);
        new_message.reply(&ctx.http, response).await?;
        break;
    }

    Ok(())
}
