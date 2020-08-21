use serenity::{
    framework::standard::{
        macros::{
            command
        }, CommandResult
    },
    model::channel::Message,
    model::prelude::*,
    prelude::*
};

#[command]
#[description = "Start a step-by-step guide."]
#[usage = ""]
#[example = ""]
#[bucket = "information"]
async fn guide(context: &Context, msg: &Message) -> CommandResult {
    let http = &context.http;
    let guild = context.cache.guild(msg.guild_id.as_ref().unwrap())
        .await.expect("Failed to retrieve guild information.");
    let member = guild.members.get(&msg.author.id).expect("Failed to retrieve member information.");
    let command: serenity::framework::standard::Command;
    Ok(())
}