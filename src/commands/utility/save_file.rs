use poise::CreateReply;
use serenity::all::Message;

use crate::shared::structs::authentication::login;
use crate::shared::structs::utility::save_file::SaveFileRequest;
use crate::shared::structs::{ContextData, ContextError};

#[derive(Debug, Clone, poise::Modal)]
#[name = "Save File to Tetsu's Server"]
struct FilenameModal {
    #[placeholder = "The filename of the file on the server"]
    #[min_length = 1]
    #[max_length = 100]
    pub filename: String,
}

#[poise::command(
    context_menu_command = "Save File to Tetsu's Server",
    required_permissions = "ADMINISTRATOR",
    owners_only
)]
pub async fn save_file(
    ctx: poise::ApplicationContext<'_, ContextData, ContextError>,
    message: Message,
) -> Result<(), ContextError> {
    use poise::Modal as _;

    if message.attachments.is_empty() {
        ctx.send(CreateReply::default().content("I can't find any attachment in the message!"))
            .await?;
        return Ok(());
    }

    let modal_data = FilenameModal::execute(ctx).await?;

    if let Some(data) = modal_data {
        let file_url = message.attachments[0].url.clone();
        let request = SaveFileRequest {
            filename: data.filename,
            file_url,
        };

        let endpoint = format!("{}/{}", &ctx.data.config.server_endpoint, "save_file");

        login(ctx.data).await?;
        let auth = ctx.data.authentication.clone();

        let response = ctx
            .data
            .http_client
            .post(endpoint)
            .json(&request)
            .bearer_auth(auth.read().await.token.clone())
            .send()
            .await?
            .error_for_status();

        if let Err(e) = response {
            tracing::error!("Failed to save file to the server: {}", e);
            ctx.send(CreateReply::default().content(format!(
                "An error occurred when saving file to the server!\nError: {e:?}"
            )))
            .await?;
        } else {
            ctx.send(CreateReply::default().content("Successfully saved file to Tetsu's server!"))
                .await?;
        }
    }

    Ok(())
}
