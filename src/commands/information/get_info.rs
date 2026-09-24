use std::time::Duration;

use itertools::Itertools;
use poise::CreateReply;
use reqwest::Client;
use serenity::builder::{CreateAttachment, CreateMessage};

use crate::shared::{
    constants::DISCORD_MESSAGE_HARD_LENGTH_LIMIT,
    structs::{
        Context, ContextError,
        authentication::login,
        novel::{
            CodexSummaryContainerResponse, CodexSummaryContainerStatusResponse,
            CodexSummaryRequest, CodexSummaryRequestedLanguage, CodexSummaryResponseLogs, Novel,
        },
    },
};

const DEFAULT_INSTRUCTION: &str = r#"Write in $language and make sure to create the card if the image is available.
Your response will be shown directly in my application, so only return the summary part after creating the card (or not creating the card)."#;

#[poise::command(slash_command, category = "Information")]
pub async fn get_info(
    ctx: Context<'_>,
    #[description = "The novel to query info from. Leave it empty to get a merged info."]
    novel: Option<Novel>,
    #[description = "The name of the codex you want to get info about."] keyword: String,
    #[description = "The requested word count of the codex summary. Minimum 100. Default to 150."]
    #[min = 100]
    word_count: Option<u16>,
    #[description = "The language of the info to present in."]
    language: CodexSummaryRequestedLanguage,
    #[description = "Additional instructions to attach after the language specification."]
    additional_instructions: Option<String>,
) -> Result<(), ContextError> {
    let novel = novel.map(|n| match n {
        Novel::ForgedInStarlight => "Forged In Starlight".to_string(),
        Novel::Chronosplit => "Chronosplit".to_string(),
    });

    let summary_language = match language {
        CodexSummaryRequestedLanguage::ZhTw => "traditional Chinese",
        CodexSummaryRequestedLanguage::JaJp => "Japanese",
        CodexSummaryRequestedLanguage::EnUs => "English",
    };

    let mut prompt = DEFAULT_INSTRUCTION.replace("$language", summary_language);

    if let Some(instructions) = additional_instructions {
        prompt.push(' ');
        prompt.push_str(&instructions);
    }

    let payload = CodexSummaryRequest {
        keyword,
        word_count: word_count.unwrap_or(150) as i32,
        novel,
        additional_instructions: Some(prompt),
        request_language: language,
        push_to_line: false,
        schedule_polling: false,
    };

    ctx.defer().await?;

    login(ctx.data()).await?;
    let auth = ctx.data().authentication.clone();

    let endpoint = format!(
        "{}/novel/summary",
        ctx.data().config.server_endpoint.as_str()
    );

    let response = ctx
        .data()
        .http_client
        .post(&endpoint)
        .json(&payload)
        .bearer_auth(auth.read().await.token.clone())
        .send()
        .await?;

    let bytes = response.bytes().await?;

    if let Ok(decoded) = serde_json::from_slice::<CodexSummaryResponseLogs>(&*bytes) {
        tracing::info!("Decoded Payload: {:?}", &decoded);
        build_replies(ctx, decoded).await?;
        return Ok(());
    }

    let container_id =
        serde_json::from_slice::<CodexSummaryContainerResponse>(&*bytes)?.container_id;

    let server_endpoint = ctx.data().config.server_endpoint.clone();
    let container_id_clone = container_id.clone();
    let http_client_clone = ctx.data().http_client.clone();
    let auth_token_clone = ctx.data().authentication.read().await.token.clone();

    let payload = tokio::spawn(async move {
        poll(
            server_endpoint,
            container_id_clone,
            http_client_clone,
            auth_token_clone,
        )
        .await
    })
    .await??;

    build_replies(ctx, payload).await?;

    Ok(())
}

async fn poll(
    server_endpoint: String,
    container_id: String,
    http_client: Client,
    auth_token: String,
) -> anyhow::Result<CodexSummaryResponseLogs> {
    let endpoint = format!("{}/novel/summary/{}", server_endpoint, container_id);
    let sleep = tokio::time::sleep(Duration::from_mins(10));
    let mut timeout = std::pin::pin!(sleep);

    let mut interval = tokio::time::interval(Duration::from_secs(10));
    interval.tick().await;
    let mut timed_out = false;

    loop {
        // let endpoint_clone = endpoint.clone();

        tokio::select! {
            _ = &mut timeout => {
                timed_out = true;
                break;
            }

            _ = interval.tick() => {
                let response = http_client
                    .get(&endpoint)
                    .bearer_auth(&auth_token)
                    .send()
                    .await;

                if let Ok(res) = response
                    && let Ok(payload) = res.json::<CodexSummaryContainerStatusResponse>().await
                    && let lowercase_status = payload.state.status.to_lowercase()
                    && (lowercase_status == "exited" || lowercase_status == "dead") {
                        break;
                }
            }
        }
    }

    if timed_out {
        return Err(anyhow::anyhow!("Timed out."));
    }

    let endpoint = format!("{}/result", endpoint);

    let payload = http_client
        .get(&endpoint)
        .bearer_auth(&auth_token)
        .send()
        .await?
        .json::<CodexSummaryResponseLogs>()
        .await?;

    tracing::info!("Got payload: {:?}", &payload);

    Ok(payload)
}

async fn build_replies(ctx: Context<'_>, payload: CodexSummaryResponseLogs) -> anyhow::Result<()> {
    let has_image = !payload.images.is_empty();
    let image_count = payload.images.len();
    let full_output_length: usize = payload.outs.iter().map(|s| s.len()).sum();
    let mut full_output = payload.outs.join("\n");
    let mut texts_to_send = Vec::new();

    if has_image {
        let mut real_paths = payload
            .images
            .into_iter()
            .map(|s| {
                s.replace(
                    "Artifacts/",
                    &format!("{}/novel/artifacts/", &ctx.data().config.server_endpoint),
                )
            })
            .collect::<Vec<_>>();

        tracing::info!("Real paths: {:?}", &real_paths);

        let mut builder = CreateReply::new();
        let range = if image_count > 4 {
            0..4
        } else {
            0..image_count
        };

        for path in real_paths.drain(range) {
            builder = builder.attachment(CreateAttachment::url(ctx.http(), &path).await?);
        }

        let reply_handle = ctx.send(builder).await?;
        let channel = reply_handle.message().await?.channel(ctx.http()).await?;

        if !real_paths.is_empty() {
            for chunk in real_paths.chunks(4) {
                let mut attachments = Vec::with_capacity(chunk.len());

                for c in chunk.into_iter() {
                    attachments.push(CreateAttachment::url(ctx.http(), c).await?);
                }

                channel
                    .id()
                    .send_files(ctx.http(), attachments, CreateMessage::new())
                    .await?;

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }

        for chunk in full_output
            .chars()
            .chunks(DISCORD_MESSAGE_HARD_LENGTH_LIMIT)
            .into_iter()
        {
            let s = chunk.collect::<String>();
            texts_to_send.push(s);
        }
    } else if full_output_length > DISCORD_MESSAGE_HARD_LENGTH_LIMIT {
        let first_message = full_output
            .drain(..DISCORD_MESSAGE_HARD_LENGTH_LIMIT)
            .collect::<String>();

        let _ = ctx.send(CreateReply::new().content(first_message)).await?;

        for chunk in full_output
            .chars()
            .chunks(DISCORD_MESSAGE_HARD_LENGTH_LIMIT)
            .into_iter()
        {
            let s = chunk.collect::<String>();
            texts_to_send.push(s);
        }
    } else {
        let _ = ctx.send(CreateReply::new().content(full_output)).await?;
        return Ok(());
    }

    for s in texts_to_send.into_iter() {
        ctx.channel_id()
            .send_message(ctx.http(), CreateMessage::new().content(s))
            .await?;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
