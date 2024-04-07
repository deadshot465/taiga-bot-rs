use std::time::Instant;

use poise::CreateReply;

use crate::shared::structs::{Context, ContextError};

/// Returns latency and API ping.
#[poise::command(slash_command, category = "Information")]
pub async fn ping(ctx: Context<'_>) -> Result<(), ContextError> {
    let is_kou = ctx.data().kou;

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
    let reply_handle = ctx
        .send(CreateReply::default().content(starting_msg))
        .await?;
    let current_time = Instant::now();
    let elapsed = current_time.duration_since(original_time);
    let ending_msg = ending_msg.replace("{latency}", &elapsed.as_millis().to_string());
    reply_handle
        .edit(ctx, CreateReply::default().content(ending_msg))
        .await?;
    Ok(())
}
