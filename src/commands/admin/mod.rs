use crate::shared::structs::config::channel_control::CHANNEL_CONTROL;

use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
    GetMessages,
};
use serenity::model::application::{CommandDataOptionValue, CommandInteraction};
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn dispatch_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    if let Some(opt) = command.data.options.get(0) {
        match opt.name.as_str() {
            "enable" => Box::pin(enable(ctx, command)),
            "disable" => Box::pin(disable(ctx, command)),
            "allow" => Box::pin(allow(ctx, command)),
            "disallow" => Box::pin(disallow(ctx, command)),
            "purge" => Box::pin(purge(ctx, command)),
            _ => Box::pin(enable(ctx, command)),
        }
    } else {
        Box::pin(async move { Ok(()) })
    }
}

async fn enable(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if check_if_channel_id_exists_in_enabled(channel_id).await {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("The channel <#{}> is already enabled!", channel_id)),
                ),
            )
            .await?;
    } else {
        {
            let mut channel_control_write_lock = CHANNEL_CONTROL
                .get()
                .expect("Failed to get channel control")
                .write()
                .await;
            channel_control_write_lock.enabled_channels.push(channel_id);
            channel_control_write_lock.write_channel_control()?;
        }
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Successfully enabled channel <#{}>!", channel_id)),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn disable(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if !check_if_channel_id_exists_in_enabled(channel_id).await {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("The channel <#{}> is not yet enabled!", channel_id)),
                ),
            )
            .await?;
    } else {
        {
            let mut channel_control_write_lock = CHANNEL_CONTROL
                .get()
                .expect("Failed to get channel control")
                .write()
                .await;
            let filtered_channels = channel_control_write_lock
                .enabled_channels
                .iter()
                .filter(|id| **id != channel_id)
                .copied()
                .collect::<Vec<_>>();
            channel_control_write_lock.enabled_channels = filtered_channels;
            channel_control_write_lock.write_channel_control()?;
        }
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("Successfully disabled channel <#{}>!", channel_id)),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn allow(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if !check_if_channel_id_exists_in_ignored(channel_id).await {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(format!(
                        "The channel <#{}> is not yet disallowed!",
                        channel_id
                    )),
                ),
            )
            .await?;
    } else {
        {
            let mut channel_control_write_lock = CHANNEL_CONTROL
                .get()
                .expect("Failed to get channel control")
                .write()
                .await;
            let filtered_channels = channel_control_write_lock
                .ignored_channels
                .iter()
                .filter(|id| **id != channel_id)
                .copied()
                .collect::<Vec<_>>();
            channel_control_write_lock.ignored_channels = filtered_channels;
            channel_control_write_lock.write_channel_control()?;
        }
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(format!(
                        "Successfully allowed channel <#{}> for bot responses!",
                        channel_id
                    )),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn disallow(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if check_if_channel_id_exists_in_ignored(channel_id).await {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(format!(
                        "The channel <#{}> is already disallowed!",
                        channel_id
                    )),
                ),
            )
            .await?;
    } else {
        {
            let mut channel_control_write_lock = CHANNEL_CONTROL
                .get()
                .expect("Failed to get channel control")
                .write()
                .await;
            channel_control_write_lock.ignored_channels.push(channel_id);
            channel_control_write_lock.write_channel_control()?;
        }
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(format!(
                        "Successfully disallowed channel <#{}> for bot responses!",
                        channel_id
                    )),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn purge(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Okay. Hold on..."),
            ),
        )
        .await?;

    let amount = command
        .data
        .options
        .get(0)
        .map(|opt| &opt.value)
        .and_then(|value| value.as_i64())
        .map(|v| if v > 100 { 100 } else { v })
        .unwrap_or(10);

    let guild_channel = ctx
        .cache
        .guild_channels(command.guild_id.unwrap_or_default())
        .and_then(|channels| channels.get(&command.channel_id).cloned());
    if let Some(channel) = guild_channel {
        let sent_msg = command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content("Retrieving messages..."),
            )
            .await?;

        let messages = channel
            .messages(
                &ctx.http,
                GetMessages::new().limit(amount as u8).before(sent_msg.id),
            )
            .await?;

        command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content(format!(
                    "The last {} messages in this channel will be deleted in 5 seconds.",
                    amount
                )),
            )
            .await?;

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        channel.delete_messages(&ctx.http, messages).await?;
        command.delete_response(&ctx.http).await?;
    }

    Ok(())
}

fn extract_channel_id(command: &CommandInteraction) -> u64 {
    command
        .data
        .options
        .get(0)
        .map(|opt| opt.value)
        .map(|resolved| {
            if let CommandDataOptionValue::Channel(channel_id) = resolved {
                channel_id.get()
            } else {
                0
            }
        })
        .unwrap_or_default()
}

async fn check_if_channel_id_exists_in_enabled(channel_id: u64) -> bool {
    CHANNEL_CONTROL
        .get()
        .expect("Failed to get channel control")
        .read()
        .await
        .enabled_channels
        .contains(&channel_id)
}

async fn check_if_channel_id_exists_in_ignored(channel_id: u64) -> bool {
    CHANNEL_CONTROL
        .get()
        .expect("Failed to get channel control")
        .read()
        .await
        .ignored_channels
        .contains(&channel_id)
}
