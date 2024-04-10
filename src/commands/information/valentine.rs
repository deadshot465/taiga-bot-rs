use std::borrow::Cow;

use poise::CreateReply;
use rand::prelude::*;
use serenity::all::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

use crate::shared::structs::record::user_record::write_user_records;
use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{
    get_author_avatar, get_author_name, get_first_name, get_static_emote_url,
};

/// Tells you your next valentine.
#[poise::command(slash_command, category = "Information")]
pub async fn valentine(ctx: Context<'_>) -> Result<(), ContextError> {
    let valentine = {
        let mut rng = thread_rng();
        ctx.data()
            .valentines
            .choose(&mut rng)
            .cloned()
            .expect("Failed to get a valentine.")
    };

    let is_keitaro = get_first_name(&valentine.name) == "Keitaro";
    let is_kou = ctx.data().kou;
    let rig_keitaro = !is_kou && is_keitaro;
    let prefix_suffix = if rig_keitaro { "~~" } else { "" };

    let footer = if rig_keitaro {
        "See? Told you Keitaro is my boyfriend. Later loser."
    } else {
        "Don't fret if {firstName} isn't your type. Who knows, maybe it's time for a new favorite."
    }
    .replace("{firstName}", get_first_name(&valentine.name));

    let valentine_name = format!(
        "{}Your valentine is {}{}",
        prefix_suffix,
        valentine.name.as_str(),
        prefix_suffix
    );

    let color =
        u32::from_str_radix(&valentine.color, 16).expect("Failed to create a color from string.");

    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_icon = get_author_avatar(author);
    let message = CreateReply::default().embed(
        CreateEmbed::new()
            .author(CreateEmbedAuthor::new(&author_name).icon_url(&author_icon))
            .color(color)
            .field("Age", valentine.age.to_string(), true)
            .field("Birthday", &valentine.birthday, true)
            .field("Animal Motif", &valentine.animal, true)
            .footer(CreateEmbedFooter::new(footer))
            .description(format!(
                "{}{}{}",
                prefix_suffix, &valentine.description, prefix_suffix
            ))
            .thumbnail(get_static_emote_url(&valentine.emote_id))
            .title(&valentine_name),
    );

    let message = if is_keitaro {
        message.content(if is_kou {
            "I heard someone is super jealous about this guy, but you bet I will protect Nene senpai!"
        } else {
            "**Bah, we're already dating and I'm the best. No chance for you, loser.**"
        })
    } else {
        message
    };

    ctx.send(message).await?;

    let user_records = ctx.data().user_records.clone();
    {
        let mut user_records_lock = user_records.write().await;
        let entry = user_records_lock
            .entry(author.id.get().to_string())
            .or_default();
        *entry.valentine.entry(valentine.name.clone()).or_default() += 1;
    }

    let user_records_lock = user_records.read().await;
    write_user_records(&user_records_lock)?;

    Ok(())
}
