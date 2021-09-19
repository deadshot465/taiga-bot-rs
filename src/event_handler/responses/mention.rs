use crate::event_handler::hit_or_miss;
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use crate::shared::structs::config::random_response::get_random_message;
use serenity::model::prelude::Message;
use serenity::prelude::*;

pub async fn handle_mention_self(ctx: &Context, new_message: &Message) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    let mention_reply_chance = CONFIGURATION
        .get()
        .map(|c| c.mention_reply_chance)
        .unwrap_or_default();

    if !hit_or_miss(mention_reply_chance) {
        return Ok(());
    }

    let bot_id = CONFIGURATION.get().map(|c| c.bot_id).unwrap_or_default();

    let is_kou = KOU.get().copied().unwrap_or(false);

    if new_message.content.contains(&bot_id.to_string()) {
        let random_message = get_random_message(if is_kou { "kou" } else { "taiga" });

        new_message.reply(&ctx.http, random_message).await?;
    }

    Ok(())
}
