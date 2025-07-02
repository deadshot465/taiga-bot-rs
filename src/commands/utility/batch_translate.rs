use crate::commands::utility::translate::{LanguageModel, Novel};
use crate::shared::services::open_router_service::translate_with_model;
use crate::shared::structs::{Context, ContextError};
use poise::CreateReply;
use serenity::all::{Attachment, CreateAttachment};
use tokio::task::JoinSet;

/// Translate English to traditional Chinese. This is designed for Tetsu's Forged in Starlight.
#[poise::command(slash_command, category = "Utility")]
pub async fn batch_translate(
    ctx: Context<'_>,
    #[description = "The novel's title to translate."] novel: Novel,
    #[description = "The document to translate to traditional Chinese."] file: Attachment,
    #[description = "Whether to translate with o1 (High) as well. Default to false."]
    with_o1: Option<bool>,
) -> Result<(), ContextError> {
    ctx.defer().await?;

    let mut join_set = JoinSet::new();
    let with_o1 = with_o1.unwrap_or(false);

    let models = if with_o1 {
        LanguageModel::all()
    } else {
        LanguageModel::all_except_o1()
    };

    for model in models.into_iter() {
        let instructions = match novel {
            Novel::ForgedInStarlight => ctx.data().forged_in_starlight_instructions.clone(),
            Novel::Chronosplit => ctx.data().chronosplit_instructions.clone(),
        };
        let openai_client = ctx.data().openai_client.clone();
        let openai_compatible_clients = ctx.data().openai_compatible_clients.clone();
        let attachment = file.clone();
        join_set.spawn(async move {
            let result = translate_with_model(
                novel,
                instructions,
                openai_client,
                openai_compatible_clients,
                attachment,
                model,
            )
            .await;
            match result {
                Ok(s) => {
                    format!("{model}:\n{s}")
                }
                Err(e) => {
                    format!("Failed to get response using {model}: {e:?}")
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
