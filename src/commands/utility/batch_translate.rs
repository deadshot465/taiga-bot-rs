use crate::commands::utility::translate::LanguageModel;
use crate::shared::services::open_router_service::translate_with_model;
use crate::shared::structs::{Context, ContextError};
use poise::CreateReply;
use serenity::all::{Attachment, CreateAttachment};
use tokio::task::JoinSet;

/// Translate English to traditional Chinese. This is designed for Tetsu's Forged in Starlight.
#[poise::command(slash_command, category = "Utility")]
pub async fn batch_translate(
    ctx: Context<'_>,
    #[description = "The document to translate to traditional Chinese."] file: Attachment,
) -> Result<(), ContextError> {
    ctx.defer().await?;

    let mut join_set = JoinSet::new();

    for model in LanguageModel::all().into_iter() {
        let instructions = ctx.data().translation_instructions.clone();
        let client = ctx.data().open_router_client.clone();
        let attachment = file.clone();
        join_set.spawn(async move {
            let result = translate_with_model(instructions, client, attachment, model).await;
            match result {
                Ok(s) => {
                    format!("{}:\n{}", model, s)
                }
                Err(e) => {
                    format!("Failed to get response using {}: {}", model, e)
                }
            }
        });
    }

    let results = join_set.join_all().await;

    let response = results.join("\n--------------------\n");
    ctx.send(
        CreateReply::default()
            .attachment(CreateAttachment::bytes(response.as_bytes(), "result.txt")),
    )
    .await?;

    Ok(())
}
