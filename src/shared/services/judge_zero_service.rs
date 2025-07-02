#![allow(unused)]
use crate::shared::constants::{KOU_COLOR, RUST_LOGO, TAIGA_COLOR};
use crate::shared::structs::Context;
use crate::shared::structs::utility::judge_zero::{
    JudgeZeroGetResponse, JudgeZeroPostRequest, JudgeZeroPostResponse, JudgeZeroRequestResult,
};
use base64::Engine;
use base64::engine::{GeneralPurpose, GeneralPurposeConfig, general_purpose};
use once_cell::sync::OnceCell;
use reqwest::header::HeaderMap;
use serenity::all::CreateEmbedAuthor;
use serenity::builder::CreateEmbed;

const RUST_LANG_CODE: u8 = 73;
const ENDPOINT: &str = "judge0-ce.p.rapidapi.com";
const SUBMISSION_URL: &str =
    "https://judge0-ce.p.rapidapi.com/submissions?base64_encoded=true&fields=*";
const RESULT_RAW_URL: &str =
    "https://judge0-ce.p.rapidapi.com/submissions/{token}?base64_encoded=true&fields=*";
const DEFAULT_MAX_ATTEMPTS: u8 = 10;

static HEADER_MAP: OnceCell<HeaderMap> = OnceCell::new();

pub fn build_embed(
    ctx: Context<'_>,
    response: JudgeZeroGetResponse,
    author_name: &str,
    author_avatar_url: &str,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let content = if is_kou {
        format!(
            "Hey, {author_name}! I tried my best and this is what I got for you! <a:kou_anime:700020702585290782>\n```rust\n"
        )
    } else {
        format!(
            "Guess I have to lend my hand to you because you're just like Eduard and Lee, {author_name}! <:TaigaSmug:702210822310723614>\n```rust\n"
        )
    };

    let content = response
        .stdout
        .map(|s| format!("{}{}", &content, s))
        .unwrap_or_else(|| content);

    let content = response
        .compile_output
        .map(|s| format!("{}{}", &content, s))
        .unwrap_or_else(|| content);

    let content = content + "\n```";

    embed = embed
        .author(CreateEmbedAuthor::new(author_name).icon_url(author_avatar_url))
        .color(color)
        .description(content)
        .thumbnail(RUST_LOGO);

    if let Some(time_spent) = response.time {
        embed = embed.field("Time Spent", format!("{time_spent} sec"), true);
    }

    if let Some(memory) = response.memory {
        embed = embed.field("Memory Spent", format!("{memory} KB"), true);
    }

    embed
}

pub async fn create_eval_request(ctx: Context<'_>, source_code: String) -> anyhow::Result<String> {
    let request = JudgeZeroPostRequest {
        language_id: RUST_LANG_CODE,
        source_code: general_purpose::STANDARD.encode(&source_code),
    };

    let header_map = HEADER_MAP.get_or_init(|| initialize_header_map(ctx));

    let response = ctx
        .data()
        .http_client
        .post(SUBMISSION_URL)
        .json(&request)
        .headers(header_map.clone())
        .send()
        .await?
        .json::<JudgeZeroPostResponse>()
        .await?;

    Ok(response.token)
}

pub async fn try_get_eval_result(
    ctx: Context<'_>,
    token: String,
) -> anyhow::Result<JudgeZeroRequestResult> {
    let result_url = RESULT_RAW_URL.replace("{token}", &token);
    let mut result = get_eval_result(ctx, &result_url).await?;

    if result != JudgeZeroRequestResult::InProgress {
        return Ok(result);
    }

    for _ in 0..DEFAULT_MAX_ATTEMPTS {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        result = get_eval_result(ctx, &result_url).await?;
        if result != JudgeZeroRequestResult::InProgress {
            break;
        }
    }

    Ok(result)
}

fn initialize_header_map(ctx: Context<'_>) -> HeaderMap {
    let api_key = ctx.data().config.rapid_api_key.clone();

    let mut header_map = HeaderMap::new();
    header_map.insert(
        "x-rapidapi-key",
        api_key
            .parse()
            .expect("Failed to parse Rapid API key for header value."),
    );
    header_map.insert(
        "x-rapidapi-host",
        ENDPOINT
            .parse()
            .expect("Failed to parse Rapid API key for header value."),
    );
    header_map
}

async fn get_eval_result(
    ctx: Context<'_>,
    result_url: &str,
) -> anyhow::Result<JudgeZeroRequestResult> {
    let header_map = HEADER_MAP.get_or_init(|| initialize_header_map(ctx));
    let response = ctx
        .data()
        .http_client
        .get(result_url)
        .headers(header_map.clone())
        .send()
        .await?
        .json::<JudgeZeroGetResponse>()
        .await?;

    let response = handle_response(response);
    let error_occurred = response.stderr.is_some() || response.message.is_some();
    if error_occurred {
        return Ok(JudgeZeroRequestResult::Failed(response));
    }

    let in_progress = response.stdout.is_none() && response.compile_output.is_none();
    if in_progress {
        return Ok(JudgeZeroRequestResult::InProgress);
    }

    Ok(JudgeZeroRequestResult::Success(response))
}

fn handle_response(response: JudgeZeroGetResponse) -> JudgeZeroGetResponse {
    let stderr = response.stderr.map(decode_base64);
    let message = response.message.map(decode_base64);
    let stdout = response.stdout.map(decode_base64);
    let compile_output = response.compile_output.map(decode_base64);
    JudgeZeroGetResponse {
        stderr,
        message,
        stdout,
        compile_output,
        ..response
    }
}

fn decode_base64(s: String) -> String {
    if !s.is_empty() && s.chars().count() > 0 {
        let config = GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true);
        let alphabets = base64::alphabet::STANDARD;
        let engine = GeneralPurpose::new(&alphabets, config);
        let sanitized_string = s.trim().replace('\n', "");
        let decode_result = engine.decode(sanitized_string);
        let bytes = if let Err(ref e) = decode_result {
            tracing::error!("Error when decoding base64 text: {}", e);
            vec![]
        } else {
            decode_result.unwrap_or_default()
        };

        let string_result = String::from_utf8(bytes);
        if let Err(ref e) = string_result {
            tracing::error!("Error when building string from utf8 vector: {}", e);
            String::new()
        } else {
            string_result.unwrap_or_default()
        }
    } else {
        s
    }
}
