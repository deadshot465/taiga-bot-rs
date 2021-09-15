use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

const EMOTE_BASE_LINK: &str = "https://cdn.discordapp.com/emojis/";

static EMOTE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(<a?:\w+:\d+>)").expect("Failed to initialize regular expression."));

static EMOTE_ID_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(:\w+:)(\d+)").expect("Failed to initialize regular expression."));

static EMOTE_IS_ANIMATED_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(<a)").expect("Failed to initialize regular expression."));

pub fn enlarge_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(enlarge(ctx, command))
}

async fn enlarge(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let raw_string = if let Some(opt) = command.data.options.get(0) {
        opt.value
            .as_ref()
            .and_then(|v| v.as_str())
            .unwrap_or_default()
    } else {
        ""
    };

    if !EMOTE_ID_REGEX.is_match(raw_string) {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("There are no emotes in the input!")
                })
            })
            .await?;
        return Ok(());
    }

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.content("Alright then, these are the emotes you requested.")
            })
        })
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
            .create_followup_message(&ctx.http, |response| response.content(emote_link))
            .await?;
    }

    Ok(())
}
