use crate::commands::utility::translate::{LanguageModel, Novel};
use crate::shared::structs::{ContextData, OpenAICompatibleClients};
use crate::shared::utility::build_author_name_map;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestDeveloperMessage, ChatCompletionRequestDeveloperMessageContent,
    ChatCompletionRequestMessage, ChatCompletionRequestProvider,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageContent,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    CreateChatCompletionRequestArgs, ReasoningEffort,
};
use async_openai::Client;
use serenity::all::{Attachment, GetMessages, Message};
use serenity::client::Context;
use std::sync::Arc;

const DEEP_SEEK_MODEL: &str = "deepseek/deepseek-chat-v3-0324";
const GPT_41_MODEL: &str = "openai/gpt-4.1";
const MISTRAL_LARGE_2411_MODEL: &str = "mistralai/mistral-large-2411";
const QWEN_MAX_MODEL: &str = "qwen/qwen-max";
const COHERE_COMMAND_A_MODEL: &str = "cohere/command-a";
const DEEP_SEEK_R1_MODEL: &str = "deepseek/deepseek-r1";
const GROK_3_BETA_MODEL: &str = "x-ai/grok-3-beta";
const GEMINI_25_FLASH_PREVIEW_MODEL: &str = "google/gemini-2.5-flash-preview-05-20";
const MINIMAX_01_MODEL: &str = "minimax/minimax-01";
const O4_MINI_MODEL: &str = "o4-mini";
const O1_MODEL: &str = "o1";
const NOVA_PRO_MODEL: &str = "amazon/nova-pro-v1";
const GEMINI_PRO_25_PREVIEW_MODEL: &str = "google/gemini-2.5-pro-preview";
const DOUBAO_15_PRO_256K_MODEL: &str = "ep-20250218185054-vnnbk";
const KIMI_LATEST_MODEL: &str = "kimi-latest";
const STEP_2_16K_MODEL: &str = "step-2-16k";
const GLM_4_PLUS_MODEL: &str = "GLM-4-Plus";
const OPUS_4_MODEL: &str = "anthropic/claude-opus-4";
const SONNET_4_MODEL: &str = "anthropic/claude-sonnet-4";
const TEMPERATURE: f32 = 1.0;
const TOP_P: f32 = 1.0;

const FORGED_IN_STARLIGHT_SYSTEM_PROMPT: &str =
    "你是一位獲獎無數的中文科幻小說作家。你有完美的記憶能力並且會嚴格遵守獲得的指示與前後文。\
    你會完美記得所有的內容跟提示，並且不會偏離劇情的內容與方向。\
    你充滿創意與自由，擅長使用你獲獎無數的中文科幻小說筆觸及高品質文學作品的水準，將英文小說的內容翻成繁體中文。\
    請將重點擺在將語句和角色間的對話翻譯成自然、通順，且符合繁體中文口語及對話習慣的內容，而不是執著於將英文直翻為中文。\
    記住：你的主要讀者及對象是居住在台灣的台灣居民，因此在翻譯角色間的對話時，必須翻譯成符合台灣人對話方式的中文。\
    \
    在翻譯時，請務必記得以下指示：{INSTRUCTION}";

const CHRONOSPLIT_SYSTEM_PROMPT: &str =
    "你是一位獲獎無數的中文都市奇幻與科幻小說作家。你有完美的記憶能力並且會嚴格遵守獲得的指示與前後文。\
    你會完美記得所有的內容跟提示，並且不會偏離劇情的內容與方向。\
    你充滿創意與自由，擅長使用你獲獎無數的中文都市奇幻與科幻小說筆觸及高品質文學作品的水準，將英文小說的內容翻成繁體中文。\
    請將重點擺在將語句和角色間的對話翻譯成自然、通順，且符合繁體中文口語及對話習慣的內容，而不是執著於將英文直翻為中文。\
    記住：你的主要讀者及對象是居住在台灣的台灣居民，因此在翻譯角色間的對話時，必須翻譯成符合台灣人對話方式的中文。\
    \
    在翻譯時，請務必記得以下指示：{INSTRUCTION}";

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

pub async fn translate_with_model(
    novel: Novel,
    instructions: String,
    openai_client: Client<OpenAIConfig>,
    openai_compatible_clients: Arc<OpenAICompatibleClients>,
    attachment: Attachment,
    model: LanguageModel,
) -> anyhow::Result<String> {
    let raw_bytes = attachment.download().await?;
    let text = String::from_utf8(raw_bytes)?;
    let replacement = format!("\n{}", instructions);
    let system_prompt = match novel {
        Novel::ForgedInStarlight => {
            FORGED_IN_STARLIGHT_SYSTEM_PROMPT.replace("{INSTRUCTION}", &replacement)
        }
        Novel::Chronosplit => CHRONOSPLIT_SYSTEM_PROMPT.replace("{INSTRUCTION}", &replacement),
    };

    let model_str = match model {
        LanguageModel::DeepSeekV3 => DEEP_SEEK_MODEL,
        LanguageModel::Gpt41 => GPT_41_MODEL,
        LanguageModel::MistralLarge => MISTRAL_LARGE_2411_MODEL,
        LanguageModel::QwenMax => QWEN_MAX_MODEL,
        LanguageModel::CohereCommandA => COHERE_COMMAND_A_MODEL,
        LanguageModel::Grok3 => GROK_3_BETA_MODEL,
        LanguageModel::DeepSeekR1 => DEEP_SEEK_R1_MODEL,
        LanguageModel::Gemini25FlashPreview => GEMINI_25_FLASH_PREVIEW_MODEL,
        LanguageModel::MiniMax01 => MINIMAX_01_MODEL,
        LanguageModel::O4MiniHigh => O4_MINI_MODEL,
        LanguageModel::O1High => O1_MODEL,
        LanguageModel::NovaPro => NOVA_PRO_MODEL,
        LanguageModel::Gemini25ProPreview => GEMINI_PRO_25_PREVIEW_MODEL,
        LanguageModel::Doubao15Pro256k => DOUBAO_15_PRO_256K_MODEL,
        LanguageModel::KimiLatest => KIMI_LATEST_MODEL,
        LanguageModel::Step16k => STEP_2_16K_MODEL,
        LanguageModel::Glm4Plus => GLM_4_PLUS_MODEL,
        LanguageModel::Opus4 => OPUS_4_MODEL,
        LanguageModel::Sonnet4 => SONNET_4_MODEL,
    };

    let system_prompt = match model {
        m if m == LanguageModel::O1High || m == LanguageModel::O4MiniHigh => {
            ChatCompletionRequestMessage::Developer(ChatCompletionRequestDeveloperMessage {
                content: ChatCompletionRequestDeveloperMessageContent::Text(system_prompt),
                name: None,
            })
        }
        _ => ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
            name: None,
        }),
    };

    let messages = vec![
        system_prompt,
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(text),
            name: None,
        }),
    ];

    let temperature = match model {
        LanguageModel::KimiLatest => 0.3,
        LanguageModel::DeepSeekV3 => 1.8,
        _ => TEMPERATURE,
    };

    let top_p = match model {
        LanguageModel::DeepSeekV3 => 0.98,
        _ => TOP_P,
    };

    let mut request = CreateChatCompletionRequestArgs::default();
    request
        .model(model_str)
        .temperature(temperature)
        .top_p(top_p)
        .messages(messages);

    let request = match model {
        m if m == LanguageModel::O1High || m == LanguageModel::O4MiniHigh => {
            request.reasoning_effort(ReasoningEffort::High).build()?
        }
        m if m == LanguageModel::DeepSeekV3 || m == LanguageModel::DeepSeekR1 => request
            .provider(ChatCompletionRequestProvider {
                order: vec!["DeepSeek".into()],
                allow_fallbacks: false,
            })
            .build()?,
        _ => request.build()?,
    };

    let result = match model {
        m if m == LanguageModel::O1High || m == LanguageModel::O4MiniHigh => {
            openai_client.chat().create(request).await
        }
        LanguageModel::Doubao15Pro256k => {
            openai_compatible_clients
                .volc_engine_client
                .chat()
                .create(request)
                .await
        }
        LanguageModel::KimiLatest => {
            openai_compatible_clients
                .moonshot_client
                .chat()
                .create(request)
                .await
        }
        LanguageModel::Step16k => {
            openai_compatible_clients
                .step_client
                .chat()
                .create(request)
                .await
        }
        LanguageModel::Glm4Plus => {
            openai_compatible_clients
                .zhipu_client
                .chat()
                .create(request)
                .await
        }
        _ => {
            openai_compatible_clients
                .open_router_client
                .chat()
                .create(request)
                .await
        }
    };

    match result {
        Ok(response) => response.choices[0]
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Sorry, but I can't seem to translate that!")),
        Err(e) => Err(anyhow::anyhow!("Failed to send Open Router request: {}", e)),
    }
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

    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(prompt),
            name: None,
        }),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model(MISTRAL_LARGE_2411_MODEL)
        .messages(messages)
        .temperature(TEMPERATURE)
        .build()?;

    match data
        .openai_compatible_clients
        .open_router_client
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
    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: CATEGORIZE_QUESTION_SYSTEM_PROMPT.into(),
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(message),
            name: None,
        }),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model(MISTRAL_LARGE_2411_MODEL)
        .temperature(TEMPERATURE)
        .messages(messages)
        .build()?;

    match data
        .openai_compatible_clients
        .open_router_client
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

    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(message_chain.join("\n")),
            name: None,
        }),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .temperature(TEMPERATURE)
        .model(MISTRAL_LARGE_2411_MODEL)
        .messages(messages)
        .build()?;

    match data
        .openai_compatible_clients
        .open_router_client
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

    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: ChatCompletionRequestSystemMessageContent::Text(system_prompt),
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(previous_messages),
            name: None,
        }),
    ];

    let request = CreateChatCompletionRequestArgs::default()
        .model(MISTRAL_LARGE_2411_MODEL)
        .temperature(TEMPERATURE)
        .messages(messages)
        .build()?;

    match data
        .openai_compatible_clients
        .open_router_client
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
