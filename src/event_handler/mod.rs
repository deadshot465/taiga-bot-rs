use poise::FrameworkContext;
use rand::prelude::*;
use serenity::all::FullEvent;

use crate::event_handler::presences::set_initial_presence;
use crate::event_handler::responses::certify::handle_certify;
use crate::event_handler::responses::greet::greet;
use crate::event_handler::responses::handle_bot_responses;
use crate::event_handler::responses::qotd::handle_qotd;
use crate::shared::constants::KOU_SERVER_ID;
use crate::shared::services::message_service::record_message;
use crate::shared::structs::smite::schedule_unsmite;
use crate::shared::structs::{ContextData, ContextError};

pub mod presences;
pub mod responses;

pub async fn handle_event(
    ctx: FrameworkContext<'_, ContextData, ContextError>,
    event: &FullEvent,
) -> Result<(), ContextError> {
    let serenity_ctx = ctx.serenity_context;
    let ctx_data = ctx.user_data().await;

    match event {
        FullEvent::GuildMemberAddition { new_member } => {
            if new_member.guild_id.get() == KOU_SERVER_ID {
                return Ok(());
            }

            let guild = serenity_ctx
                .cache
                .guild(new_member.guild_id)
                .expect("Failed to retrieve guild from cache.")
                .clone();
            if let Err(e) = greet(serenity_ctx, guild, new_member, ctx_data).await {
                tracing::error!("Error when greeting a new member: {}", e);
            }
        }
        FullEvent::Message { new_message } => {
            if let Err(e) = handle_qotd(serenity_ctx, new_message, ctx_data).await {
                tracing::error!("Error when handling qotd: {}", e);
            }

            let endpoint = format!("{}/message/record/new", &ctx_data.config.server_endpoint);
            record_message(serenity_ctx, new_message, ctx_data, endpoint).await?;

            if let Err(e) = handle_bot_responses(serenity_ctx, new_message, ctx_data).await {
                tracing::error!("Error when handling bot responses: {}", e);
            }

            handle_certify(serenity_ctx, new_message, ctx_data).await;
        }
        FullEvent::Ready { data_about_bot } => {
            set_initial_presence(serenity_ctx, ctx_data).await;
            schedule_unsmite(serenity_ctx, ctx_data).await;
            tracing::info!("{} is now online.", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}

pub fn hit_or_miss(probability: i32) -> bool {
    rand::rng().random_range(0..100) < probability
}
