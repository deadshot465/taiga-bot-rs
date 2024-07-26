use poise::CreateReply;
use serenity::all::Attachment;

use crate::shared::services::open_router_service::translate_with_deep_seek;
use crate::shared::structs::{Context, ContextError};

#[poise::command(slash_command, category = "Utility")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "The document to translate to traditional Chinese."] file: Attachment,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Translating..."))
        .await?;

    let result = translate_with_deep_seek(ctx.data(), &file).await?;

    reply_handle
        .edit(ctx, CreateReply::default().content(result))
        .await?;

    Ok(())
}
