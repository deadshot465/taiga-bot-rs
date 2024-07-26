use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    CreateChatCompletionRequestArgs,
};
use serenity::all::Attachment;

use crate::shared::structs::config::configuration::Configuration;
use crate::shared::structs::ContextData;

const DEEP_SEEK_MODEL: &str = "deepseek/deepseek-chat";
const TEMPERATURE: f32 = 1.0;
const OPEN_ROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

const TRANSLATION_SYSTEM_PROMPT: &str =
    "你是一位獲獎無數的中文科幻小說作家。你有完美的記憶能力並且會嚴格遵守獲得的指示與前後文。\
    你會完美記得所有的內容跟提示，並且不會偏離劇情的內容與方向。\
    你充滿創意與自由，擅長使用你獲獎無數的中文科幻小說筆觸，將英文小說的內容翻成繁體中文。\
    \
    在翻譯時，請務必記得以下指示：{INSTRUCTION}";

pub fn initialize_open_router_client(config: &Configuration) -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new()
        .with_api_base(OPEN_ROUTER_BASE_URL)
        .with_api_key(config.open_router_api_key.clone());

    Client::with_config(config)
}

pub async fn translate_with_deep_seek(
    data: &ContextData,
    attachment: &Attachment,
) -> anyhow::Result<String> {
    let raw_bytes = attachment.download().await?;
    let text = String::from_utf8(raw_bytes)?;
    let instructions = data.translation_instructions.clone();
    let replacement = format!("\n{}", instructions);
    let system_prompt = TRANSLATION_SYSTEM_PROMPT.replace("{INSTRUCTION}", &replacement);

    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: system_prompt,
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(text),
            name: None,
        }),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model(DEEP_SEEK_MODEL)
        .temperature(TEMPERATURE)
        .messages(messages)
        .build();

    match request {
        Ok(request) => match data.open_router_client.chat().create(request).await {
            Ok(response) => {
                let response_message = response.choices[0]
                    .message
                    .content
                    .clone()
                    .unwrap_or("Sorry, but I can't seem to translate that!".into());

                Ok(response_message)
            }
            Err(e) => Err(anyhow::anyhow!("Failed to send Open Router request: {}", e)),
        },
        Err(e) => Err(anyhow::anyhow!(
            "Failed to create Open Router request: {}",
            e
        )),
    }
}
