use crate::event_handler::responses::emote::handle_emote;
use crate::event_handler::responses::mention::handle_mention_self;
use crate::event_handler::responses::reaction::handle_reactions;
use crate::event_handler::responses::response::handle_responses;
use crate::shared::structs::config::channel_control::CHANNEL_CONTROL;
use serenity::model::prelude::Message;
use serenity::prelude::*;

pub mod emote;
pub mod greet;
pub mod mention;
pub mod qotd;
pub mod reaction;
pub mod response;

pub async fn handle_bot_responses(ctx: &Context, new_message: &Message) -> anyhow::Result<()> {
    let is_channel_ignored = {
        CHANNEL_CONTROL
            .get()
            .expect("Failed to get channel control.")
            .read()
            .await
            .ignored_channels
            .iter()
            .any(|channel_id| *channel_id == new_message.channel_id.0)
    };

    if is_channel_ignored {
        return Ok(());
    }

    if let Err(e) = handle_mention_self(&ctx, &new_message).await {
        log::error!("Failed to reply to self mention: {}", e);
    }

    if let Err(e) = handle_reactions(&ctx, &new_message).await {
        log::error!("Failed to react to the message: {}", e);
    }

    if let Err(e) = handle_responses(&ctx, &new_message).await {
        log::error!("Failed to reply to the message: {}", e);
    }

    if let Err(e) = handle_emote(&ctx, &new_message).await {
        log::error!("Failed to send emote: {}", e);
    }

    Ok(())
}
