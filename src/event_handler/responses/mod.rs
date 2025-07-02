use crate::event_handler::responses::emote::handle_emote;
use crate::event_handler::responses::mention::handle_mention_self;
use crate::event_handler::responses::reaction::handle_reactions;
use crate::event_handler::responses::response::handle_responses;
use crate::shared::services::open_router_service::build_reply_to_message_chain;
use crate::shared::structs::ContextData;
use serenity::all::{GuildChannel, PrivateChannel};
use serenity::model::prelude::Message;
use serenity::prelude::*;

pub mod certify;
pub mod emote;
pub mod greet;
pub mod mention;
pub mod qotd;
pub mod reaction;
pub mod response;

pub async fn handle_bot_responses(
    ctx: &Context,
    new_message: &Message,
    data: &ContextData,
) -> anyhow::Result<()> {
    if let Err(e) = handle_emote(ctx, new_message, data).await {
        tracing::error!("Failed to send emote: {}", e);
    }

    if let Err(e) = handle_mention_self(ctx, new_message, data).await {
        tracing::error!("Failed to reply to self mention: {}", e);
    }

    if let Some(original_message) = new_message.referenced_message.as_ref() {
        if original_message.author.id.get() == data.config.bot_id {
            let channel = new_message.channel(&ctx.http).await?;
            let guild_channel = channel.clone().guild();
            let private_channel = channel.private();

            let mut message_chain = build_message_chain(
                ctx,
                new_message.clone(),
                guild_channel,
                private_channel,
                vec![],
            )
            .await?;

            message_chain.reverse();
            let mut built_message_chain = vec![];
            let bot_user = ctx.http.get_current_user().await?;
            let bot_nick = bot_user.name.clone();

            for message in message_chain.into_iter() {
                let author_nick = if message.author.id == bot_user.id {
                    bot_nick.clone()
                } else {
                    message.author.name.clone()
                };

                built_message_chain.push(format!("{}: {}", author_nick, message.content));
            }

            match build_reply_to_message_chain(data, built_message_chain, bot_nick).await {
                Ok(response) => {
                    new_message.reply(&ctx.http, response).await?;
                }
                Err(e) => {
                    let error_message = format!("Failed to reply to message chain: {e:?}");
                    tracing::error!("{}", &error_message);
                    new_message.reply(&ctx.http, error_message).await?;
                }
            }

            return Ok(());
        }
    }

    let channel_control = data.channel_control.clone();

    let is_channel_ignored = {
        channel_control
            .read()
            .await
            .ignored_channels
            .iter()
            .any(|channel_id| *channel_id == new_message.channel_id.get())
    };

    if is_channel_ignored {
        return Ok(());
    }

    if let Err(e) = handle_reactions(ctx, new_message, data).await {
        tracing::error!("Failed to react to the message: {}", e);
    }

    if let Err(e) = handle_responses(ctx, new_message, data).await {
        tracing::error!("Failed to reply to the message: {}", e);
    }

    Ok(())
}

async fn build_message_chain(
    ctx: &Context,
    original_message: Message,
    maybe_guild_channel: Option<GuildChannel>,
    maybe_private_channel: Option<PrivateChannel>,
    mut message_chain: Vec<Message>,
) -> anyhow::Result<Vec<Message>> {
    let message = original_message.clone();
    message_chain.push(message);

    if let Some(old_message) = original_message.referenced_message.as_ref() {
        let old_message_id = old_message.id;
        let old_message = if let Some(ref guild_channel) = maybe_guild_channel {
            guild_channel.message(&ctx.http, old_message_id).await?
        } else {
            let private_channel = maybe_private_channel
                .clone()
                .expect("Failed to get private channel for the message.");
            private_channel.message(&ctx.http, old_message_id).await?
        };

        let chain = Box::pin(build_message_chain(
            ctx,
            old_message,
            maybe_guild_channel,
            maybe_private_channel,
            message_chain,
        ))
        .await?;

        Ok(chain)
    } else {
        Ok(message_chain)
    }
}
