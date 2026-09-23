use crate::shared::constants::DISCORD_MESSAGE_HARD_LENGTH_LIMIT;
use crate::shared::services::open_router_service::opine_specific;
use crate::shared::structs::{ContextData, ContextError};
use crate::shared::utility::get_author_name;
use itertools::Itertools;
use poise::CreateReply;
use serenity::all::Message;
use serenity::builder::CreateMessage;

#[poise::command(context_menu_command = "What do you think?")]
pub async fn what_do_you_think(
    ctx: poise::ApplicationContext<'_, ContextData, ContextError>,
    message: Message,
) -> Result<(), ContextError> {
    let author = ctx.author();
    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author_name = get_author_name(author, &member);

    let prompt = format!(
        "{} said \
    <quote> \
    {} \
    </quote> \
    What do you think of this take?",
        &author_name, &message.content
    );

    ctx.defer().await?;

    match opine_specific(ctx.data(), prompt).await {
        Ok(response) => {
            let length = response.len();
            let mut messages = Vec::new();

            if length > DISCORD_MESSAGE_HARD_LENGTH_LIMIT {
                for c in response.chars().chunks(1000).into_iter() {
                    let text = c.collect::<String>();
                    messages.push(text);
                }
            } else {
                messages.push(response);
            }

            let first = &messages[0];
            let remaining = &messages[1..];

            let reply_handle = ctx.send(CreateReply::default().content(first)).await?;
            let channel = reply_handle.message().await?.channel(ctx.http()).await?;

            for m in remaining.into_iter() {
                channel
                    .id()
                    .send_message(ctx.http(), CreateMessage::new().content(m))
                    .await?;
            }
        }
        Err(e) => {
            let error_message =
                format!("An error occurred when answering what do you think: {e:?}");
            tracing::error!("{}", &error_message);
            ctx.send(CreateReply::default().content(error_message))
                .await?;
        }
    }

    Ok(())
}
