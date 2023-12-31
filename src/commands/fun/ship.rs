#![allow(clippy::too_many_arguments)]
use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::services::ship_service::{
    calculate_ship_score, download_avatar, generate_ship_image, get_ship_message,
    monochrome_if_lower_score,
};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::utility::{find_user_in_members, get_author_avatar, get_author_name};
use serenity::all::{
    Color, CreateAttachment, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateMessage, EditInteractionResponse,
};
use serenity::builder::CreateEmbed;
use serenity::model::application::CommandInteraction;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn ship_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(ship(ctx, command))
}

async fn ship(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Alright! Hold on..."),
            ),
        )
        .await?;

    let users = command
        .data
        .resolved
        .users
        .clone()
        .into_values()
        .collect::<Vec<_>>();

    let ship_score = calculate_ship_score(users[0].id.get(), users[1].id.get());

    let mut user_1_avatar_url = get_author_avatar(&users[0]);
    let mut user_2_avatar_url = get_author_avatar(&users[1]);

    user_1_avatar_url = monochrome_if_lower_score(ship_score, user_1_avatar_url);
    user_2_avatar_url = monochrome_if_lower_score(ship_score, user_2_avatar_url);

    let user_1_avatar = download_avatar(&user_1_avatar_url).await?;
    let user_2_avatar = download_avatar(&user_2_avatar_url).await?;

    let members = command
        .guild_id
        .unwrap_or_default()
        .members(&ctx.http, None::<u64>, None::<UserId>)
        .await?;
    let user_1_member = find_user_in_members(&users[0], &members);
    let user_2_member = find_user_in_members(&users[1], &members);

    let user_1_display_name = get_author_name(&users[0], &user_1_member.cloned());
    let user_2_display_name = get_author_name(&users[1], &user_2_member.cloned());

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
            tracing::warn!("{}. Retrying with PNG...", e.to_string());

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
                    tracing::error!("{}", e.to_string());
                    command
                        .edit_response(
                            &ctx.http,
                            EditInteractionResponse::new()
                                .content(format!("Sorry, an occurred! Error: {}", e)),
                        )
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
    command: &CommandInteraction,
    image: Vec<u8>,
    ship_score_text: String,
    user_1_display_name: String,
    user_2_display_name: String,
    ship_msg: String,
    color: Color,
) -> anyhow::Result<()> {
    command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(&ship_score_text),
        )
        .await?;
    let files = [CreateAttachment::bytes(image, "result.png")];
    command
        .channel_id
        .send_files(
            &ctx.http,
            files,
            CreateMessage::new().embed(
                CreateEmbed::new()
                    .title(format!(
                        "{} and {}",
                        user_1_display_name, user_2_display_name
                    ))
                    .description(format!("{}\n\n{}", ship_score_text, ship_msg))
                    .attachment("result.png")
                    .color(color),
            ),
        )
        .await?;
    Ok(())
}
