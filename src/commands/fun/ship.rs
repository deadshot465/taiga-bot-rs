#![allow(clippy::too_many_arguments)]

use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::services::ship_service::{
    calculate_ship_score, download_avatar, generate_ship_image, get_ship_message,
    monochrome_if_lower_score,
};
use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{find_user_in_members, get_author_avatar, get_author_name};
use poise::{CreateReply, ReplyHandle};
use serenity::all::{Color, CreateAttachment, CreateMessage, User};
use serenity::builder::CreateEmbed;
use serenity::model::id::UserId;

/// Ship two users.
#[poise::command(slash_command, category = "Fun")]
pub async fn ship(
    ctx: Context<'_>,
    #[description = "The first user to ship with the second user."] user_1: User,
    #[description = "The second user to ship with the first user."] user_2: User,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! Hold on..."))
        .await?;

    let users = vec![user_1, user_2];
    let ship_score = calculate_ship_score(users[0].id.get(), users[1].id.get());

    let mut user_1_avatar_url = get_author_avatar(&users[0]);
    let mut user_2_avatar_url = get_author_avatar(&users[1]);

    user_1_avatar_url = monochrome_if_lower_score(ship_score, user_1_avatar_url);
    user_2_avatar_url = monochrome_if_lower_score(ship_score, user_2_avatar_url);

    let user_1_avatar = download_avatar(ctx, &user_1_avatar_url).await?;
    let user_2_avatar = download_avatar(ctx, &user_2_avatar_url).await?;

    let members = ctx
        .guild_id()
        .unwrap_or_default()
        .members(ctx.http(), None::<u64>, None::<UserId>)
        .await?;
    let user_1_member = find_user_in_members(&users[0], &members);
    let user_2_member = find_user_in_members(&users[1], &members);

    let user_1_display_name = get_author_name(&users[0], &user_1_member.cloned());
    let user_2_display_name = get_author_name(&users[1], &user_2_member.cloned());

    let ship_msg = get_ship_message(ctx, ship_score)
        .replace("$1", &user_1_display_name)
        .replace("$2", &user_2_display_name);

    let result_handle =
        tokio::spawn(async move { generate_ship_image(&user_1_avatar, &user_2_avatar) });

    let ship_score_text = format!("Your love score is {}!", ship_score);
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    match result_handle.await? {
        Ok(result) => {
            send_ship_embed(
                ctx,
                reply_handle,
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

            match retry_with_png_if_error(ctx, &user_1_avatar_url, &user_2_avatar_url).await {
                Ok(result) => {
                    send_ship_embed(
                        ctx,
                        reply_handle,
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
                    reply_handle
                        .edit(
                            ctx,
                            CreateReply::default()
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
    ctx: Context<'_>,
    avatar_url_1: &str,
    avatar_url_2: &str,
) -> anyhow::Result<Vec<u8>> {
    let avatar_url_1 = avatar_url_1.replace(".webp", ".png");
    let avatar_url_2 = avatar_url_2.replace(".webp", ".png");
    let user_1_avatar = download_avatar(ctx, &avatar_url_1).await?;
    let user_2_avatar = download_avatar(ctx, &avatar_url_2).await?;
    generate_ship_image(&user_1_avatar, &user_2_avatar)
}

async fn send_ship_embed(
    ctx: Context<'_>,
    reply_handle: ReplyHandle<'_>,
    image: Vec<u8>,
    ship_score_text: String,
    user_1_display_name: String,
    user_2_display_name: String,
    ship_msg: String,
    color: Color,
) -> anyhow::Result<()> {
    reply_handle
        .edit(ctx, CreateReply::default().content(&ship_score_text))
        .await?;
    let files = [CreateAttachment::bytes(image, "result.png")];
    ctx.channel_id()
        .send_files(
            ctx.http(),
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
