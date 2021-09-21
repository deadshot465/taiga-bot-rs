#![allow(clippy::too_many_arguments)]
use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::services::ship_service::{
    calculate_ship_score, download_avatar, generate_ship_image, get_ship_message,
    monochrome_if_lower_score,
};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::utility::{find_user_in_members, get_author_avatar, get_author_name};
use serenity::model::id::UserId;
use serenity::model::prelude::application_command::{
    ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
};
use serenity::model::prelude::User;
use serenity::prelude::*;
use serenity::utils::Color;
use std::future::Future;
use std::pin::Pin;

pub fn ship_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(ship(ctx, command))
}

async fn ship(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Alright! Hold on..."))
        })
        .await?;

    let user_1 = command
        .data
        .options
        .get(0)
        .and_then(|opt| opt.resolved.as_ref())
        .map(|resolved| {
            if let ApplicationCommandInteractionDataOptionValue::User(u, _) = resolved {
                u.clone()
            } else {
                User::default()
            }
        });

    let user_2 = command
        .data
        .options
        .get(1)
        .and_then(|opt| opt.resolved.as_ref())
        .map(|resolved| {
            if let ApplicationCommandInteractionDataOptionValue::User(u, _) = resolved {
                u.clone()
            } else {
                User::default()
            }
        });

    let user_1_id = user_1.clone().map(|u| u.id.0).unwrap_or_default();
    let user_2_id = user_2.clone().map(|u| u.id.0).unwrap_or_default();
    let ship_score = calculate_ship_score(user_1_id, user_2_id);

    let mut user_1_avatar_url = user_1
        .clone()
        .map(|user| get_author_avatar(&user))
        .unwrap_or_default();
    let mut user_2_avatar_url = user_2
        .clone()
        .map(|user| get_author_avatar(&user))
        .unwrap_or_default();

    user_1_avatar_url = monochrome_if_lower_score(ship_score, user_1_avatar_url);
    user_2_avatar_url = monochrome_if_lower_score(ship_score, user_2_avatar_url);

    let user_1_avatar = download_avatar(&user_1_avatar_url).await?;
    let user_2_avatar = download_avatar(&user_2_avatar_url).await?;

    let members = command
        .guild_id
        .unwrap_or_default()
        .members(&ctx.http, None::<u64>, None::<UserId>)
        .await?;
    let user_1_member = user_1
        .clone()
        .and_then(|user| find_user_in_members(user, &members));
    let user_2_member = user_2
        .clone()
        .and_then(|user| find_user_in_members(user, &members));

    let user_1_display_name = get_author_name(
        &user_1.expect("Failed to get user 1."),
        &user_1_member.cloned(),
    );
    let user_2_display_name = get_author_name(
        &user_2.expect("Failed to get user 1."),
        &user_2_member.cloned(),
    );

    let ship_msg = get_ship_message(ship_score)
        .replace("$1", &user_1_display_name)
        .replace("$2", &user_2_display_name);

    let result_handle =
        tokio::spawn(async move { generate_ship_image(&user_1_avatar, &user_2_avatar) });

    let ship_score_text = format!("Your love score is {}!", ship_score);
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    match result_handle.await? {
        Ok(result) => {
            send_ship_embed(
                &ctx,
                &command,
                result,
                ship_score_text,
                user_1_display_name,
                user_2_display_name,
                ship_msg,
                color,
            )
            .await?
        }
        Err(e) => {
            log::warn!("{}. Retrying with PNG...", e.to_string());

            match retry_with_png_if_error(&user_1_avatar_url, &user_2_avatar_url).await {
                Ok(result) => {
                    send_ship_embed(
                        &ctx,
                        &command,
                        result,
                        ship_score_text,
                        user_1_display_name,
                        user_2_display_name,
                        ship_msg,
                        color,
                    )
                    .await?
                }
                Err(e) => {
                    log::error!("{}", e.to_string());
                    command
                        .edit_original_interaction_response(&ctx.http, |response| {
                            response.content(format!("Sorry, an occurred! Error: {}", e))
                        })
                        .await?;
                }
            }
        }
    }
    Ok(())
}

async fn retry_with_png_if_error(
    avatar_url_1: &str,
    avatar_url_2: &str,
) -> anyhow::Result<Vec<u8>> {
    let avatar_url_1 = avatar_url_1.replace(".webp", ".png");
    let avatar_url_2 = avatar_url_2.replace(".webp", ".png");
    let user_1_avatar = download_avatar(&avatar_url_1).await?;
    let user_2_avatar = download_avatar(&avatar_url_2).await?;
    generate_ship_image(&user_1_avatar, &user_2_avatar)
}

async fn send_ship_embed(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    image: Vec<u8>,
    ship_score_text: String,
    user_1_display_name: String,
    user_2_display_name: String,
    ship_msg: String,
    color: Color,
) -> anyhow::Result<()> {
    command
        .edit_original_interaction_response(&ctx.http, |response| {
            response.content(&ship_score_text)
        })
        .await?;
    let files = vec![(image.as_slice(), "result.png")];
    command
        .channel_id
        .send_files(&ctx.http, files, |m| {
            m.embed(|embed| {
                embed
                    .title(format!(
                        "{} and {}",
                        user_1_display_name, user_2_display_name
                    ))
                    .description(format!("{}\n\n{}", ship_score_text, ship_msg))
                    .attachment("result.png")
                    .color(color)
            })
        })
        .await?;
    Ok(())
}
