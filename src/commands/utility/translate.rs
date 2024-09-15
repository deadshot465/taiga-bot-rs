use poise::CreateReply;
use serenity::all::Attachment;

use crate::shared::services::open_router_service::translate_with_deep_seek;
use crate::shared::structs::{Context, ContextError};

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum LanguageModel {
    #[name = "DeepSeek-v2"]
    DeepSeekV2,
    #[name = "GPT-4o (2024-08-06)"]
    Gpt4o,
    #[name = "Mistral Large"]
    MistralLarge,
}

/// Translate English to traditional Chinese. This is designed for Tetsu's Forged in Starlight.
#[poise::command(slash_command, category = "Utility")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "The document to translate to traditional Chinese."] file: Attachment,
    #[description = "The language model to use to translate. Default to DeepSeek-v2."]
    model: Option<LanguageModel>,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Translating..."))
        .await?;

    let model = model.unwrap_or(LanguageModel::DeepSeekV2);

    let result = translate_with_deep_seek(ctx.data(), &file, model).await?;

    reply_handle
        .edit(ctx, CreateReply::default().content(result))
        .await?;

    Ok(())
}
