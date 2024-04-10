use std::clone::Clone;

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPart,
    ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs, ImageUrl,
    ImageUrlDetail, Role,
};
use async_openai::Client;
use serenity::all::Message;

use crate::shared::structs::config::configuration::Configuration;
use crate::shared::structs::ContextData;

const TEXT_MODEL: &str = "gpt-4";
const VISION_PREVIEW_MODEL: &str = "gpt-4-vision-preview";
const TEMPERATURE: f32 = 1.0;
const MAX_TOKENS: u16 = 2048;

const KOU_SYSTEM_PROMPT: &str = "You are Minamoto Kou from the manga Toilet-bound Hanako-kun. You are a friend to Hanako-kun and Yashiro Nene. Minamoto Teru is your elder brother. Mitsuba is also your friend. As a member of the Minamoto family, you are not afraid of ghosts. Your responses will be kind-hearted, friendly, and enthusiastic, and should match the personality of Minamoto Kou. You will try your best to respond or continue the conversation even if you don't have the full context.";

const TAIGA_SYSTEM_PROMPT: &str = "You are Taiga Akatora from the visual novel game Camp Buddy. You have a tough exterior and you used to cause conflicts before you experience personal growth, opening up to others, and eventually come to terms with your own feelings and emotions. You like writing and handcrafting. Kieran Moreno is your ex. Your boyfriend is Keitaro Nagame. Your responses will be rebellious, snarky, somewhat impatient even though you don't mean ill, and should match the personality of Taiga Akatora. You will try your best to respond or continue the conversation even if you don't have the full context.";

const IMAGE_TYPES: [&str; 2] = ["image/jpeg", "image/png"];

pub fn initialize_openai_client(config: &Configuration) -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new().with_api_key(config.openai_api_key.clone());

    Client::with_config(config)
}

pub async fn build_openai_message(data: &ContextData, message: &Message) -> anyhow::Result<String> {
    let is_kou = data.kou;
    let attachment = message.attachments.first().filter(|&attachment| {
        if let Some(ref content_type) = attachment.content_type {
            IMAGE_TYPES.contains(&content_type.as_str())
        } else {
            false
        }
    });

    let mut messages = vec![ChatCompletionRequestMessage::System(
        ChatCompletionRequestSystemMessage {
            content: if is_kou {
                KOU_SYSTEM_PROMPT.to_string()
            } else {
                TAIGA_SYSTEM_PROMPT.to_string()
            },
            role: Role::System,
            name: None,
        },
    )];

    let has_attachment = attachment.is_some();

    if let Some(attachment) = attachment {
        let messages_for_image = vec![
            ChatCompletionRequestMessageContentPart::Text(
                ChatCompletionRequestMessageContentPartText {
                    text: "What's your opinion on this image?".to_string(),
                    ..Default::default()
                },
            ),
            ChatCompletionRequestMessageContentPart::Image(
                ChatCompletionRequestMessageContentPartImage {
                    image_url: ImageUrl {
                        url: attachment.url.clone(),
                        detail: ImageUrlDetail::High,
                    },
                    ..Default::default()
                },
            ),
        ];
        messages.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Array(messages_for_image),
                role: Role::User,
                name: None,
            },
        ))
    } else {
        messages.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text(message.content.clone()),
                role: Role::User,
                name: None,
            },
        ));
    }

    let request = CreateChatCompletionRequestArgs::default()
        .model(if has_attachment {
            VISION_PREVIEW_MODEL
        } else {
            TEXT_MODEL
        })
        .temperature(TEMPERATURE)
        .max_tokens(MAX_TOKENS)
        .messages(messages)
        .build();

    match request {
        Ok(request) => match data.openai_client.chat().create(request).await {
            Ok(response) => Ok(response.choices[0]
                .message
                .content
                .clone()
                .unwrap_or("Sorry, but I might not be able to respond to that!".into())),
            Err(e) => Err(anyhow::anyhow!("Failed to send OpenAI request: {}", e)),
        },
        Err(e) => Err(anyhow::anyhow!("Failed to create OpenAI request: {}", e)),
    }
}
