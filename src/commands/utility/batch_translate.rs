use crate::commands::utility::translate::LanguageModel;
use crate::shared::services::open_router_service::translate_with_model;
use crate::shared::structs::{Context, ContextError};
use poise::CreateReply;
use serenity::all::{Attachment, CreateAttachment};

/// Translate English to traditional Chinese. This is designed for Tetsu's Forged in Starlight.
#[poise::command(slash_command, category = "Utility")]
pub async fn batch_translate(
    ctx: Context<'_>,
    #[description = "The document to translate to traditional Chinese."] file: Attachment,
) -> Result<(), ContextError> {
    ctx.defer().await?;

    let mut results = Vec::new();

    for model in LanguageModel::all().iter() {
        let result = translate_with_model(ctx.data(), &file, *model).await?;
        results.push(format!("{}:\n{}", model, result));
    }

    let response = results.join("\n--------------------\n");
    ctx.send(
        CreateReply::default()
            .attachment(CreateAttachment::bytes(response.as_bytes(), "result.txt")),
    )
    .await?;

    Ok(())
}
