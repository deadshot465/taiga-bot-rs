use crate::shared::structs::config::configuration::KOU;
use crate::shared::structs::information::character::VALENTINES;
use crate::shared::structs::record::user_record::{write_user_records, USER_RECORDS};
use crate::shared::utility::{
    get_author_avatar, get_author_name, get_first_name, get_static_emote_url,
};
use rand::prelude::*;
use serenity::all::{
    CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::model::application::CommandInteraction;
use serenity::prelude::Context;
use std::future::Future;
use std::pin::Pin;

pub fn valentine_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(valentine(ctx, command))
}

async fn valentine(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let valentine = {
        let mut rng = rand::thread_rng();
        VALENTINES
            .choose(&mut rng)
            .cloned()
            .expect("Failed to get a valentine.")
    };

    let is_keitaro = get_first_name(&valentine.name) == "Keitaro";
    let is_kou = KOU.get().copied().unwrap_or(false);
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

    let member = command.member.clone().map(|m| *m);
    let author_name = get_author_name(&command.user, &member);
    let author_icon = get_author_avatar(&command.user);
    let message = CreateInteractionResponseMessage::new().embed(
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

    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await?;

    if let Some(user_records) = USER_RECORDS.get() {
        {
            let mut user_records_lock = user_records.write().await;
            let entry = user_records_lock
                .entry(command.user.id.get().to_string())
                .or_default();
            *entry.valentine.entry(valentine.name.clone()).or_default() += 1;
        }
        let user_records_lock = user_records.read().await;
        write_user_records(&user_records_lock)?;
    }

    Ok(())
}
