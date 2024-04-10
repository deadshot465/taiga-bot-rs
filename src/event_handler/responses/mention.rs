use crate::event_handler::hit_or_miss;
use crate::shared::structs::config::random_response::get_random_message;
use crate::shared::structs::ContextData;
use serenity::model::prelude::Message;
use serenity::prelude::*;

pub async fn handle_mention_self(
    ctx: &Context,
    new_message: &Message,
    data: &ContextData,
) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    let mention_reply_chance = data.config.mention_reply_chance;

    if !hit_or_miss(mention_reply_chance) {
        return Ok(());
    }

    let bot_id = data.config.bot_id;
    let is_kou = data.kou;

    if new_message.content.contains(&bot_id.to_string()) {
        let random_message =
            get_random_message(&data.random_response, if is_kou { "kou" } else { "taiga" });
        new_message.reply(&ctx.http, random_message).await?;
    }

    Ok(())
}
