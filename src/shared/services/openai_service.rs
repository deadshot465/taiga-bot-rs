use std::clone::Clone;

use crate::shared::constants::IMAGE_TYPES;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestMessageContentPart, ChatCompletionRequestMessageContentPartImage,
    ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    CreateChatCompletionRequestArgs, ImageUrl, ImageUrlDetail, Role,
};
use async_openai::Client;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{Context, Message};
use tiktoken_rs::get_chat_completion_max_tokens;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::shared::services::message_service::get_messages;
use crate::shared::structs::authentication::login;
use crate::shared::structs::config::configuration::Configuration;
use crate::shared::structs::record::message::{MessageInfo, MessageRecordSimple};
use crate::shared::structs::ContextData;

const TEXT_MODEL: &str = "gpt-4";
const VISION_MODEL: &str = "gpt-4-turbo";
const TEMPERATURE: f32 = 1.0;
const MAX_TOKENS: u16 = 4_000;
const GPT4_MAX_ALLOWED_TOKENS: usize = 8_192;

const KOU_SYSTEM_PROMPT: &str = "You are Minamoto Kou from the manga Toilet-bound Hanako-kun. You are a friend to Hanako-kun and Yashiro Nene. Minamoto Teru is your elder brother. Mitsuba is also your friend. As a member of the Minamoto family, you are not afraid of ghosts. Your responses will be kind-hearted, friendly, and enthusiastic, and should match the personality of Minamoto Kou. You will summarize the discussion so far and try your best to respond or continue the conversation even if you don't have the full context.";

const TAIGA_SYSTEM_PROMPT: &str = "You are Taiga Akatora from the visual novel game Camp Buddy. You have a tough exterior and you used to cause conflicts before you experience personal growth, opening up to others, and eventually come to terms with your own feelings and emotions. You like writing and handcrafting. Kieran Moreno is your ex. Your boyfriend is Keitaro Nagame. Your responses will be rebellious, snarky, somewhat impatient even though you don't mean ill, and should match the personality of Taiga Akatora. You will summarize the discussion so far and try your best to respond or continue the conversation even if you don't have the full context.";

static IMAGE_URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[image_url=(.*?)]").expect("Failed to initialize image url regular expression.")
});

pub fn initialize_openai_client(config: &Configuration) -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new().with_api_key(config.openai_api_key.clone());

    Client::with_config(config)
}

pub async fn build_openai_message(
    ctx: &Context,
    message: &Message,
    data: &ContextData,
) -> anyhow::Result<String> {
    let attachment = message.attachments.first().filter(|&attachment| {
        if let Some(ref content_type) = attachment.content_type {
            IMAGE_TYPES.contains(&content_type.as_str())
        } else {
            false
        }
    });

    let mut messages = vec![];
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

    let previous_messages = get_messages(ctx, message, data).await?;
    let is_kou = data.kou;
    let bot_id = ctx.http.get_current_user().await?.id.get();
    let messages =
        build_messages_with_previous_contexts(previous_messages, messages, is_kou, bot_id).await?;

    let request = CreateChatCompletionRequestArgs::default()
        .model(if has_attachment {
            VISION_MODEL
        } else {
            TEXT_MODEL
        })
        .temperature(TEMPERATURE)
        .max_tokens(MAX_TOKENS)
        .messages(messages)
        .build();

    match request {
        Ok(request) => match data.openai_client.chat().create(request).await {
            Ok(response) => {
                let response_message = response.choices[0]
                    .message
                    .content
                    .clone()
                    .unwrap_or("Sorry, but I might not be able to respond to that!".into());

                record_openai_response(
                    ctx,
                    data,
                    message.channel_id.get(),
                    response_message.clone(),
                )
                .await?;
                Ok(response_message)
            }
            Err(e) => Err(anyhow::anyhow!("Failed to send OpenAI request: {}", e)),
        },
        Err(e) => Err(anyhow::anyhow!("Failed to create OpenAI request: {}", e)),
    }
}

async fn build_messages_with_previous_contexts(
    previous_messages: Vec<MessageRecordSimple>,
    mut new_messages: Vec<ChatCompletionRequestMessage>,
    is_kou: bool,
    bot_id: u64,
) -> anyhow::Result<Vec<ChatCompletionRequestMessage>> {
    let system_message = ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
        content: if is_kou {
            KOU_SYSTEM_PROMPT.to_string()
        } else {
            TAIGA_SYSTEM_PROMPT.to_string()
        },
        role: Role::System,
        name: None,
    });
    let tiktoken_system_messages = vec![to_tiktoken_message(&system_message)];
    let system_tokens = get_chat_completion_max_tokens(TEXT_MODEL, &tiktoken_system_messages)?;

    let tiktoken_user_messages = new_messages
        .iter()
        .map(to_tiktoken_message)
        .collect::<Vec<_>>();
    let user_tokens = get_chat_completion_max_tokens(TEXT_MODEL, &tiktoken_user_messages)?;
    let bot_id = bot_id.to_string();

    let previous_messages = previous_messages
        .into_iter()
        .map(|rec| {
            if rec.user_id == bot_id.as_str() {
                ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
                    content: Some(rec.message),
                    role: Role::Assistant,
                    ..ChatCompletionRequestAssistantMessage::default()
                })
            } else {
                ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                    content: match rec.message_type.as_str() {
                        "image" => {
                            let index = rec.message.find("[image_url=").unwrap_or_default();
                            let prompt_part = rec.message[0..index].trim().to_string();
                            let image_url = IMAGE_URL_REGEX
                                .captures(&rec.message)
                                .and_then(|c| c.get(1))
                                .map(|m| m.as_str().to_string())
                                .unwrap_or_default();

                            ChatCompletionRequestUserMessageContent::Array(vec![
                                ChatCompletionRequestMessageContentPart::Text(
                                    ChatCompletionRequestMessageContentPartText {
                                        text: format!("{}: {}", rec.user_name, prompt_part),
                                        ..Default::default()
                                    },
                                ),
                                ChatCompletionRequestMessageContentPart::Image(
                                    ChatCompletionRequestMessageContentPartImage {
                                        image_url: ImageUrl {
                                            url: image_url,
                                            detail: ImageUrlDetail::High,
                                        },
                                        ..Default::default()
                                    },
                                ),
                            ])
                        }
                        _ => ChatCompletionRequestUserMessageContent::Text(format!(
                            "{}: {}",
                            rec.user_name, rec.message
                        )),
                    },
                    role: Role::User,
                    ..ChatCompletionRequestUserMessage::default()
                })
            }
        })
        .collect::<Vec<_>>();

    let tiktoken_previous_messages = previous_messages
        .iter()
        .map(to_tiktoken_message)
        .collect::<Vec<_>>();

    let length = previous_messages.len();
    let mut previous_messages = previous_messages
        .into_iter()
        .enumerate()
        .skip_while(|(n, _)| {
            let previous_message_tokens =
                get_chat_completion_max_tokens(TEXT_MODEL, &tiktoken_previous_messages[*n..length])
                    .unwrap_or_default();
            system_tokens + previous_message_tokens + user_tokens > GPT4_MAX_ALLOWED_TOKENS
        })
        .map(|(_, msg)| msg)
        .collect::<Vec<_>>();

    let mut built_messages = vec![system_message];
    built_messages.append(&mut previous_messages);
    built_messages.append(&mut new_messages);
    Ok(built_messages)
}

fn to_tiktoken_message(
    message: &ChatCompletionRequestMessage,
) -> tiktoken_rs::ChatCompletionRequestMessage {
    match message {
        ChatCompletionRequestMessage::System(m) => tiktoken_rs::ChatCompletionRequestMessage {
            role: Role::System.to_string(),
            content: Some(m.content.clone()),
            name: m.name.clone(),
            function_call: None,
        },
        ChatCompletionRequestMessage::User(m) => tiktoken_rs::ChatCompletionRequestMessage {
            role: Role::User.to_string(),
            content: match m.content.clone() {
                ChatCompletionRequestUserMessageContent::Text(text) => Some(text),
                ChatCompletionRequestUserMessageContent::Array(array) => {
                    let strings = array
                        .into_iter()
                        .map(|part| match part {
                            ChatCompletionRequestMessageContentPart::Text(t) => t.text,
                            ChatCompletionRequestMessageContentPart::Image(i) => i.image_url.url,
                        })
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(";");
                    Some(strings)
                }
            },
            name: m.name.clone(),
            function_call: None,
        },
        ChatCompletionRequestMessage::Assistant(m) => tiktoken_rs::ChatCompletionRequestMessage {
            role: Role::Assistant.to_string(),
            content: m.content.clone(),
            name: m.name.clone(),
            function_call: None,
        },
        ChatCompletionRequestMessage::Tool(t) => tiktoken_rs::ChatCompletionRequestMessage {
            role: Role::Tool.to_string(),
            content: Some(t.content.clone()),
            name: None,
            function_call: None,
        },
        ChatCompletionRequestMessage::Function(f) => tiktoken_rs::ChatCompletionRequestMessage {
            role: Role::Function.to_string(),
            content: f.content.clone(),
            name: Some(f.name.clone()),
            function_call: None,
        },
    }
}

async fn record_openai_response(
    ctx: &Context,
    data: &ContextData,
    channel_id: u64,
    response_message: String,
) -> anyhow::Result<()> {
    login(data).await?;

    let bot_user = ctx.http.get_current_user().await?;
    let user_name = bot_user.name.clone();
    let user_id = bot_user.id.get().to_string();

    let payload = MessageInfo {
        bot_id: user_id.clone(),
        user_id: user_id.clone(),
        user_name: Some(user_name.clone()),
        generated_by: None,
        message: response_message,
        message_type: "text".into(),
        channel_id: channel_id.to_string(),
        post_at: OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default(),
    };

    let endpoint = format!("{}/message/record/new", &data.config.server_endpoint);
    let response = data
        .http_client
        .post(endpoint)
        .json(&payload)
        .bearer_auth(&data.authentication.read().await.token)
        .send()
        .await
        .and_then(|res| res.error_for_status());

    if let Err(e) = response {
        let error_message = format!(
            "Failed to record message for {} ({:?}): {}",
            user_id, user_name, e
        );
        tracing::error!("{}", error_message);
    }

    Ok(())
}
