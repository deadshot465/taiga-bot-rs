use crate::shared::constants::{CAMP_BUDDY_STAR, KOU_COLOR, RUST_LOGO, TAIGA_COLOR};
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use std::future::Future;
use std::pin::Pin;

const ABOUT_KOU_PATH: &str = "assets/txt/about_kou.txt";
const ABOUT_TAIGA_PATH: &str = "assets/txt/about_taiga.txt";

pub fn about_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(about(ctx, command))
}

async fn about(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let configuration = CONFIGURATION.get().expect("Failed to get configuration.");

    let description = std::fs::read_to_string(if is_kou {
        ABOUT_KOU_PATH
    } else {
        ABOUT_TAIGA_PATH
    })?
    .replace("{VERSION}", &configuration.version_number);

    let footer = if is_kou {
        format!(
            "Kou Bot: Release {} | {}",
            &configuration.version_number, &configuration.update_date
        )
    } else {
        format!(
            "Taiga Bot: Release {} | {}",
            &configuration.version_number, &configuration.update_date
        )
    };

    let author_name = if is_kou {
        "Minamoto Kou from Jibaku Shōnen Hanako-kun"
    } else {
        "Taiga from Camp Buddy"
    };

    let author_icon = if is_kou {
        let current_user = ctx.http.get_current_user().await?;
        current_user
            .avatar_url()
            .unwrap_or_else(|| current_user.default_avatar_url())
    } else {
        CAMP_BUDDY_STAR.to_string()
    };

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.embed(|embed| {
                    embed
                        .author(|author| author.name(author_name).icon_url(&author_icon))
                        .color(color)
                        .footer(|f| f.text(footer))
                        .description(description)
                        .thumbnail(RUST_LOGO)
                })
            })
        })
        .await?;
    Ok(())
}
