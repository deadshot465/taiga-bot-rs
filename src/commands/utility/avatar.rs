use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::CreateEmbed;
use serenity::model::application::CommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn avatar_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(avatar(ctx, command))
}

async fn avatar(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let emoji = if is_kou {
        "<:KouSugoi:705613007119450172>"
    } else {
        "<:TaigaFingerGunsLeft:702691580078850078>"
    };

    let users = command
        .data
        .resolved
        .users
        .clone()
        .into_values()
        .collect::<Vec<_>>();

    if let Some(user) = users.get(0) {
        let avatar_url = user
            .avatar_url()
            .unwrap_or_else(|| user.default_avatar_url());
        let user_name = user.name.clone();

        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().embed(
                        CreateEmbed::new()
                            .title(&user_name)
                            .description(format!(
                                "Here is {}'s avatar! {}\n**[Avatar URL]({})**",
                                user_name, emoji, &avatar_url
                            ))
                            .color(color)
                            .image(avatar_url),
                    ),
                ),
            )
            .await?;
    } else {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Sorry, I can't seem to execute your command!"),
                ),
            )
            .await?;
    }

    Ok(())
}
