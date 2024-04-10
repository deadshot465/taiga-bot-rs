use std::borrow::Cow;

use poise::CreateReply;
use rand::prelude::*;
use serenity::all::{Color, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::builder::CreateEmbed;

use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{get_author_avatar, get_author_name};

const THUMBNAIL_URL: &str = "https://cdn.discordapp.com/emojis/701918026164994049.png?v=1";

/// Draw an oracle and know the future of something on your mind.
#[poise::command(slash_command, category = "Information")]
pub async fn oracle(ctx: Context<'_>) -> Result<(), ContextError> {
    let oracle = {
        let mut rng = thread_rng();
        ctx.data()
            .oracles
            .choose(&mut rng)
            .expect("Failed to choose a oracle.")
    };

    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_icon = get_author_avatar(author);
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .author(CreateEmbedAuthor::new(&author_name).icon_url(&author_icon))
                .color(Color::new(0xff0000))
                .field("No", oracle.no.to_string(), true)
                .field("Meaning", &oracle.meaning, true)
                .footer(CreateEmbedFooter::new("Wish you good luck!"))
                .description(&oracle.content)
                .thumbnail(THUMBNAIL_URL)
                .title(&oracle.fortune),
        ),
    )
    .await?;
    Ok(())
}
