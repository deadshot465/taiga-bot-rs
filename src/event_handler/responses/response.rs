use rand::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::event_handler::hit_or_miss;
use crate::shared::services::openai_service::build_openai_message;
use crate::shared::structs::config::random_response::{get_random_message, get_shuffled_keywords};
use crate::shared::structs::ContextData;

pub async fn handle_responses(
    ctx: &Context,
    new_message: &Message,
    data: &ContextData,
) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    let random_reply_chance = data.config.random_reply_chance;

    if !hit_or_miss(random_reply_chance) {
        return Ok(());
    }

    let openai_reply_chance = data.config.openai_reply_chance;

    let reply_with_openai = hit_or_miss(openai_reply_chance);

    let is_kou = data.kou;
    let message_content = new_message.content.to_lowercase();
    let shuffled_keywords = get_shuffled_keywords(&data.random_response);
    let mut replied = false;
    for keyword in shuffled_keywords.into_iter() {
        if !message_content.contains(&keyword) {
            continue;
        }

        let trimmed_keyword = keyword.trim();
        if !is_kou && trimmed_keyword == "kou" {
            continue;
        }

        let response = get_random_message(&data.random_response, trimmed_keyword);
        new_message.reply(&ctx.http, response).await?;
        replied = true;
        break;
    }

    if !replied {
        let author_id_skippable = data
            .config
            .skip_user_ids
            .contains(&new_message.author.id.get());

        let random_common_response = if reply_with_openai && !author_id_skippable {
            build_openai_message(ctx, new_message, data)
                .await
                .unwrap_or_default()
        } else {
            let mut rng = thread_rng();
            data.common_settings
                .common_responses
                .choose(&mut rng)
                .cloned()
                .unwrap_or_default()
        };

        if !random_common_response.is_empty() {
            new_message.reply(&ctx.http, random_common_response).await?;
        }
    }

    Ok(())
}
