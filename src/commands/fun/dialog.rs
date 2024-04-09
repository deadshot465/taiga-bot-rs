use poise::CreateReply;
use serenity::all::{CreateAttachment, CreateMessage};

use crate::shared::services::dialog_service::{get_dialog, validate_dialog};
use crate::shared::structs::authentication::login;
use crate::shared::structs::{Context, ContextError};

/// Returns an image of a character saying anything you want.
#[poise::command(slash_command, category = "Fun")]
pub async fn dialog(
    ctx: Context<'_>,
    #[description = "The background of the character. A random background if the specified one doesn't exist."]
    background: String,
    #[description = "The character whom you want to make saying something."] character: String,
    #[description = "The text of the dialog. Cannot be over 180 characters."] text: String,
) -> Result<(), ContextError> {
    let mut background = background.to_lowercase();
    let character = character.to_lowercase();
    let text = text.trim().to_string();

    let is_kou = ctx.data().kou;
    let reply_handle = ctx
        .send(CreateReply::default().content(if is_kou {
            "Ok...Just give me sometime to figure out how to do this. <:KouConfused:717495654003245076>"
        } else {
            "Fine! I, Taiga Akatora, will give you the result as soon as possible. <:TaigaClimb:699710154861445172>"
        })).await?;

    if let Err(e) = validate_dialog(ctx, &mut background, &character, &text, is_kou).await {
        reply_handle
            .edit(ctx, CreateReply::default().content(e.to_string()))
            .await?;
        return Ok(());
    }
    login(ctx.data()).await?;

    let result = get_dialog(ctx, background, character, text).await?;

    let files = [CreateAttachment::bytes(result, "result.png")];
    ctx.channel_id()
        .send_files(
            ctx.http(),
            files,
            CreateMessage::new().content("Here you go~!"),
        )
        .await?;

    Ok(())
}
