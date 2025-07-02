use poise::CreateReply;
use serenity::all::Attachment;
use std::fmt::{Display, Formatter};

use crate::shared::services::open_router_service::translate_with_model;
use crate::shared::structs::{Context, ContextError};

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum LanguageModel {
    #[name = "DeepSeek-v3-0324"]
    DeepSeekV3,
    #[name = "GPT-4.1"]
    Gpt41,
    #[name = "Mistral Large (2411)"]
    MistralLarge,
    #[name = "Qwen-Max"]
    QwenMax,
    #[name = "Cohere Command A"]
    CohereCommandA,
    #[name = "Grok 3"]
    Grok3,
    #[name = "DeepSeek R1"]
    DeepSeekR1,
    #[name = "Gemini 2.5 Flash"]
    Gemini25Flash,
    #[name = "MiniMax-M1"]
    MiniMaxM1,
    #[name = "o3"]
    O3,
    #[name = "Amazon Nova Pro 1.0"]
    NovaPro,
    #[name = "Gemini 2.5 Pro"]
    Gemini25Pro,
    #[name = "Doubao Seed 1.6"]
    DoubaoSeed16,
    #[name = "Kimi Latest"]
    KimiLatest,
    #[name = "Step 2 16k"]
    Step16k,
    #[name = "GLM-4-Plus"]
    Glm4Plus,
    #[name = "Claude Opus 4"]
    Opus4,
    #[name = "Claude Sonnet 4"]
    Sonnet4,
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
            LanguageModel::Gpt41,
            LanguageModel::MistralLarge,
            LanguageModel::QwenMax,
            LanguageModel::CohereCommandA,
            LanguageModel::Grok3,
            LanguageModel::DeepSeekR1,
            LanguageModel::Gemini25Flash,
            LanguageModel::MiniMaxM1,
            LanguageModel::O3,
            LanguageModel::NovaPro,
            LanguageModel::Gemini25Pro,
            LanguageModel::DoubaoSeed16,
            LanguageModel::KimiLatest,
            LanguageModel::Step16k,
            LanguageModel::Glm4Plus,
            LanguageModel::Opus4,
            LanguageModel::Sonnet4,
        ]
    }

    pub fn all_except_o1() -> Vec<Self> {
        Self::all()
            .into_iter()
            .filter(|model| *model != LanguageModel::O3)
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
            LanguageModel::Gpt41 => "GPT-4.1".into(),
            LanguageModel::MistralLarge => "Mistral Large (2411)".into(),
            LanguageModel::QwenMax => "Qwen-Max".into(),
            LanguageModel::CohereCommandA => "Cohere Command A".into(),
            LanguageModel::Grok3 => "Grok 3".into(),
            LanguageModel::DeepSeekR1 => "DeepSeek R1".into(),
            LanguageModel::Gemini25Flash => "Gemini 2.5 Flash".into(),
            LanguageModel::MiniMaxM1 => "MiniMax M1".into(),
            LanguageModel::O3 => "o3".into(),
            LanguageModel::NovaPro => "Amazon Nova Pro 1.0".into(),
            LanguageModel::Gemini25Pro => "Gemini Pro 2.5".into(),
            LanguageModel::DoubaoSeed16 => "Doubao Seed 1.6".into(),
            LanguageModel::KimiLatest => "Kimi Latest".into(),
            LanguageModel::Step16k => "Step 2 16k".into(),
            LanguageModel::Glm4Plus => "GLM-4-Plus".into(),
            LanguageModel::Opus4 => "Claude Opus 4".into(),
            LanguageModel::Sonnet4 => "Claude Sonnet 4".into(),
        }
    }
}
