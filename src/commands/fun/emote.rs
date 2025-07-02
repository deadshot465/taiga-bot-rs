use std::borrow::Cow;

use once_cell::sync::Lazy;
use poise::CreateReply;
use regex::Regex;
use serenity::all::{CreateEmbed, CreateEmbedAuthor};

use crate::shared::constants::{
    EMOTE_BASE_LINK, EMOTE_ID_REGEX, EMOTE_IS_ANIMATED_REGEX, EMOTE_REGEX, KOU_COLOR,
    SHIBA_KEK_ICON, TAIGA_COLOR,
};
use crate::shared::structs::fun::emote::Emote;
use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{get_author_avatar, get_author_name};

static NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\w").expect("Failed to initialize regular expression."));

/// Add or remove an emote from the bot. Emotes from servers which the bot is not in won't work.
#[poise::command(
    slash_command,
    subcommands("list", "add", "remove"),
    subcommand_required,
    category = "Fun"
)]
pub async fn emote(_: Context<'_>) -> Result<(), ContextError> {
    Ok(())
}

/// Add an emote to the emote list to be used with Kou/Taiga.
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "The name of the emote to be used with prefix."] name: String,
    #[description = "The emote to register."] emote: String,
) -> Result<(), ContextError> {
    let emote_name = name.to_lowercase();
    let is_kou = ctx.data().kou;

    if !NAME_REGEX.is_match(&emote_name) {
        ctx
            .send(CreateReply::default().content(if is_kou {
                "I'm not really good at languages...Could you pick another name, please?... <:KouConcern:736062067299188817>"
            } else {
                "Well I *can* do it if you really want such a weird name, but no, I don't *want* to do."
            }))
            .await?;
        return Ok(());
    }

    let emote_list = ctx.data().emote_list.clone();

    let emote_exists = {
        emote_list
            .read()
            .await
            .emotes
            .iter()
            .any(|emote| emote.name.as_str() == emote_name.as_str())
    };

    if emote_exists {
        ctx.send(CreateReply::default().content("The emote you specified already existed!"))
            .await?;
        return Ok(());
    }

    if !EMOTE_REGEX.is_match(&emote) {
        ctx.send(CreateReply::default().content(if is_kou {
            "It's not a valid emote, I think...?"
        } else {
            "Obviously this is not a correct or valid emote, you dummy..."
        }))
        .await?;
        return Ok(());
    }

    let emote_id = EMOTE_ID_REGEX
        .captures(&emote)
        .and_then(|captures| captures.get(2))
        .map(|match_item| match_item.as_str())
        .and_then(|string| string.parse::<u64>().ok())
        .unwrap_or_default();

    let file_extension = if EMOTE_IS_ANIMATED_REGEX.is_match(&emote) {
        ".gif"
    } else {
        ".png"
    };

    let link = format!("{EMOTE_BASE_LINK}{emote_id}{file_extension}");

    {
        let mut emote_list_write_lock = emote_list.write().await;
        emote_list_write_lock.emotes.push(Emote {
            name: emote_name,
            id: emote_id,
            link,
            raw: emote,
        });
        emote_list_write_lock.write_emote_list()?;
    }

    ctx.send(CreateReply::default().content("Successfully added the emote!"))
        .await?;

    Ok(())
}

/// List registered emotes in this server.
#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), ContextError> {
    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let emote_list = ctx.data().emote_list.clone();
    let emote_names: String = {
        emote_list
            .read()
            .await
            .emotes
            .iter()
            .map(|e| format!("`{}`", &e.name))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_avatar_url = get_author_avatar(author);

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .description(format!(
                    "The following is a list of currently registered emotes:\n\n{}",
                    if emote_names.chars().count() > 1990 {
                        &emote_names[..1990]
                    } else {
                        &emote_names
                    }
                ))
                .author(CreateEmbedAuthor::new(&author_name).icon_url(&author_avatar_url))
                .title("Registered Emotes")
                .color(color)
                .thumbnail(SHIBA_KEK_ICON),
        ),
    )
    .await?;

    Ok(())
}

/// Remove an emote from the emote list.
#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The name of the emote to be removed."] name: String,
) -> Result<(), ContextError> {
    let emote_name = name.to_lowercase();
    let emote_list = ctx.data().emote_list.clone();
    let emote = {
        let emote_list_read_lock = emote_list.read().await;
        emote_list_read_lock
            .emotes
            .iter()
            .find(|item| item.name.as_str() == emote_name.as_str())
            .cloned()
    };

    if let Some(emote) = emote {
        {
            let mut emote_list_write_lock = emote_list.write().await;
            let filtered_emote_list = emote_list_write_lock
                .emotes
                .clone()
                .into_iter()
                .filter(|registered_emote| registered_emote.name.as_str() != emote.name.as_str())
                .collect::<Vec<_>>();
            emote_list_write_lock.emotes = filtered_emote_list;
            emote_list_write_lock.write_emote_list()?;
        }

        ctx.send(CreateReply::default().content("Successfully removed the emote!"))
            .await?;
    } else {
        ctx.send(CreateReply::default().content("The emote you specified is not registered!"))
            .await?;
    }

    Ok(())
}
