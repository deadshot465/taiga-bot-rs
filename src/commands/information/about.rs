use serenity::all::{CreateEmbedAuthor, CreateEmbedFooter};
use serenity::builder::CreateEmbed;

use crate::shared::constants::{CAMP_BUDDY_STAR, KOU_COLOR, RUST_LOGO, TAIGA_COLOR};
use crate::shared::structs::{Context, ContextError};

const ABOUT_KOU_PATH: &str = "assets/txt/about_kou.txt";
const ABOUT_TAIGA_PATH: &str = "assets/txt/about_taiga.txt";

/// Shows information about the bot.
#[poise::command(slash_command, category = "Information")]
pub async fn about(ctx: Context<'_>) -> Result<(), ContextError> {
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let configuration = &ctx.data().config;

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
        let current_user = ctx.http().get_current_user().await?;
        current_user
            .avatar_url()
            .unwrap_or_else(|| current_user.default_avatar_url())
    } else {
        CAMP_BUDDY_STAR.to_string()
    };

    ctx.send(
        poise::CreateReply::default().embed(
            CreateEmbed::new()
                .author(CreateEmbedAuthor::new(author_name).icon_url(author_icon))
                .color(color)
                .footer(CreateEmbedFooter::new(footer))
                .description(description)
                .thumbnail(RUST_LOGO),
        ),
    )
    .await?;
    Ok(())
}
