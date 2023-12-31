use crate::shared::structs::information::oracle::ORACLES;
use crate::shared::utility::{get_author_avatar, get_author_name};
use rand::prelude::*;
use serenity::all::{
    Color, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::builder::CreateEmbed;
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;
use std::future::Future;
use std::pin::Pin;

const THUMBNAIL_URL: &str = "https://cdn.discordapp.com/emojis/701918026164994049.png?v=1";

pub fn oracle_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(oracle(ctx, command))
}

async fn oracle(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let oracle = {
        let mut rng = thread_rng();
        ORACLES
            .choose(&mut rng)
            .expect("Failed to choose a oracle.")
    };

    let member = command.member.clone().map(|m| *m.clone());
    let author_name = get_author_name(&command.user, &member);
    let author_icon = get_author_avatar(&command.user);
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(
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
            ),
        )
        .await?;
    Ok(())
}
