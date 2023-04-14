use crate::shared::constants::{KOU_COLOR, RUST_LOGO, TAIGA_COLOR};
use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use crate::shared::structs::utility::judge_zero::{
    JudgeZeroGetResponse, JudgeZeroPostRequest, JudgeZeroPostResponse, JudgeZeroRequestResult,
};
use base64::engine::{general_purpose, GeneralPurpose, GeneralPurposeConfig};
use base64::Engine;
use once_cell::sync::Lazy;
use reqwest::header::HeaderMap;
use serenity::builder::CreateEmbed;

const RUST_LANG_CODE: u8 = 73;
const ENDPOINT: &str = "judge0-ce.p.rapidapi.com";
const SUBMISSION_URL: &str =
    "https://judge0-ce.p.rapidapi.com/submissions?base64_encoded=true&fields=*";
const RESULT_RAW_URL: &str =
    "https://judge0-ce.p.rapidapi.com/submissions/{token}?base64_encoded=true&fields=*";
const DEFAULT_MAX_ATTEMPTS: u8 = 10;

static HEADER_MAP: Lazy<HeaderMap> = Lazy::new(|| {
    let api_key = CONFIGURATION
        .get()
        .map(|c| c.rapid_api_key.as_str())
        .unwrap_or_default();

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
});

pub fn build_embed(
    response: JudgeZeroGetResponse,
    author_name: &str,
    author_avatar_url: &str,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let content = if is_kou {
        format!("Hey, {}! I tried my best and this is what I got for you! <a:kou_anime:700020702585290782>\n```rust\n", author_name)
    } else {
        format!("Guess I have to lend my hand to you because you're just like Eduard and Lee, {}! <:TaigaSmug:702210822310723614>\n```rust\n", author_name)
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

    embed
        .author(|a| a.name(author_name).icon_url(author_avatar_url))
        .color(color)
        .description(content)
        .thumbnail(RUST_LOGO);

    if let Some(time_spent) = response.time {
        embed.field("Time Spent", format!("{} sec", time_spent), true);
    }

    if let Some(memory) = response.memory {
        embed.field("Memory Spent", format!("{} KB", memory), true);
    }

    embed
}

pub async fn create_eval_request(source_code: String) -> anyhow::Result<String> {
    let request = JudgeZeroPostRequest {
        language_id: RUST_LANG_CODE,
        source_code: general_purpose::STANDARD.encode(&source_code),
    };

    let response = HTTP_CLIENT
        .post(SUBMISSION_URL)
        .json(&request)
        .headers((*HEADER_MAP).clone())
        .send()
        .await?
        .json::<JudgeZeroPostResponse>()
        .await?;

    Ok(response.token)
}

pub async fn try_get_eval_result(token: String) -> anyhow::Result<JudgeZeroRequestResult> {
    let result_url = RESULT_RAW_URL.replace("{token}", &token);
    let mut result = get_eval_result(&result_url).await?;

    if result != JudgeZeroRequestResult::InProgress {
        return Ok(result);
    }

    for _ in 0..DEFAULT_MAX_ATTEMPTS {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        result = get_eval_result(&result_url).await?;
        if result != JudgeZeroRequestResult::InProgress {
            break;
        }
    }

    Ok(result)
}

async fn get_eval_result(result_url: &str) -> anyhow::Result<JudgeZeroRequestResult> {
    let response = HTTP_CLIENT
        .get(result_url)
        .headers((*HEADER_MAP).clone())
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
