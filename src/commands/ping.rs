use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::time::Instant;

#[command]
pub fn ping(context: &mut Context, msg: &Message) -> CommandResult {
    let original_time = Instant::now();
    let ping_msg = msg.channel_id.say(&context.http, "🏓 Pinging...");
    if let Err(e) = ping_msg {
        eprintln!("An error occurred when pinging: {:?}", e);
    }
    else {
        let current_time = Instant::now();
        let latency = current_time.duration_since(original_time);
        ping_msg.unwrap().edit(&context, |m| {
            m.content(format!("🏓 Pong!\nLatency is: {}ms.", latency.as_millis()))
        })?;
    }

    Ok(())
}