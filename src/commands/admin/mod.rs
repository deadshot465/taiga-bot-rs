use std::borrow::Cow;
use std::sync::Arc;

use poise::CreateReply;
use serenity::all::{Channel, GetMessages, PrivateChannel};
use tokio::sync::RwLock;

use crate::shared::structs::config::channel_control::ChannelControl;
use crate::shared::structs::{Context, ContextError};

/// Administrative commands.
#[poise::command(
    slash_command,
    subcommands("enable", "disable", "allow", "disallow", "purge"),
    subcommand_required
)]
pub async fn admin(_: Context<'_>) -> Result<(), ContextError> {
    Ok(())
}

/// Enable a specific channel for bot usage.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Admin"
)]
pub async fn enable(
    ctx: Context<'_>,
    #[description = "The channel to enable for bot usage."] channel: Channel,
) -> Result<(), ContextError> {
    let channel_id = channel.id().get();
    let channel_control = ctx.data().channel_control.clone();

    if check_if_channel_id_exists_in_enabled(&channel_control, channel_id).await {
        ctx.send(
            CreateReply::default()
                .content(format!("The channel <#{channel_id}> is already enabled!")),
        )
        .await?;
    } else {
        {
            let mut channel_control_write_lock = channel_control.write().await;
            channel_control_write_lock.enabled_channels.push(channel_id);
            channel_control_write_lock.write_channel_control()?;
        }
        ctx.send(
            CreateReply::default()
                .content(format!("Successfully enabled channel <#{channel_id}>!")),
        )
        .await?;
    }

    Ok(())
}

/// Disable a specific channel for bot usage.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn disable(
    ctx: Context<'_>,
    #[description = "The channel to disable for bot usage."] channel: Channel,
) -> Result<(), ContextError> {
    let channel_id = channel.id().get();
    let channel_control = ctx.data().channel_control.clone();

    if !check_if_channel_id_exists_in_enabled(&channel_control, channel_id).await {
        ctx.send(
            CreateReply::default()
                .content(format!("The channel <#{channel_id}> is not yet enabled!")),
        )
        .await?;
    } else {
        {
            let mut channel_control_write_lock = channel_control.write().await;
            let filtered_channels = channel_control_write_lock
                .enabled_channels
                .iter()
                .filter(|id| **id != channel_id)
                .copied()
                .collect::<Vec<_>>();
            channel_control_write_lock.enabled_channels = filtered_channels;
            channel_control_write_lock.write_channel_control()?;
        }
        ctx.send(
            CreateReply::default()
                .content(format!("Successfully disabled channel <#{channel_id}>!")),
        )
        .await?;
    }

    Ok(())
}

/// Allow a specific channel for random responses of bot.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn allow(
    ctx: Context<'_>,
    #[description = "The channel to allow for random responses."] channel: Channel,
) -> Result<(), ContextError> {
    let channel_id = channel.id().get();
    let channel_control = ctx.data().channel_control.clone();

    if !check_if_channel_id_exists_in_ignored(&channel_control, channel_id).await {
        ctx.send(CreateReply::default().content(format!(
            "The channel <#{channel_id}> is not yet disallowed!"
        )))
        .await?;
    } else {
        {
            let mut channel_control_write_lock = channel_control.write().await;
            let filtered_channels = channel_control_write_lock
                .ignored_channels
                .iter()
                .filter(|id| **id != channel_id)
                .copied()
                .collect::<Vec<_>>();
            channel_control_write_lock.ignored_channels = filtered_channels;
            channel_control_write_lock.write_channel_control()?;
        }
        ctx.send(CreateReply::default().content(format!(
            "Successfully allowed channel <#{channel_id}> for bot responses!"
        )))
        .await?;
    }

    Ok(())
}

/// Disallow a specific channel for random responses of bot.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn disallow(
    ctx: Context<'_>,
    #[description = "The channel to disallow for random responses."] channel: Channel,
) -> Result<(), ContextError> {
    let channel_id = channel.id().get();
    let channel_control = ctx.data().channel_control.clone();

    if check_if_channel_id_exists_in_ignored(&channel_control, channel_id).await {
        ctx.send(CreateReply::default().content(format!(
            "The channel <#{channel_id}> is already disallowed!"
        )))
        .await?;
    } else {
        {
            let mut channel_control_write_lock = channel_control.write().await;
            channel_control_write_lock.ignored_channels.push(channel_id);
            channel_control_write_lock.write_channel_control()?;
        }
        ctx.send(CreateReply::default().content(format!(
            "Successfully disallowed channel <#{channel_id}> for bot responses!"
        )))
        .await?;
    }

    Ok(())
}

/// Purge messages from this channel. Default to 10 most recent messages. Maximum 100 messages.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "The amount of messages to purge."] amount: Option<i32>,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Okay. Hold on..."))
        .await?;

    let guild_id = ctx.guild_id().unwrap_or_default();
    let channel_id = ctx.channel_id();
    let guild_channel = ctx
        .cache()
        .guild(guild_id)
        .and_then(|g| g.channels.get(&channel_id).cloned());

    reply_handle
        .edit(
            ctx,
            CreateReply::default().content("Retrieving messages..."),
        )
        .await?;

    let amount = amount.unwrap_or(10);

    let sent_message = reply_handle.message().await?;
    let sent_message = match sent_message {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    };

    let mut dm_channel: Option<PrivateChannel> = None;

    let messages = if let Some(ref guild_channel) = guild_channel {
        guild_channel
            .messages(
                ctx.http(),
                GetMessages::new()
                    .limit(amount as u8)
                    .before(sent_message.id),
            )
            .await?
    } else {
        let channel = ctx.author().create_dm_channel(ctx.http()).await?;
        let messages = channel
            .messages(
                ctx.http(),
                GetMessages::new()
                    .limit(amount as u8)
                    .before(sent_message.id),
            )
            .await?;
        dm_channel = Some(channel);
        messages
    };

    reply_handle
        .edit(
            ctx,
            CreateReply::default().content(format!(
                "The last {amount} messages in this channel will be deleted in 5 seconds."
            )),
        )
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    if let Some(guild_channel) = guild_channel {
        guild_channel.delete_messages(ctx.http(), messages).await?;
    } else if let Some(dm_channel) = dm_channel {
        dm_channel.delete_messages(ctx.http(), messages).await?;
    }

    reply_handle.delete(ctx).await?;

    Ok(())
}

async fn check_if_channel_id_exists_in_enabled(
    channel_control: &Arc<RwLock<ChannelControl>>,
    channel_id: u64,
) -> bool {
    channel_control
        .read()
        .await
        .enabled_channels
        .contains(&channel_id)
}

async fn check_if_channel_id_exists_in_ignored(
    channel_control: &Arc<RwLock<ChannelControl>>,
    channel_id: u64,
) -> bool {
    channel_control
        .read()
        .await
        .ignored_channels
        .contains(&channel_id)
}
