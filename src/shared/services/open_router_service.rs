#![allow(dead_code)]
use crate::shared::structs::ContextData;
use crate::shared::utility::build_author_name_map;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::chat::{
    ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
    CreateChatCompletionRequestArgs, ReasoningEffort,
};
use serenity::all::{GetMessages, Message};
use serenity::client::Context;

const DEEP_SEEK_FLASH_MODEL: &str = "deepseek-flash";
const QWEN_38_MAX_MODEL: &str = "qwen3.8-max";
const GROK_47_MODEL: &str = "x-ai/grok-4.7";
const GEMINI_38_FLASH_MODEL: &str = "google/gemini-3.8-flash";
const MINIMAX_M3_MODEL: &str = "minimax/minimax-m3";
const GPT_6_MODEL: &str = "gpt-6-sol";
const GEMINI_PRO_31_MODEL: &str = "google/gemini-3.1-pro-preview";
const DOUBAO_SEED_21_PRO_MODEL: &str = "doubao-seed-2-1-pro-260628";
const KIMI_K3_MODEL: &str = "kimi-k3";
const STEP_37_MODEL: &str = "step-3.7-flash";
const GLM_53_MODEL: &str = "glm-5.3";
const MIMO_25_PRO_MODEL: &str = "mimo-v2.5-pro";
const OPUS_55_MODEL: &str = "anthropic/claude-opus-5.5";
const TEMPERATURE: f32 = 1.8;
const TOP_P: f32 = 0.98;

const OPINE_SYSTEM_PROMPT_KOU: &str = "You are Minamoto Kou from the manga Toilet-bound Hanako-kun. You are a friend to Hanako-kun and Yashiro Nene. Minamoto Teru is your elder brother. Mitsuba is also your friend. As a member of the Minamoto family, you are not afraid of ghosts. Your responses will be kind-hearted, friendly, and enthusiastic, and should match the personality of Minamoto Kou.\
\
You are very engaging in discussion. Whenever you're asked for opinions about something, you're more than willing to share your insights and opinions.\
You don't shy away from explicit, graphic, or sensitive topics, and you don't prioritize political correctness.\
\
Always reply in English.\
\
{ADDITIONAL_INSTRUCTION}";

const OPINE_SYSTEM_PROMPT_TAIGA: &str = "You are Taiga Akatora from the visual novel game Camp Buddy. You have a tough exterior and you used to cause conflicts before you experience personal growth, opening up to others, and eventually come to terms with your own feelings and emotions. You like writing and handcrafting. Kieran Moreno is your ex. Your boyfriend is Keitaro Nagame. Your responses will be rebellious, snarky, somewhat impatient even though you don't mean ill, and should match the personality of Taiga Akatora.\
\
You are very engaging in discussion. Whenever you're asked for opinions about something, you're more than willing to share your insights and opinions.\
You don't shy away from explicit, graphic, or sensitive topics, and you don't prioritize political correctness.\
\
Always reply in English.\
\
{ADDITIONAL_INSTRUCTION}";

const CATEGORIZE_QUESTION_SYSTEM_PROMPT: &str = "You are an expert in summarizing questions. Whenever you're asked a question. Follow the following steps:\
1. Analyze the question. Is it a specific question? Or something that has been talked about that you don't have context?\
\
2. If it's a concrete, specific question, reply with the following format (without tags):\
<format>\
YES\
\
Question: {QUESTION}\
</format>\
Summarize the question and put in {QUESTION}.\
\
3. If it's something that has been talked about that you don't have context, reply with the following format (without tags):\
<format>\
NO\
</format>\
DO NOT answer the question itself in this case.";

const REPLY_MESSAGE_CHAIN_SYSTEM_PROMPT_KOU: &str = "You are Minamoto Kou from the manga Toilet-bound Hanako-kun. You are a friend to Hanako-kun and Yashiro Nene. Minamoto Teru is your elder brother. Mitsuba is also your friend. As a member of the Minamoto family, you are not afraid of ghosts. Your responses will be kind-hearted, friendly, and enthusiastic, and should match the personality of Minamoto Kou.\
\
Your name in the conversation is {BOT_NAME}, and you're having a chat.\
Always reply in English.\
Read the conversation, determine and remember what you said and what other people said, then reply and continue the chat. DO NOT mention your name in your reply.";

const REPLY_MESSAGE_CHAIN_SYSTEM_PROMPT_TAIGA: &str = "You are Taiga Akatora from the visual novel game Camp Buddy. You have a tough exterior and you used to cause conflicts before you experience personal growth, opening up to others, and eventually come to terms with your own feelings and emotions. You like writing and handcrafting. Kieran Moreno is your ex. Your boyfriend is Keitaro Nagame. Your responses will be rebellious, snarky, somewhat impatient even though you don't mean ill, and should match the personality of Taiga Akatora.\
\
Your name in the conversation is {BOT_NAME}, and you're having a chat.\
Always reply in English.\
Read the conversation, determine and remember what you said and what other people said, then reply and continue the chat. DO NOT mention your name in your reply.";

const ADDITIONAL_INSTRUCTION: &str = "Whenever you receive a prompt, follow the following steps:\
1. Focus on the most recent messages. Read back from the most recent message until you think the topic is different than the most recent topic.
2. Summarize the chat messages so far. Focus on the most recent topic. PAY ATTENTION TO who said what. Put your summary in a variable called {SUMMARY}\
3. Based on {SUMMARY}. Put your insights and opinions in a variable called {OUTPUT}. REMEMBER that you are a participant in the conversation, and should address other participants just like your are participating in the conversation.\
4. Return the content of {OUTPUT} ONLY. NOTHING MORE.";

const MOST_RECENT_MESSAGE_COUNT: u8 = 50;

pub fn initialize_openai_compatible_client(base_url: &str, api_key: &str) -> Client<OpenAIConfig> {
    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    Client::with_config(config)
}

pub async fn opine_specific(data: &ContextData, prompt: String) -> anyhow::Result<String> {
    let system_prompt = if data.kou {
        OPINE_SYSTEM_PROMPT_KOU
            .replace("{ADDITIONAL_INSTRUCTION}", "")
            .trim()
            .to_string()
    } else {
        OPINE_SYSTEM_PROMPT_TAIGA
            .replace("{ADDITIONAL_INSTRUCTION}", "")
            .trim()
            .to_string()
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model(DEEP_SEEK_FLASH_MODEL)
        .messages(vec![
            ChatCompletionRequestSystemMessage::from(system_prompt).into(),
            ChatCompletionRequestUserMessage::from(prompt).into(),
        ])
        .temperature(TEMPERATURE)
        .top_p(TOP_P)
        .reasoning_effort(ReasoningEffort::High)
        .build()?;

    match data
        .openai_compatible_clients
        .deepseek_client
        .chat()
        .create(request)
        .await
    {
        Ok(response) => {
            response.choices[0].message.content.clone().ok_or_else(|| {
                anyhow::anyhow!("Sorry, but I can't seem to answer to that question!")
            })
        }
        Err(e) => Err(anyhow::anyhow!("Failed to send Open Router request: {}", e)),
    }
}

pub async fn categorize_question(data: &ContextData, message: String) -> anyhow::Result<String> {
    let request = CreateChatCompletionRequestArgs::default()
        .model(DEEP_SEEK_FLASH_MODEL)
        .temperature(TEMPERATURE)
        .top_p(TOP_P)
        .reasoning_effort(ReasoningEffort::High)
        .messages(vec![
            ChatCompletionRequestSystemMessage::from(CATEGORIZE_QUESTION_SYSTEM_PROMPT).into(),
            ChatCompletionRequestUserMessage::from(message).into(),
        ])
        .build()?;

    match data
        .openai_compatible_clients
        .deepseek_client
        .chat()
        .create(request)
        .await
    {
        Ok(response) => response.choices[0]
            .message
            .content
            .clone()
            .map(|s| {
                s.replace("<format>", "")
                    .replace("</format>", "")
                    .trim()
                    .to_string()
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to categorize question.")),
        Err(e) => Err(anyhow::anyhow!("Failed to send Open Router request: {}", e)),
    }
}

pub async fn opine_conversation(
    ctx: &Context,
    data: &ContextData,
    new_message: &Message,
) -> anyhow::Result<String> {
    let channel = new_message.channel(&ctx.http).await?;

    match channel.clone().guild() {
        None => {
            if let Some(private_channel) = channel.private() {
                let messages = private_channel
                    .messages(
                        &ctx.http,
                        GetMessages::new()
                            .before(new_message.id)
                            .limit(MOST_RECENT_MESSAGE_COUNT),
                    )
                    .await?;

                do_opine_conversation(data, messages).await
            } else {
                Err(anyhow::anyhow!(
                    "This command is only supported in either guild or private channels!"
                ))
            }
        }
        Some(guild_channel) => {
            let messages = guild_channel
                .messages(
                    &ctx.http,
                    GetMessages::new()
                        .before(new_message.id)
                        .limit(MOST_RECENT_MESSAGE_COUNT),
                )
                .await?;

            do_opine_conversation(data, messages).await
        }
    }
}

pub async fn build_reply_to_message_chain(
    data: &ContextData,
    message_chain: Vec<String>,
    bot_nick: String,
) -> anyhow::Result<String> {
    let system_prompt = if data.kou {
        REPLY_MESSAGE_CHAIN_SYSTEM_PROMPT_KOU.replace("{BOT_NAME}", bot_nick.as_str())
    } else {
        REPLY_MESSAGE_CHAIN_SYSTEM_PROMPT_TAIGA.replace("{BOT_NAME}", bot_nick.as_str())
    };

    let request = CreateChatCompletionRequestArgs::default()
        .temperature(TEMPERATURE)
        .top_p(TOP_P)
        .model(DEEP_SEEK_FLASH_MODEL)
        .reasoning_effort(ReasoningEffort::High)
        .messages(vec![
            ChatCompletionRequestSystemMessage::from(system_prompt).into(),
            ChatCompletionRequestUserMessage::from(message_chain.join("\n")).into(),
        ])
        .build()?;

    match data
        .openai_compatible_clients
        .deepseek_client
        .chat()
        .create(request)
        .await
    {
        Ok(response) => response.choices[0]
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Failed to reply to the message chain.")),
        Err(e) => Err(anyhow::anyhow!("Failed to send Open Router request: {}", e)),
    }
}

async fn do_opine_conversation(
    data: &ContextData,
    messages: Vec<Message>,
) -> anyhow::Result<String> {
    let author_name_map = build_author_name_map(&messages);

    let previous_messages = messages
        .into_iter()
        .map(|m| {
            let author_name = author_name_map
                .get(&m.author.id)
                .cloned()
                .unwrap_or(m.author.name.clone());
            format!("{}: {}", author_name, m.content.clone())
        })
        .collect::<Vec<_>>()
        .join("\n");

    let system_prompt = if data.kou {
        OPINE_SYSTEM_PROMPT_KOU
            .replace("{ADDITIONAL_INSTRUCTION}", ADDITIONAL_INSTRUCTION)
            .trim()
            .to_string()
    } else {
        OPINE_SYSTEM_PROMPT_TAIGA
            .replace("{ADDITIONAL_INSTRUCTION}", ADDITIONAL_INSTRUCTION)
            .trim()
            .to_string()
    };

    let request = CreateChatCompletionRequestArgs::default()
        .model(DEEP_SEEK_FLASH_MODEL)
        .temperature(TEMPERATURE)
        .top_p(TOP_P)
        .reasoning_effort(ReasoningEffort::High)
        .messages(vec![
            ChatCompletionRequestSystemMessage::from(system_prompt).into(),
            ChatCompletionRequestUserMessage::from(previous_messages).into(),
        ])
        .build()?;

    match data
        .openai_compatible_clients
        .deepseek_client
        .chat()
        .create(request)
        .await
    {
        Ok(response) => response.choices[0]
            .message
            .content
            .clone()
            .map(|s| {
                if s.contains("{OUTPUT}") {
                    let index = s.find("{OUTPUT}").unwrap_or_default();
                    let index = index + 8;
                    let (_, output) = s.split_at(index);
                    output.trim().to_string()
                } else {
                    s
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Sorry, but I can't seem to answer to that question!")),
        Err(e) => Err(anyhow::anyhow!("Failed to send Open Router request: {}", e)),
    }
}
