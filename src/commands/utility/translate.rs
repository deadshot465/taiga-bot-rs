use poise::CreateReply;
use serenity::all::Attachment;
use std::fmt::{Display, Formatter};

use crate::shared::services::open_router_service::translate_with_model;
use crate::shared::structs::{Context, ContextError};

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum LanguageModel {
    #[name = "DeepSeek-v2"]
    DeepSeekV25,
    #[name = "GPT-4o (2024-08-06)"]
    Gpt4o,
    #[name = "Mistral Large"]
    MistralLarge,
    #[name = "Qwen 2.5 72B Instruct"]
    Qwen2572BInstruct,
    #[name = "Cohere Command R+ (08-2024)"]
    CohereCommandRPlus082024,
    #[name = "Grok 2"]
    Grok2,
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

    let model = model.unwrap_or(LanguageModel::DeepSeekV25);
    let instructions = ctx.data().translation_instructions.clone();
    let client = ctx.data().open_router_client.clone();

    let result = translate_with_model(instructions, client, file, model).await?;

    reply_handle
        .edit(ctx, CreateReply::default().content(result))
        .await?;

    Ok(())
}

impl LanguageModel {
    pub fn all() -> Vec<Self> {
        vec![
            LanguageModel::DeepSeekV25,
            LanguageModel::Gpt4o,
            LanguageModel::MistralLarge,
            LanguageModel::Qwen2572BInstruct,
            LanguageModel::CohereCommandRPlus082024,
            LanguageModel::Grok2
        ]
    }
}

impl Display for LanguageModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <LanguageModel as Into<String>>::into(*self))
    }
}

impl From<LanguageModel> for String {
    fn from(value: LanguageModel) -> Self {
        match value {
            LanguageModel::DeepSeekV25 => "DeepSeek v2.5".into(),
            LanguageModel::Gpt4o => "GPT-4o".into(),
            LanguageModel::MistralLarge => "Mistral Large".into(),
            LanguageModel::Qwen2572BInstruct => "Qwen 2.5 72B Instruct".into(),
            LanguageModel::CohereCommandRPlus082024 => "Cohere Command R+ (08-2024)".into(),
            LanguageModel::Grok2 => "Grok 2".into()
        }
    }
}
