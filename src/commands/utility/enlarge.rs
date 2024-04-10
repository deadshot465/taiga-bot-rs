use poise::CreateReply;
use serenity::all::CreateInteractionResponseFollowup;

use crate::shared::constants::{
    EMOTE_BASE_LINK, EMOTE_ID_REGEX, EMOTE_IS_ANIMATED_REGEX, EMOTE_REGEX,
};
use crate::shared::structs::{Context, ContextError};

/// Returns enlarged emote(s).
#[poise::command(slash_command, category = "Utility")]
pub async fn enlarge(
    ctx: Context<'_>,
    #[description = "One or more emotes to enlarge."] emote: String,
) -> Result<(), ContextError> {
    if !EMOTE_ID_REGEX.is_match(&emote) {
        ctx.send(CreateReply::default().content("There are no emotes in the input!"))
            .await?;
        return Ok(());
    }

    ctx.send(CreateReply::default().content("Alright then, these are the emotes you requested."))
        .await?;

    let split_emotes = emote.split(' ').collect::<Vec<_>>();
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

    if let Context::Application(app_context) = ctx {
        for emote_link in emote_links.into_iter() {
            app_context
                .interaction
                .create_followup(
                    &ctx.http(),
                    CreateInteractionResponseFollowup::new().content(emote_link),
                )
                .await?;
        }
    }

    Ok(())
}
