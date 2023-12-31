use crate::event_handler::hit_or_miss;
use crate::shared::constants::EMOTE_IS_ANIMATED_REGEX;
use crate::shared::structs::config::configuration::CONFIGURATION;
use crate::shared::structs::config::random_response::{
    get_random_reaction, RANDOM_RESPONSES_KEYWORDS,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;
use serenity::model::prelude::Message;
use serenity::prelude::*;

static EMOTE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<a?:(\w+):(\d+)>").expect("Failed to initialize regular expression.")
});

pub async fn handle_reactions(ctx: &Context, new_message: &Message) -> anyhow::Result<()> {
    if new_message.author.bot {
        return Ok(());
    }

    let random_reply_chance = CONFIGURATION
        .get()
        .map(|c| c.random_reply_chance)
        .unwrap_or_default();

    if !hit_or_miss(random_reply_chance) {
        return Ok(());
    }

    let message_content = new_message.content.to_lowercase();
    for keyword in RANDOM_RESPONSES_KEYWORDS.iter() {
        if !message_content.contains(keyword) {
            continue;
        }

        let reaction = get_random_reaction(keyword.trim());
        let emote = if EMOTE_REGEX.is_match(&reaction) {
            build_emote(&reaction)
        } else {
            ReactionType::Unicode(reaction)
        };

        new_message.react(&ctx.http, emote).await?;
    }

    Ok(())
}

fn build_emote(reaction: &str) -> ReactionType {
    let animated = EMOTE_IS_ANIMATED_REGEX.is_match(reaction);
    let captures = EMOTE_REGEX
        .captures(reaction)
        .expect("Failed to get captures for emote regex.");

    let (emote_name, emote_id) = (
        captures
            .get(1)
            .expect("Failed to get first capture of the emote regex.")
            .as_str(),
        captures
            .get(2)
            .expect("Failed to get second capture of the emote regex.")
            .as_str()
            .parse::<u64>()
            .expect("Failed to parse emote ID to u64."),
    );

    ReactionType::Custom {
        animated,
        id: EmojiId::new(emote_id),
        name: Some(emote_name.into()),
    }
}
