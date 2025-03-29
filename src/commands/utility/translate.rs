use poise::CreateReply;
use serenity::all::Attachment;
use std::fmt::{Display, Formatter};

use crate::shared::services::open_router_service::translate_with_model;
use crate::shared::structs::{Context, ContextError};

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum LanguageModel {
    #[name = "DeepSeek-v3-0324"]
    DeepSeekV3,
    #[name = "GPT-4o (2024-11-20)"]
    Gpt4o,
    #[name = "Mistral Large (2411)"]
    MistralLarge,
    #[name = "Qwen-Max"]
    QwenMax,
    #[name = "Cohere Command A"]
    CohereCommandA,
    #[name = "Grok 2 (1212)"]
    Grok2,
    #[name = "DeepSeek R1"]
    DeepSeekR1,
    #[name = "Gemini 2.0 Flash"]
    Gemini2Flash,
    #[name = "MiniMax-01"]
    MiniMax01,
    #[name = "o3-mini (High)"]
    O3MiniHigh,
    #[name = "o1 (High)"]
    O1High,
    #[name = "Amazon Nova Pro 1.0"]
    NovaPro,
    #[name = "Gemini Pro 2.5 Experimental"]
    Gemini25ProExperimental,
    #[name = "Doubao 1.5 Pro 256k"]
    Doubao15Pro256k,
    #[name = "Kimi Latest"]
    KimiLatest,
    #[name = "Step 2 16k"]
    Step16k,
    #[name = "GLM-4-Plus"]
    Glm4Plus,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum Novel {
    #[name = "Forged in Starlight"]
    ForgedInStarlight,
    Chronosplit,
}

/// Translate English to traditional Chinese. This is designed for Tetsu's Forged in Starlight.
#[poise::command(slash_command, category = "Utility")]
pub async fn translate(
    ctx: Context<'_>,
    #[description = "The novel's title to translate."] novel: Novel,
    #[description = "The document to translate to traditional Chinese."] file: Attachment,
    #[description = "The language model to use to translate. Default to DeepSeek-v3."]
    model: Option<LanguageModel>,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Translating..."))
        .await?;

    let model = model.unwrap_or(LanguageModel::DeepSeekV3);
    let instructions = match novel {
        Novel::ForgedInStarlight => ctx.data().forged_in_starlight_instructions.clone(),
        Novel::Chronosplit => ctx.data().chronosplit_instructions.clone(),
    };
    let openai_client = ctx.data().openai_client.clone();
    let openai_compatible_clients = ctx.data().openai_compatible_clients.clone();

    let result = translate_with_model(
        novel,
        instructions,
        openai_client,
        openai_compatible_clients,
        file,
        model,
    )
    .await?;

    reply_handle
        .edit(ctx, CreateReply::default().content(result))
        .await?;

    Ok(())
}

impl LanguageModel {
    pub fn all() -> Vec<Self> {
        vec![
            LanguageModel::DeepSeekV3,
            LanguageModel::Gpt4o,
            LanguageModel::MistralLarge,
            LanguageModel::QwenMax,
            LanguageModel::CohereCommandA,
            LanguageModel::Grok2,
            LanguageModel::DeepSeekR1,
            LanguageModel::Gemini2Flash,
            LanguageModel::MiniMax01,
            LanguageModel::O3MiniHigh,
            LanguageModel::O1High,
            LanguageModel::NovaPro,
            LanguageModel::Gemini25ProExperimental,
            LanguageModel::Doubao15Pro256k,
            LanguageModel::KimiLatest,
            LanguageModel::Step16k,
            LanguageModel::Glm4Plus,
        ]
    }

    pub fn all_except_o1() -> Vec<Self> {
        Self::all()
            .into_iter()
            .filter(|model| *model != LanguageModel::O1High)
            .collect()
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
            LanguageModel::DeepSeekV3 => "DeepSeek v3 0324".into(),
            LanguageModel::Gpt4o => "GPT-4o (2024-11-20)".into(),
            LanguageModel::MistralLarge => "Mistral Large (2411)".into(),
            LanguageModel::QwenMax => "Qwen-Max".into(),
            LanguageModel::CohereCommandA => "Cohere Command A".into(),
            LanguageModel::Grok2 => "Grok 2 (1212)".into(),
            LanguageModel::DeepSeekR1 => "DeepSeek R1".into(),
            LanguageModel::Gemini2Flash => "Gemini 2.0 Flash".into(),
            LanguageModel::MiniMax01 => "MiniMax-01".into(),
            LanguageModel::O3MiniHigh => "o3-mini (High)".into(),
            LanguageModel::O1High => "o1 (High)".into(),
            LanguageModel::NovaPro => "Amazon Nova Pro 1.0".into(),
            LanguageModel::Gemini25ProExperimental => "Gemini Pro 2.5 Experimental".into(),
            LanguageModel::Doubao15Pro256k => "Doubao 1.5 Pro 256k".into(),
            LanguageModel::KimiLatest => "Kimi Latest".into(),
            LanguageModel::Step16k => "Step 2 16k".into(),
            LanguageModel::Glm4Plus => "GLM-4-Plus".into(),
        }
    }
}
