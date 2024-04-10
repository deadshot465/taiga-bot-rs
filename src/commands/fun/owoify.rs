use std::borrow::Cow;

use owoify_rs::{Owoifiable, OwoifyLevel};
use poise::CreateReply;

use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::get_author_name;

const OWOIFY_LENGTH_LIMIT: usize = 1024;

#[derive(Debug, Copy, Clone, poise::ChoiceParameter)]
pub enum Owoness {
    Soft,
    Medium,
    Hard,
}

/// This command will owoify your text.
#[poise::command(slash_command, category = "Fun")]
pub async fn owoify(
    ctx: Context<'_>,
    #[description = "The owoness you want to owoify your text."] level: Option<Owoness>,
    #[description = "The text to owoify."] text: String,
) -> Result<(), ContextError> {
    let level = match level.unwrap_or(Owoness::Soft) {
        Owoness::Soft => OwoifyLevel::Owo,
        Owoness::Medium => OwoifyLevel::Uwu,
        Owoness::Hard => OwoifyLevel::Uvu,
    };

    let is_kou = ctx.data().kou;

    if text.is_empty() {
        cancel_owoify(
            ctx,
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
        text.as_str()
    };

    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();

    ctx
        .send(CreateReply::default().content(if length_exceeded {
            format!("{}\n\n{}", if is_kou {
                "<:KouCry:705054435826597928> I'm not really smart so I can't owoify such a long sentence..."
            } else {
                "<:TaigaUneasy2:700006812673638500> Even idiocy has its limit. Same goes for owoification as well. I won't do any text that is more than 1000 characters."
            }, trimmed_text.owoify(level))
        } else {
            let author_name = get_author_name(author, &member);
            format!("OwO-ified for {}~!\n\n{}", author_name, trimmed_text.owoify(level))
        }))
        .await?;

    Ok(())
}

async fn cancel_owoify(ctx: Context<'_>, msg: &str) -> anyhow::Result<()> {
    ctx.send(CreateReply::default().content(msg)).await?;
    Ok(())
}
