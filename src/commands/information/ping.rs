use crate::shared::structs::config::configuration::KOU;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

pub fn ping_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(ping(ctx, command))
}

async fn ping(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);

    let starting_msg = if is_kou {
        "<:KouBrave:705182851397845193> Pinging..."
    } else {
        "🏓 Pinging..."
    };

    let ending_msg = if is_kou {
        "<:KouPoint:717505202651136051> Pong!\nLatency is: {latency}ms."
    } else {
        "🏓 Pong!\nLatency is: {latency}ms."
    };

    let original_time = Instant::now();
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content(starting_msg))
        })
        .await?;
    let current_time = Instant::now();
    let elapsed = current_time.duration_since(original_time);
    let ending_msg = ending_msg.replace("{latency}", &elapsed.as_millis().to_string());
    command
        .edit_original_interaction_response(&ctx.http, |data| data.content(ending_msg))
        .await?;
    Ok(())
}
