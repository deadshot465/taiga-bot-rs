use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use serenity::model::prelude::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn avatar_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(avatar(ctx, command))
}

async fn avatar(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let emoji = if is_kou {
        "<:KouSugoi:705613007119450172>"
    } else {
        "<:TaigaFingerGunsLeft:702691580078850078>"
    };

    if let Some(option) = command.data.options.get(0) {
        if let ApplicationCommandInteractionDataOptionValue::User(user, _) = option
            .resolved
            .as_ref()
            .expect("Failed to resolve option value.")
        {
            let avatar_url = user.avatar_url().unwrap_or(user.default_avatar_url());
            let user_name = user.name.clone();

            command
                .create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|data| {
                        data.create_embed(|embed| {
                            embed
                                .title(&user_name)
                                .description(format!(
                                    "Here is {}'s avatar! {}\n**[Avatar URL]({})**",
                                    user_name, emoji, &avatar_url
                                ))
                                .color(color)
                                .image(avatar_url)
                        })
                    })
                })
                .await?;
        }
    } else {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("Sorry, I can't seem to execute your command!")
                })
            })
            .await?;
    }

    Ok(())
}
