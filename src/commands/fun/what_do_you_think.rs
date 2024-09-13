use crate::shared::services::open_router_service::opine_specific;
use crate::shared::structs::{ContextData, ContextError};
use crate::shared::utility::get_author_name;
use poise::CreateReply;
use serenity::all::Message;
use std::borrow::Cow;

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

    match opine_specific(ctx.data, prompt).await {
        Ok(response) => {
            ctx.send(CreateReply::default().content(response)).await?;
        }
        Err(e) => {
            let error_message =
                format!("An error occurred when answering what do you think: {}", e);
            tracing::error!("{}", &error_message);
            ctx.send(CreateReply::default().content(error_message))
                .await?;
        }
    }

    Ok(())
}
