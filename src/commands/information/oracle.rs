use crate::shared::structs::information::oracle::ORACLES;
use crate::shared::utility::{get_author_avatar, get_author_name};
use rand::prelude::*;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Color;
use std::future::Future;
use std::pin::Pin;

const THUMBNAIL_URL: &str = "https://cdn.discordapp.com/emojis/701918026164994049.png?v=1";

pub fn oracle_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(oracle(ctx, command))
}

async fn oracle(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let oracle = {
        let mut rng = rand::thread_rng();
        ORACLES
            .choose(&mut rng)
            .expect("Failed to choose a oracle.")
    };

    let author_name = get_author_name(&command.user, &command.member);
    let author_icon = get_author_avatar(&command.user);
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.embed(|embed| {
                    embed
                        .author(|author| author.name(&author_name).icon_url(&author_icon))
                        .color(Color::new(0xff0000))
                        .field("No", oracle.no, true)
                        .field("Meaning", &oracle.meaning, true)
                        .footer(|f| f.text("Wish you good luck!"))
                        .description(&oracle.content)
                        .thumbnail(THUMBNAIL_URL)
                        .title(&oracle.fortune)
                })
            })
        })
        .await?;
    Ok(())
}
