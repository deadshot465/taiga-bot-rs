use crate::shared::services::dialog_service::{get_dialog, validate_dialog};
use crate::shared::structs::authentication::login;
use crate::shared::structs::config::configuration::KOU;
use crate::shared::utility::extract_string_option;
use serenity::all::{
    CreateAttachment, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
    EditInteractionResponse,
};
use serenity::model::application::CommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn dialog_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(dialog(ctx, command))
}

async fn dialog(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let mut background = extract_string_option(&command, 0).to_lowercase();
    let character = extract_string_option(&command, 1).to_lowercase();
    let text = extract_string_option(&command, 2).trim().to_string();

    let is_kou = KOU.get().copied().unwrap_or(false);
    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .content(if is_kou {
                "Ok...Just give me sometime to figure out how to do this. <:KouConfused:717495654003245076>"
            } else {
                "Fine! I, Taiga Akatora, will give you the result as soon as possible. <:TaigaClimb:699710154861445172>"
            }))).await?;

    if let Err(e) = validate_dialog(&mut background, &character, &text, is_kou).await {
        command
            .edit_response(
                &ctx.http,
                EditInteractionResponse::new().content(e.to_string()),
            )
            .await?;
        return Ok(());
    }
    login().await?;

    let result = get_dialog(background, character, text).await?;

    let files = [CreateAttachment::bytes(result, "result.png")];
    command
        .channel_id
        .send_files(
            &ctx.http,
            files,
            CreateMessage::new().content("Here you go~!"),
        )
        .await?;

    Ok(())
}
