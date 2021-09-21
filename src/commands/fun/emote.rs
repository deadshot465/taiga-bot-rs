use crate::shared::constants::{
    EMOTE_BASE_LINK, EMOTE_ID_REGEX, EMOTE_IS_ANIMATED_REGEX, EMOTE_REGEX, KOU_COLOR,
    SHIBA_KEK_ICON, TAIGA_COLOR,
};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::structs::fun::emote::{Emote, EMOTE_LIST};
use crate::shared::utility::{get_author_avatar, get_author_name};
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use serenity::utils::Color;
use std::future::Future;
use std::pin::Pin;

static NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\w").expect("Failed to initialize regular expression."));

pub fn emote_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(emote(ctx, command))
}

async fn emote(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };

    // Dispatch to different subcommands
    if let Some(opt) = command.data.options.get(0) {
        match opt.name.as_str() {
            "list" => list(ctx, command, color).await?,
            "add" => add(ctx, command, is_kou).await?,
            "remove" => remove(ctx, command).await?,
            _ => list(ctx, command, color).await?,
        }
    }

    Ok(())
}

async fn add(
    ctx: Context,
    command: ApplicationCommandInteraction,
    is_kou: bool,
) -> anyhow::Result<()> {
    let emote_name = extract_argument(&command, 0).to_lowercase();

    if !NAME_REGEX.is_match(&emote_name) {
        command.create_interaction_response(&ctx.http, |response| response
            .interaction_response_data(|data| data.content(if is_kou {
                "I'm not really good at languages...Could you pick another name, please?... <:KouConcern:736062067299188817>"
            } else {
                "Well I *can* do it if you really want such a weird name, but no, I don't *want* to do."
            }))).await?;
        return Ok(());
    }

    let emote_exists = {
        EMOTE_LIST
            .read()
            .await
            .emotes
            .iter()
            .any(|emote| emote.name.as_str() == emote_name.as_str())
    };

    if emote_exists {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("The emote you specified already existed!")
                })
            })
            .await?;
        return Ok(());
    }

    let emote_string = extract_argument(&command, 1);

    if !EMOTE_REGEX.is_match(&emote_string) {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content(if is_kou {
                        "It's not a valid emote, I think...?"
                    } else {
                        "Obviously this is not a correct or valid emote, you dummy..."
                    })
                })
            })
            .await?;
        return Ok(());
    }

    let emote_id = EMOTE_ID_REGEX
        .captures(&emote_string)
        .and_then(|captures| captures.get(2))
        .map(|match_item| match_item.as_str())
        .and_then(|string| string.parse::<u64>().ok())
        .unwrap_or_default();

    let file_extension = if EMOTE_IS_ANIMATED_REGEX.is_match(&emote_string) {
        ".gif"
    } else {
        ".png"
    };

    let link = format!("{}{}{}", EMOTE_BASE_LINK, emote_id, file_extension);

    {
        let mut emote_list_write_lock = EMOTE_LIST.write().await;
        emote_list_write_lock.emotes.push(Emote {
            name: emote_name,
            id: emote_id,
            link,
            raw: emote_string,
        });
        emote_list_write_lock.write_emote_list()?;
    }

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Successfully added the emote!"))
        })
        .await?;

    Ok(())
}

async fn list(
    ctx: Context,
    command: ApplicationCommandInteraction,
    color: Color,
) -> anyhow::Result<()> {
    let emote_names: String = {
        EMOTE_LIST
            .read()
            .await
            .emotes
            .iter()
            .map(|e| format!("`{}`", &e.name))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let author_name = get_author_name(&command.user, &command.member);
    let author_avatar_url = get_author_avatar(&command.user);

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.create_embed(|embed| {
                    embed
                        .description(format!(
                            "The following is a list of currently registered emotes:\n\n{}",
                            if emote_names.chars().count() > 1990 {
                                &emote_names[..1990]
                            } else {
                                &emote_names
                            }
                        ))
                        .author(|a| a.name(&author_name).icon_url(&author_avatar_url))
                        .title("Registered Emotes")
                        .color(color)
                        .thumbnail(SHIBA_KEK_ICON)
                })
            })
        })
        .await?;

    Ok(())
}

async fn remove(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let emote_name = extract_argument(&command, 0).to_lowercase();
    let emote = {
        let emote_list_read_lock = EMOTE_LIST.read().await;
        emote_list_read_lock
            .emotes
            .iter()
            .find(|item| item.name.as_str() == emote_name.as_str())
            .cloned()
    };

    if let Some(emote) = emote {
        {
            let mut emote_list_write_lock = EMOTE_LIST.write().await;
            let filtered_emote_list = emote_list_write_lock
                .emotes
                .clone()
                .into_iter()
                .filter(|registered_emote| registered_emote.name.as_str() != emote.name.as_str())
                .collect::<Vec<_>>();
            emote_list_write_lock.emotes = filtered_emote_list;
            emote_list_write_lock.write_emote_list()?;
        }

        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("Successfully removed the emote!")
                })
            })
            .await?;
    } else {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("The emote you specified is not registered!")
                })
            })
            .await?;
    }

    Ok(())
}

fn extract_argument(command: &ApplicationCommandInteraction, index: usize) -> String {
    command
        .data
        .options
        .get(0)
        .and_then(|opt| opt.options.get(index))
        .and_then(|opt| opt.value.as_ref())
        .and_then(|value| value.as_str())
        .map(ToString::to_string)
        .unwrap_or_default()
}
