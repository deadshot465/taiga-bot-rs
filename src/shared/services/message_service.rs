use crate::shared::structs::authentication::login;
use crate::shared::structs::record::message::{
    GetMessageRequest, GetMessageResponse, MessageInfo, MessageRecordSimple,
};
use crate::shared::structs::ContextData;
use serenity::all::{Context, Message};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

pub async fn record_message(
    ctx: &Context,
    message: &Message,
    data: &ContextData,
    endpoint: String,
) -> anyhow::Result<()> {
    login(data).await?;

    let bot_user = ctx.http.get_current_user().await?;
    let user_name = message
        .author_nick(&ctx.http)
        .await
        .or(Some(message.author.name.clone()));
    let user_id = message.author.id.get().to_string();

    let payload = MessageInfo {
        bot_id: bot_user.id.get().to_string(),
        user_id: user_id.clone(),
        user_name: user_name.clone(),
        message: message.content.clone(),
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
        let error_message = format!(
            "Failed to record message for {} ({:?}): {}",
            user_id, user_name, e
        );
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
