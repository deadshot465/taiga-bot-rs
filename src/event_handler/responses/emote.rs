use crate::shared::structs::config::configuration::CONFIGURATION;
use crate::shared::structs::fun::emote::EMOTE_LIST;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_emote(ctx: &Context, new_message: &Message) -> anyhow::Result<()> {
    let message_content = &new_message.content;
    let prefix = CONFIGURATION
        .get()
        .map(|c| c.prefix.as_str())
        .unwrap_or("ta!");

    if !message_content.starts_with(prefix) {
        return Ok(());
    }

    let arguments = (&message_content[prefix.chars().count()..])
        .split(' ')
        .collect::<Vec<_>>();
    let emote_name = arguments.get(0).copied().unwrap_or_default();
    let count = arguments
        .get(1)
        .copied()
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or_default();

    let emote = {
        EMOTE_LIST
            .read()
            .await
            .emotes
            .iter()
            .find(|emote| emote.name.as_str() == emote_name)
            .cloned()
    };

    if let Some(emote) = emote {
        if count > 0 {
            let emotes: String = (0_u8..count)
                .collect::<Vec<_>>()
                .into_iter()
                .map(|_| emote.raw.as_str())
                .collect::<Vec<_>>()
                .join(" ");

            new_message.reply(&ctx.http, emotes).await?;
        } else {
            new_message.reply(&ctx.http, &emote.link).await?;
        }
    }

    Ok(())
}
