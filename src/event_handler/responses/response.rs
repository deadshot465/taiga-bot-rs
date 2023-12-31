use crate::event_handler::hit_or_miss;
use crate::shared::services::openai_service::build_openai_message;
use crate::shared::structs::config::common_settings::COMMON_SETTINGS;
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use crate::shared::structs::config::random_response::{get_random_message, get_shuffled_keywords};
use rand::prelude::*;
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

    let openai_reply_chance = CONFIGURATION
        .get()
        .map(|c| c.openai_reply_chance)
        .unwrap_or_default();

    let reply_with_openai = hit_or_miss(openai_reply_chance);

    let is_kou = KOU.get().copied().unwrap_or(false);
    let message_content = new_message.content.to_lowercase();
    let shuffled_keywords = get_shuffled_keywords();
    let mut replied = false;
    for keyword in shuffled_keywords.into_iter() {
        if !message_content.contains(&keyword) {
            continue;
        }

        let trimmed_keyword = keyword.trim();
        if !is_kou && trimmed_keyword == "kou" {
            continue;
        }

        let response = get_random_message(trimmed_keyword);
        new_message.reply(&ctx.http, response).await?;
        replied = true;
        break;
    }

    if !replied {
        let author_id_skippable = CONFIGURATION
            .get()
            .map(|c| c.skip_user_ids.contains(&new_message.author.id.get()))
            .unwrap_or(false);

        let random_common_response = if reply_with_openai && !author_id_skippable {
            build_openai_message(message_content)
                .await
                .unwrap_or_default()
        } else {
            let mut rng = rand::thread_rng();
            COMMON_SETTINGS
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
