use crate::shared::constants::{
    EMOTE_BASE_LINK, EMOTE_ID_REGEX, EMOTE_IS_ANIMATED_REGEX, EMOTE_REGEX,
};
use serenity::all::{
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
};
use serenity::model::application::CommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

pub fn enlarge_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(enlarge(ctx, command))
}

async fn enlarge(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let raw_string = command
        .data
        .options
        .get(0)
        .map(|opt| &opt.value)
        .and_then(|value| value.as_str())
        .unwrap_or_default();

    if !EMOTE_ID_REGEX.is_match(raw_string) {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("There are no emotes in the input!"),
                ),
            )
            .await?;
        return Ok(());
    }

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Alright then, these are the emotes you requested."),
            ),
        )
        .await?;

    let split_emotes = raw_string.split(' ').collect::<Vec<_>>();
    let mut emote_links = vec![];

    for &emote in split_emotes.iter() {
        if !EMOTE_REGEX.is_match(emote) {
            continue;
        }

        for capture in EMOTE_REGEX.captures_iter(emote) {
            let emote_fullname = capture
                .get(1)
                .expect("Failed to get the full name of the emote.");

            let id_capture = EMOTE_ID_REGEX
                .captures(emote_fullname.as_str())
                .expect("Failed to get emote ID from captured emote.");

            let id = id_capture
                .get(2)
                .expect("Failed to extract ID from capture.");

            emote_links.push(format!(
                "{}{}{}",
                EMOTE_BASE_LINK,
                id.as_str(),
                if EMOTE_IS_ANIMATED_REGEX.is_match(emote_fullname.as_str()) {
                    ".gif"
                } else {
                    ".png"
                }
            ));
        }
    }

    for emote_link in emote_links.into_iter() {
        command
            .create_followup(
                &ctx.http,
                CreateInteractionResponseFollowup::new().content(emote_link),
            )
            .await?;
    }

    Ok(())
}
