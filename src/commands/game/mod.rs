use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub mod quiz;

pub fn dispatch_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    if let Some(opt) = command.data.options.get(0) {
        match opt.name.as_str() {
            "quiz" => quiz::quiz_async(ctx, command),
            _ => quiz::quiz_async(ctx, command),
        }
    } else {
        Box::pin(async move { Ok(()) })
    }
}
