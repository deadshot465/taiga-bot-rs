use crate::commands::information::guide::inner_guide;
use crate::shared::structs::config::common_settings::COMMON_SETTINGS;
use crate::shared::structs::config::configuration::CONFIGURATION;
use rand::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn greet(ctx: &Context, guild: Guild, member: Member) -> anyhow::Result<()> {
    let guild_channels = &guild.channels;
    let greeting_message = {
        let mut rng = thread_rng();
        COMMON_SETTINGS
            .greetings
            .choose(&mut rng)
            .map(|s| s.replace("{name}", &member.mention().to_string()))
            .unwrap_or_default()
    };

    let general_channels = CONFIGURATION
        .get()
        .map(|c| &c.general_channel_ids)
        .map(|ids| ids.iter().map(|id| ChannelId::new(*id)).collect::<Vec<_>>())
        .unwrap_or_default();

    for general_channel_id in general_channels.into_iter() {
        if let Some((channel_id, _)) = guild_channels
            .iter()
            .find(|(channel_id, _)| **channel_id == general_channel_id)
        {
            channel_id.say(&ctx.http, greeting_message).await?;
            break;
        }
    }

    let ctx_clone = ctx.clone();
    let guild = guild.clone();
    tokio::spawn(async move {
        let ctx = ctx_clone;
        if let Err(e) = inner_guide(&ctx, guild, member).await {
            tracing::error!("Error occurred when guiding a new user: {}", e);
        }
    });

    Ok(())
}
