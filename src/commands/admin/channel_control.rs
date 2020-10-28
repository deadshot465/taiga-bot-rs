use crate::PersistenceService;
use regex::Regex;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::{GuildChannel, Message};
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::Duration;

lazy_static! {
    static ref CHANNEL_REGEX: Regex = Regex::new(r"<#(\d+)>").unwrap();
}

enum ProcessType {
    Allow,
    Disable,
    Enable,
    Ignore,
}

#[command]
#[description = "Allow a specific channel for bot responses."]
#[usage = "<channel-tag>"]
#[example = " <#123456789012345678>"]
pub async fn allow(context: &Context, msg: &Message) -> CommandResult {
    let regex = &*CHANNEL_REGEX;
    let channels = regex
        .captures_iter(&msg.content)
        .map(|m| m.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    process(context, msg, channels, &ProcessType::Allow).await?;
    Ok(())
}

#[command]
#[description = "Disable a specific channel."]
#[usage = "<channel-tag>"]
#[example = " <#123456789012345678>"]
pub async fn disable(context: &Context, msg: &Message) -> CommandResult {
    let regex = &*CHANNEL_REGEX;
    let channels = regex
        .captures_iter(&msg.content)
        .map(|m| m.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    process(context, msg, channels, &ProcessType::Disable).await?;
    Ok(())
}

#[command]
#[description = "Enable a specific channel."]
#[usage = "<channel-tag>"]
#[example = " <#123456789012345678>"]
pub async fn enable(context: &Context, msg: &Message) -> CommandResult {
    let regex = &*CHANNEL_REGEX;
    let channels = regex
        .captures_iter(&msg.content)
        .map(|m| m.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    process(context, msg, channels, &ProcessType::Enable).await?;
    Ok(())
}

#[command]
#[description = "Disallow a specific channel for bot responses."]
#[usage = "<channel-tag>"]
#[example = " <#123456789012345678>"]
pub async fn ignore(context: &Context, msg: &Message) -> CommandResult {
    let regex = &*CHANNEL_REGEX;
    let channels = regex
        .captures_iter(&msg.content)
        .map(|m| m.get(1).unwrap().as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    process(context, msg, channels, &ProcessType::Ignore).await?;
    Ok(())
}

#[command]
#[description = "Bulk-delete the most recent messages. Default (and maximum) to 100 messages"]
#[usage = "<number-of-messages-to-delete>"]
#[example = "50"]
pub async fn purge(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let arg = args.single::<u64>();
    let mut amount = 100;
    if let Ok(n) = arg {
        amount = n;
    }
    let channels: HashMap<ChannelId, GuildChannel> =
        msg.guild_id.unwrap().channels(&context.http).await?;
    let channel = channels.get(&msg.channel_id).unwrap();
    let messages: Vec<Message> = channel
        .messages(&context.http, |f| f.limit(amount).before(&msg.id))
        .await?;
    let notice_msg = msg
        .channel_id
        .say(
            &context.http,
            format!(
                "The last {} messages in this channel will be deleted.",
                &amount
            ),
        )
        .await?;
    tokio::time::delay_for(Duration::from_secs(5)).await;
    msg.channel_id
        .delete_messages(&context.http, messages)
        .await?;
    notice_msg.delete(&context.http).await?;
    msg.delete(&context.http).await?;
    Ok(())
}

async fn process(
    context: &Context,
    msg: &Message,
    channels: Vec<u64>,
    process_type: &ProcessType,
) -> CommandResult {
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    drop(data);
    let mut persistence_lock = _persistence.write().await;
    let channel_settings = persistence_lock.channel_settings.as_mut().unwrap();
    for chn in channels.iter() {
        match process_type {
            ProcessType::Allow => {
                if !channel_settings.ignored_channels.contains(chn) {
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("The channel <#{}> is not ignored.", chn).as_str(),
                        )
                        .await?;
                } else {
                    channel_settings.ignored_channels.remove(&*chn);
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("The channel <#{}> is now allowed for bot responses.", chn)
                                .as_str(),
                        )
                        .await?;
                }
            }
            ProcessType::Disable => {
                if !channel_settings.enabled_channels.contains(chn) {
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("The channel <#{}> is not enabled.", chn).as_str(),
                        )
                        .await?;
                } else {
                    channel_settings.enabled_channels.remove(&*chn);
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("Successfully disabled <#{}> channel.", chn).as_str(),
                        )
                        .await?;
                }
            }
            ProcessType::Enable => {
                if channel_settings.enabled_channels.contains(chn) {
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("The channel <#{}> is already enabled.", chn).as_str(),
                        )
                        .await?;
                } else {
                    channel_settings.enabled_channels.insert(*chn);
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("Successfully enabled <#{}> channel.", chn).as_str(),
                        )
                        .await?;
                }
            }
            ProcessType::Ignore => {
                if channel_settings.ignored_channels.contains(chn) {
                    msg.channel_id
                        .say(
                            &context.http,
                            format!("The channel <#{}> is already ignored.", chn).as_str(),
                        )
                        .await?;
                } else {
                    channel_settings.ignored_channels.insert(*chn);
                    msg.channel_id
                        .say(
                            &context.http,
                            format!(
                                "The channel <#{}> is now disallowed for bot responses.",
                                chn
                            )
                            .as_str(),
                        )
                        .await?;
                }
            }
        }
    }
    drop(persistence_lock);
    Ok(())
}
