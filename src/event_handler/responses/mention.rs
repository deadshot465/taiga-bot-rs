use crate::shared::services::open_router_service::{
    categorize_question, opine_conversation, opine_specific,
};
use crate::shared::structs::ContextData;
use serenity::model::prelude::Message;
use serenity::prelude::*;

pub async fn handle_mention_self(
    ctx: &Context,
    new_message: &Message,
    data: &ContextData,
) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    if new_message
        .content
        .contains(&data.config.bot_id.to_string())
    {
        match categorize_question(data, new_message.content.clone()).await {
            Ok(result) => {
                if result.starts_with("YES") {
                    let index = result.find("Question:").unwrap_or_default();
                    let index = index + 9;
                    let (_, question) = result.split_at(index);
                    let question = question.trim().into();

                    match opine_specific(data, question).await {
                        Ok(response) => {
                            new_message.reply(&ctx.http, response).await?;
                        }
                        Err(e) => {
                            let error_message = format!("Failed to reply to mention: {}", e);
                            tracing::error!("{}", &error_message);
                            new_message.reply(&ctx.http, error_message).await?;
                        }
                    }
                } else {
                    match opine_conversation(ctx, data, new_message).await {
                        Ok(response) => {
                            new_message.reply(&ctx.http, response).await?;
                        }
                        Err(e) => {
                            let error_message = format!("Failed to reply to mention: {}", e);
                            tracing::error!("{}", &error_message);
                            new_message.reply(&ctx.http, error_message).await?;
                        }
                    }
                }
            }
            Err(e) => {
                let error_message = format!("Failed to reply to mention: {}", e);
                tracing::error!("{}", &error_message);
                new_message.reply(&ctx.http, error_message).await?;
            }
        }
    }

    Ok(())
}
