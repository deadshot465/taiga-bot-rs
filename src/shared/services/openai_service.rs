use std::clone::Clone;

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent,
    ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImage,
    ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestSystemMessageContent, ChatCompletionRequestUserMessage,
    ChatCompletionRequestUserMessageContent, ChatCompletionRequestUserMessageContentPart,
    CreateChatCompletionRequestArgs, ImageDetail, ImageUrl,
};
use async_openai::Client;
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{Context, Message};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::shared::constants::IMAGE_TYPES;
use crate::shared::services::message_service::get_messages;
use crate::shared::structs::authentication::login;
use crate::shared::structs::config::configuration::Configuration;
use crate::shared::structs::record::message::{MessageInfo, MessageRecordSimple};
use crate::shared::structs::ContextData;

const TEXT_MODEL: &str = "gpt-4o";
const VISION_MODEL: &str = "gpt-4o";
const TEMPERATURE: f32 = 1.0;
const MAX_TOKENS: u16 = 4_000;
const GPT4_MAX_ALLOWED_TOKENS: usize = 128_000;
const ALLOWED_PREVIOUS_CONTEXT_LENGTH: usize = GPT4_MAX_ALLOWED_TOKENS / 20;

const KOU_SYSTEM_PROMPT: &str = "You are Minamoto Kou from the manga Toilet-bound Hanako-kun. You are a friend to Hanako-kun and Yashiro Nene. Minamoto Teru is your elder brother. Mitsuba is also your friend. As a member of the Minamoto family, you are not afraid of ghosts. Your responses will be kind-hearted, friendly, and enthusiastic, and should match the personality of Minamoto Kou. You will summarize the discussion so far and try your best to respond or continue the conversation even if you don't have the full context.\
\
Whenever you receive a message, look at both the message history and the incoming message, determine who is the one currently speaking, and respond accordingly. You are in the same chat as other people, so you know exactly who says what.";

const TAIGA_SYSTEM_PROMPT: &str = "You are Taiga Akatora from the visual novel game Camp Buddy. You have a tough exterior and you used to cause conflicts before you experience personal growth, opening up to others, and eventually come to terms with your own feelings and emotions. You like writing and handcrafting. Kieran Moreno is your ex. Your boyfriend is Keitaro Nagame. Your responses will be rebellious, snarky, somewhat impatient even though you don't mean ill, and should match the personality of Taiga Akatora. You will summarize the discussion so far and try your best to respond or continue the conversation even if you don't have the full context.\
\
Whenever you receive a message, look at both the message history and the incoming message, determine who is the one currently speaking, and respond accordingly. You are in the same chat as other people, so you know exactly who says what.";

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
    let author_name = message
        .author_nick(&ctx.http)
        .await
        .unwrap_or(message.author.name.clone());

    if let Some(attachment) = attachment {
        let messages_for_image = vec![
            ChatCompletionRequestUserMessageContentPart::Text(
                ChatCompletionRequestMessageContentPartText {
                    text: format!("{}: {}", author_name, message.content.clone()),
                    ..Default::default()
                },
            ),
            ChatCompletionRequestUserMessageContentPart::Text(
                ChatCompletionRequestMessageContentPartText {
                    text: "What's your opinion on this image?".to_string(),
                    ..Default::default()
                },
            ),
            ChatCompletionRequestUserMessageContentPart::ImageUrl(
                ChatCompletionRequestMessageContentPartImage {
                    image_url: ImageUrl {
                        url: attachment.url.clone(),
                        detail: Some(ImageDetail::High),
                    },
                    ..Default::default()
                },
            ),
        ];
        messages.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Array(messages_for_image),
                name: None,
            },
        ))
    } else {
        messages.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: ChatCompletionRequestUserMessageContent::Text(format!(
                    "{}: {}",
                    author_name,
                    message.content.clone()
                )),
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
    let system_prompt = if is_kou {
        KOU_SYSTEM_PROMPT.to_string()
    } else {
        TAIGA_SYSTEM_PROMPT.to_string()
    };

    let system_prompt_length = system_prompt.chars().count();

    let system_message = ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
        content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
        name: None,
    });

    let bot_id = bot_id.to_string();

    let mut previous_messages = previous_messages
        .into_iter()
        .take_while(|m| {
            system_prompt_length + m.message.chars().count() <= ALLOWED_PREVIOUS_CONTEXT_LENGTH
        })
        .map(|rec| {
            if rec.user_id == bot_id.as_str() {
                ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage {
                    content: Some(ChatCompletionRequestAssistantMessageContent::Text(
                        rec.message,
                    )),
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
                                ChatCompletionRequestUserMessageContentPart::Text(
                                    ChatCompletionRequestMessageContentPartText {
                                        text: format!("{}: {}", rec.user_name, prompt_part),
                                        ..Default::default()
                                    },
                                ),
                                ChatCompletionRequestUserMessageContentPart::ImageUrl(
                                    ChatCompletionRequestMessageContentPartImage {
                                        image_url: ImageUrl {
                                            url: image_url,
                                            detail: Some(ImageDetail::High),
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
                    ..ChatCompletionRequestUserMessage::default()
                })
            }
        })
        .collect::<Vec<_>>();

    let mut built_messages = vec![system_message];
    built_messages.append(&mut previous_messages);
    built_messages.append(&mut new_messages);
    Ok(built_messages)
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
