use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::services::image_service::{get_cat_image, get_dog_image, get_normal_image};
use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{get_author_avatar, get_author_name};
use poise::CreateReply;
use rand::prelude::*;
use std::borrow::Cow;

/// Get random images based on keywords.
#[poise::command(
    slash_command,
    subcommands("inner_image", "cat", "dog"),
    subcommand_required,
    category = "Utility"
)]
pub async fn image(_: Context<'_>) -> Result<(), ContextError> {
    Ok(())
}

/// Get random images based on keywords.
#[poise::command(slash_command, rename = "image")]
pub async fn inner_image(
    ctx: Context<'_>,
    #[description = "Keyword to search for."] keyword: Option<String>,
) -> Result<(), ContextError> {
    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_avatar_url = get_author_avatar(author);
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };

    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! Hold on..."))
        .await?;

    let keyword = keyword.unwrap_or("burger".into());
    let result = get_normal_image(
        ctx,
        &keyword,
        &ctx.data().http_client,
        &author_name,
        &author_avatar_url,
        color,
    )
    .await;

    match result {
        Ok(embed) => {
            reply_handle
                .edit(ctx, CreateReply::default().embed(embed))
                .await?;
        }
        Err(e) => {
            tracing::error!("Failed to retrieve image: {}", e.to_string());
            reply_handle
                .edit(ctx, CreateReply::default().content(if is_kou {
                    "Sorry...I don't understand the keyword and cannot find anything... <:KouCry:705054435826597928>"
                } else {
                    "Sorry. Not my problem. Your keyword is too weird that I can't find any image."
                })).await?;
        }
    }

    Ok(())
}

/// Get cat images.
#[poise::command(slash_command)]
pub async fn cat(
    ctx: Context<'_>,
    #[description = "Keyword to search for."] keyword: Option<String>,
) -> Result<(), ContextError> {
    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_avatar_url = get_author_avatar(author);
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };

    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! Hold on..."))
        .await?;

    let keyword = keyword.unwrap_or_default();

    let result = if thread_rng().gen_range(0..2) > 0 {
        // Invoke the Cat API
        get_cat_image(
            ctx,
            &keyword,
            &ctx.data().http_client,
            &author_name,
            &author_avatar_url,
            color,
        )
        .await
    } else {
        // Invoke Unsplash API
        let keyword = if keyword.is_empty() {
            "cat".into()
        } else {
            "cat ".to_string() + &keyword
        };
        get_normal_image(
            ctx,
            &keyword,
            &ctx.data().http_client,
            &author_name,
            &author_avatar_url,
            color,
        )
        .await
    };

    match result {
        Ok(embed) => {
            reply_handle
                .edit(ctx, CreateReply::default().embed(embed))
                .await?;
        }
        Err(e) => {
            tracing::error!("Failed to retrieve image: {}", e.to_string());
            reply_handle
                .edit(ctx, CreateReply::default().content(if is_kou {
                    "Sorry...I don't understand the keyword and cannot find anything... <:KouCry:705054435826597928>"
                } else {
                    "Sorry. Not my problem. Your keyword is too weird that I can't find any image."
                })).await?;
        }
    }

    Ok(())
}

/// Get dog images.
#[poise::command(slash_command)]
pub async fn dog(
    ctx: Context<'_>,
    #[description = "Keyword to search for."] keyword: Option<String>,
) -> Result<(), ContextError> {
    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_avatar_url = get_author_avatar(author);
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };

    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! Hold on..."))
        .await?;

    let keyword = keyword.unwrap_or_default();

    let result = if thread_rng().gen_range(0..2) > 0 {
        // Invoke the Dog API
        get_dog_image(
            &keyword,
            &ctx.data().http_client,
            &author_name,
            &author_avatar_url,
            color,
        )
        .await
    } else {
        // Invoke Unsplash API
        let keyword = if keyword.is_empty() {
            "dog".into()
        } else {
            "dog ".to_string() + &keyword
        };
        get_normal_image(
            ctx,
            &keyword,
            &ctx.data().http_client,
            &author_name,
            &author_avatar_url,
            color,
        )
        .await
    };

    match result {
        Ok(embed) => {
            reply_handle
                .edit(ctx, CreateReply::default().embed(embed))
                .await?;
        }
        Err(e) => {
            tracing::error!("Failed to retrieve image: {}", e.to_string());
            reply_handle
                .edit(ctx, CreateReply::default().content(if is_kou {
                    "Sorry...I don't understand the keyword and cannot find anything... <:KouCry:705054435826597928>"
                } else {
                    "Sorry. Not my problem. Your keyword is too weird that I can't find any image."
                })).await?;
        }
    }

    Ok(())
}
