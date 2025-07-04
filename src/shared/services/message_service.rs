use serenity::all::{Context, Message};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::shared::constants::{IMAGE_TYPES, KOU_SERVER_ID};
use crate::shared::structs::ContextData;
use crate::shared::structs::authentication::login;
use crate::shared::structs::record::message::{
    GetMessageRequest, GetMessageResponse, MessageInfo, MessageRecordSimple,
};

pub async fn record_message(
    ctx: &Context,
    message: &Message,
    data: &ContextData,
    endpoint: String,
) -> anyhow::Result<()> {
    let author_id_skippable = data.config.skip_user_ids.contains(&message.author.id.get());

    if author_id_skippable || message.author.bot {
        return Ok(());
    }

    let guild_id = message.guild_id.unwrap_or_default().get();
    if !data.kou && guild_id != KOU_SERVER_ID {
        return Ok(());
    }

    login(data).await?;

    let bot_user = ctx.http.get_current_user().await?;
    let user_name = message
        .author_nick(&ctx.http)
        .await
        .or(Some(message.author.name.clone()));
    let user_id = message.author.id.get().to_string();

    let attachment = message.attachments.first().filter(|&attachment| {
        if let Some(ref content_type) = attachment.content_type {
            IMAGE_TYPES.contains(&content_type.as_str())
        } else {
            false
        }
    });

    let (new_message, message_type) = if let Some(attachment) = attachment {
        (
            format!(
                "{} [image_url={}]",
                message.content.clone(),
                attachment.url.clone()
            ),
            "image".to_string(),
        )
    } else {
        (message.content.clone(), "text".to_string())
    };

    let payload = MessageInfo {
        bot_id: bot_user.id.get().to_string(),
        user_id: user_id.clone(),
        user_name: user_name.clone(),
        generated_by: None,
        message: new_message,
        message_type,
        channel_id: message.channel_id.get().to_string(),
        post_at: OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_default(),
    };

    let response = data
        .http_client
        .post(endpoint)
        .json(&payload)
        .bearer_auth(&data.authentication.read().await.token)
        .send()
        .await
        .and_then(|res| res.error_for_status());

    if let Err(e) = response {
        let error_message =
            format!("Failed to record message for {user_id} ({user_name:?}): {e:?}");
        tracing::error!("{}", error_message);
    }

    Ok(())
}

pub async fn get_messages(
    ctx: &Context,
    message: &Message,
    data: &ContextData,
) -> anyhow::Result<Vec<MessageRecordSimple>> {
    login(data).await?;

    let bot_user = ctx.http.get_current_user().await?;

    let payload = GetMessageRequest {
        bot_id: bot_user.id.get().to_string(),
        channel_id: message.channel_id.get().to_string(),
    };
    let endpoint = format!("{}/message/record/list", &data.config.server_endpoint);

    let response = data
        .http_client
        .post(endpoint)
        .json(&payload)
        .bearer_auth(&data.authentication.read().await.token)
        .send()
        .await
        .and_then(|res| res.error_for_status())?;

    let response = response.json::<GetMessageResponse>().await?;
    Ok(response.messages)
}
