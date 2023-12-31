use serenity::model::application::CommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub mod hangman;
pub mod quiz;

pub fn dispatch_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    if let Some(opt) = command.data.options.get(0) {
        match opt.name.as_str() {
            "quiz" => quiz::quiz_async(ctx, command),
            "hangman" => hangman::hangman_async(ctx, command),
            _ => quiz::quiz_async(ctx, command),
        }
    } else {
        Box::pin(async move { Ok(()) })
    }
}
