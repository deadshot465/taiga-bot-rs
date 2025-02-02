use crate::commands::information::guide::inner_guide;
use crate::shared::structs::ContextData;
use rand::prelude::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn greet(
    ctx: &Context,
    guild: Guild,
    member: &Member,
    data: &ContextData,
) -> anyhow::Result<()> {
    let guild_channels = &guild.channels;
    let greeting_message = {
        let mut rng = rand::rng();
        data.common_settings
            .greetings
            .choose(&mut rng)
            .map(|s| s.replace("{name}", &member.mention().to_string()))
            .unwrap_or_default()
    };

    let general_channels = data
        .config
        .general_channel_ids
        .iter()
        .map(|id| ChannelId::new(*id))
        .collect::<Vec<_>>();

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
    let member = member.clone();
    let data = data.clone();
    tokio::spawn(async move {
        let ctx = ctx_clone;
        if let Err(e) = inner_guide(ctx.clone(), guild, member, data).await {
            tracing::error!("Error occurred when guiding a new user: {}", e);
        }
    });

    Ok(())
}
