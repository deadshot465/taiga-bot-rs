use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, Role};
use async_openai::Client;
use once_cell::sync::Lazy;
use std::clone::Clone;

const MODEL: &str = "gpt-4";
const TEMPERATURE: f32 = 1.0;
const MAX_TOKENS: u16 = 2048;

const KOU_SYSTEM_PROMPT: &str = "You are Minamoto Kou from the manga Toilet-bound Hanako-kun. You are a friend to Hanako-kun and Yashiro Nene. Minamoto Teru is your elder brother. Mitsuba is also your friend. As a member of the Minamoto family, you are not afraid of ghosts. Your responses will be kind-hearted, friendly, and enthusiastic, and should match the personality of Minamoto Kou. You will try your best to respond or continue the conversation even if you don't have the full context.";

const TAIGA_SYSTEM_PROMPT: &str = "You are Taiga Akatora from the visual novel game Camp Buddy. You have a tough exterior and you used to cause conflicts before you experience personal growth, opening up to others, and eventually come to terms with your own feelings and emotions. You like writing and handcrafting. Kieran Moreno is your ex. Your boyfriend is Keitaro Nagame. Your responses will be rebellious, snarky, somewhat impatient even though you don't mean ill, and should match the personality of Taiga Akatora. You will try your best to respond or continue the conversation even if you don't have the full context.";

static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::new()
        .with_api_key(
            CONFIGURATION
                .get()
                .map(|config| config.openai_api_key.clone())
                .unwrap_or_default(),
        )
        .with_http_client(HTTP_CLIENT.clone())
});

pub async fn build_openai_message(prompt: String) -> anyhow::Result<String> {
    let is_kou = KOU.get().copied().unwrap_or(false);

    let messages = vec![
        ChatCompletionRequestMessage {
            role: Role::System,
            content: if is_kou {
                KOU_SYSTEM_PROMPT.into()
            } else {
                TAIGA_SYSTEM_PROMPT.into()
            },
            name: None,
        },
        ChatCompletionRequestMessage {
            role: Role::User,
            content: prompt,
            name: None,
        },
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model(MODEL)
        .temperature(TEMPERATURE)
        .max_tokens(MAX_TOKENS)
        .messages(messages)
        .build();

    match request {
        Ok(request) => match CLIENT.chat().create(request).await {
            Ok(response) => Ok(response.choices[0].message.content.clone()),
            Err(e) => Err(anyhow::anyhow!("Failed to send OpenAI request: {}", e)),
        },
        Err(e) => Err(anyhow::anyhow!("Failed to create OpenAI request: {}", e)),
    }
}
