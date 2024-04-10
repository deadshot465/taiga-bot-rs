use poise::CreateReply;
use serenity::all::User;
use serenity::builder::CreateEmbed;

use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::{Context, ContextError};

/// Get avatar/profile image of yourself or another user.
#[poise::command(slash_command, category = "Utility")]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "The user whose avatar to get."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    user: User,
) -> Result<(), ContextError> {
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let emoji = if is_kou {
        "<:KouSugoi:705613007119450172>"
    } else {
        "<:TaigaFingerGunsLeft:702691580078850078>"
    };

    let avatar_url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());

    let user_name = user.name.clone();

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title(&user_name)
                .description(format!(
                    "Here is {}'s avatar! {}\n**[Avatar URL]({})**",
                    user_name, emoji, &avatar_url
                ))
                .color(color)
                .image(avatar_url),
        ),
    )
    .await?;

    Ok(())
}
