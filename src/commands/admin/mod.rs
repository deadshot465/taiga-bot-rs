use crate::shared::structs::config::channel_control::CHANNEL_CONTROL;
use serenity::model::prelude::application_command::*;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn dispatch_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
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

async fn enable(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if check_if_channel_id_exists_in_enabled(channel_id).await {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!("The channel <#{}> is already enabled!", channel_id))
                })
            })
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
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!("Successfully enabled channel <#{}>!", channel_id))
                })
            })
            .await?;
    }

    Ok(())
}

async fn disable(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if !check_if_channel_id_exists_in_enabled(channel_id).await {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!("The channel <#{}> is not yet enabled!", channel_id))
                })
            })
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
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!("Successfully disabled channel <#{}>!", channel_id))
                })
            })
            .await?;
    }

    Ok(())
}

async fn allow(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if !check_if_channel_id_exists_in_ignored(channel_id).await {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!(
                        "The channel <#{}> is not yet disallowed!",
                        channel_id
                    ))
                })
            })
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
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!(
                        "Successfully allowed channel <#{}> for bot responses!",
                        channel_id
                    ))
                })
            })
            .await?;
    }

    Ok(())
}

async fn disallow(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let channel_id = extract_channel_id(&command);

    if check_if_channel_id_exists_in_ignored(channel_id).await {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!(
                        "The channel <#{}> is already disallowed!",
                        channel_id
                    ))
                })
            })
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
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(format!(
                        "Successfully disallowed channel <#{}> for bot responses!",
                        channel_id
                    ))
                })
            })
            .await?;
    }

    Ok(())
}

async fn purge(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Okay. Hold on..."))
        })
        .await?;

    let amount = command
        .data
        .options
        .get(0)
        .and_then(|opt| opt.options.get(0))
        .and_then(|opt| opt.value.as_ref())
        .and_then(|value| value.as_u64())
        .map(|v| if v > 100 { 100 } else { v })
        .unwrap_or(10);

    if let Some(channel) = ctx.cache.guild_channel(command.channel_id) {
        let sent_msg = command
            .edit_original_interaction_response(&ctx.http, |response| {
                response.content("Retrieving messages...")
            })
            .await?;

        let messages = channel
            .messages(&ctx.http, |msg| msg.limit(amount).before(sent_msg.id))
            .await?;

        command
            .edit_original_interaction_response(&ctx.http, |response| {
                response.content(format!(
                    "The last {} messages in this channel will be deleted in 5 seconds.",
                    amount
                ))
            })
            .await?;

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        channel.delete_messages(&ctx.http, messages).await?;
        command
            .delete_original_interaction_response(&ctx.http)
            .await?;
    }

    Ok(())
}

fn extract_channel_id(command: &ApplicationCommandInteraction) -> u64 {
    command
        .data
        .options
        .get(0)
        .and_then(|opt| opt.options.get(0))
        .and_then(|opt| opt.resolved.as_ref())
        .map(|resolved| {
            if let ApplicationCommandInteractionDataOptionValue::Channel(channel) = resolved {
                channel.id.0
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
