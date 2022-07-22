use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::services::image_service::{get_cat_image, get_dog_image, get_normal_image};
use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::config::configuration::KOU;
use crate::shared::utility::{get_author_avatar, get_author_name};
use rand::prelude::*;
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn image_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(image(ctx, command))
}

async fn image(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let author_name = get_author_name(&command.user, &command.member);
    let author_avatar_url = get_author_avatar(&command.user);
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Alright! Hold on..."))
        })
        .await?;

    // Dispatch to different image services (cat, image, dog).
    if let Some(opt) = command.data.options.get(0) {
        let mut keyword = opt
            .options
            .get(0)
            .and_then(|opt| opt.value.as_ref())
            .and_then(|value| value.as_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let result = match opt.name.as_str() {
            "image" => {
                if keyword.is_empty() {
                    keyword = "burger".into();
                }
                get_normal_image(&keyword, &*HTTP_CLIENT, &author_name, &author_avatar_url, color).await
            }
            "cat" => {
                if thread_rng().gen_range(0..2) > 0 {
                    // Invoke the Cat API
                    get_cat_image(&keyword, &*HTTP_CLIENT, &author_name, &author_avatar_url, color).await
                } else {
                    // Invoke Unsplash API
                    keyword = if keyword.is_empty() {
                        "cat".into()
                    } else {
                        "cat ".to_string() + &keyword
                    };
                    get_normal_image(&keyword, &*HTTP_CLIENT, &author_name, &author_avatar_url, color)
                        .await
                }
            }
            "dog" => {
                if thread_rng().gen_range(0..2) > 0 {
                    // Invoke Dog API
                    get_dog_image(&keyword, &*HTTP_CLIENT, &author_name, &author_avatar_url, color).await
                } else {
                    // Invoke Unsplash API
                    keyword = if keyword.is_empty() {
                        "dog".into()
                    } else {
                        "dog ".to_string() + &keyword
                    };
                    get_normal_image(&keyword, &*HTTP_CLIENT, &author_name, &author_avatar_url, color)
                        .await
                }
            }
            _ => Err(anyhow::anyhow!("Failed to execute the image command.")),
        }
        .unwrap_or_else(|e| {
            log::error!("{}", e);
            let mut embed = CreateEmbed::default();
            embed.description(if is_kou {
                "Sorry...I don't understand the keyword and cannot find anything... <:KouCry:705054435826597928>"
            } else {
                "Sorry. Not my problem. Your keyword is too weird that I can't find any image."
            });
            embed
        });

        command
            .edit_original_interaction_response(&ctx.http, |response| {
                let embeds = vec![result];
                response.content("").set_embeds(embeds)
            })
            .await?;
    }

    Ok(())
}
