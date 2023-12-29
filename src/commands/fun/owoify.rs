use crate::shared::structs::config::configuration::KOU;
use crate::shared::utility::get_author_name;
use owoify_rs::{Owoifiable, OwoifyLevel};
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::CommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

const OWOIFY_LENGTH_LIMIT: usize = 1024;

pub fn owoify_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(owoify(ctx, command))
}

async fn owoify(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let level = match command
        .data
        .options
        .get(0)
        .map(|opt| &opt.value)
        .and_then(|value| value.as_str())
        .unwrap_or_default()
    {
        "soft" => OwoifyLevel::Owo,
        "medium" => OwoifyLevel::Uwu,
        "hard" => OwoifyLevel::Uvu,
        _ => OwoifyLevel::Owo,
    };

    let text = command
        .data
        .options
        .get(1)
        .map(|opt| &opt.value)
        .and_then(|value| value.as_str())
        .map(|s| s.trim())
        .unwrap_or_default();

    let is_kou = KOU.get().copied().unwrap_or(false);

    if text.is_empty() {
        cancel_owoify(
            &ctx,
            &command,
            if is_kou {
                "...I don't know what to owo, sorry..."
            } else {
                "...There's nothing to owoify, you dummy."
            },
        )
        .await?;
    }

    let mut length_exceeded = false;
    let trimmed_text = if text.len() > OWOIFY_LENGTH_LIMIT {
        length_exceeded = true;
        &text[..OWOIFY_LENGTH_LIMIT]
    } else {
        text
    };

    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .content(if length_exceeded {
                format!("{}\n\n{}", if is_kou {
                    "<:KouCry:705054435826597928> I'm not really smart so I can't owoify such a long sentence..."
                } else {
                    "<:TaigaUneasy2:700006812673638500> Even idiocy has its limit. Same goes for owoification as well. I won't do any text that is more than 1000 characters."
                }, trimmed_text.owoify(level))
            } else {
                let member = command.member.clone().map(|m| *m);
                let author_name = get_author_name(&command.user, &member);
                format!("OwO-ified for {}~!\n\n{}", author_name, trimmed_text.owoify(level))
            }))).await?;

    Ok(())
}

async fn cancel_owoify(
    ctx: &Context,
    command: &CommandInteraction,
    msg: &str,
) -> anyhow::Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(msg),
            ),
        )
        .await?;
    Ok(())
}
