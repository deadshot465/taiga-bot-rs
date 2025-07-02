use std::borrow::Cow;

use poise::CreateReply;
use rand::prelude::*;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};

use crate::shared::structs::information::character::Character;
use crate::shared::structs::record::user_record::write_user_records;
use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{
    get_animated_emote_url, get_author_avatar, get_author_name, get_first_name,
};

const MATURE_HIRO_EMOTE_IDS: [&str; 5] = [
    "703591584305774662",
    "710951618576908289",
    "711192310767157248",
    "710957588237385789",
    "711227408933453844",
];

const ENDINGS: [&str; 4] = ["Perfect", "Good", "Bad", "Worst"];

const KOU_GIFS: [&str; 5] = [
    "https://tetsukizone.com/images/kou.gif",
    "https://tetsukizone.com/images/kou2.gif",
    "https://tetsukizone.com/images/kou3.gif",
    "https://tetsukizone.com/images/kou4.gif",
    "https://cdn.discordapp.com/emojis/705279783340212265.gif",
];

/// Tells you what route to play next.
#[poise::command(slash_command, category = "Information")]
pub async fn route(ctx: Context<'_>) -> Result<(), ContextError> {
    let route = get_route(ctx.data().routes.as_slice());

    let footer = format!(
        "Play {}'s route next. All bois are best bois.",
        get_first_name(&route.name)
    );
    let color =
        u32::from_str_radix(&route.color, 16).expect("Failed to create a color from string.");
    let ending = if route.name.contains("Mature") || route.name.contains("Kou") {
        "Perfect"
    } else {
        let mut rng = rand::rng();
        ENDINGS
            .choose(&mut rng)
            .expect("Failed to choose an ending.")
    };

    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author = ctx.author();
    let author_name = get_author_name(author, &member);
    let author_icon = get_author_avatar(author);
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .author(CreateEmbedAuthor::new(&author_name).icon_url(&author_icon))
                .color(color)
                .field("Age", route.age.to_string(), true)
                .field("Birthday", &route.birthday, true)
                .field("Animal Motif", &route.animal, true)
                .footer(CreateEmbedFooter::new(footer))
                .description(&route.description)
                .thumbnail(if route.name.contains("Mature") {
                    let mut rng = rand::rng();
                    let emote_id = MATURE_HIRO_EMOTE_IDS
                        .choose(&mut rng)
                        .cloned()
                        .expect("Failed to get an emote ID for mature Hiro.");
                    get_animated_emote_url(emote_id)
                } else if route.name.contains("Kou") {
                    let mut rng = rand::rng();
                    KOU_GIFS
                        .choose(&mut rng)
                        .expect("Failed to get an emote ID for Kou")
                        .to_string()
                } else {
                    get_animated_emote_url(&route.emote_id)
                })
                .title(format!("Next: {}, {} Ending", &route.name, ending)),
        ),
    )
    .await?;

    let user_records = ctx.data().user_records.clone();
    {
        let mut user_records_lock = user_records.write().await;
        let user_record_entry = user_records_lock
            .entry(author.id.get().to_string())
            .or_default();
        let route_entry = user_record_entry
            .route
            .entry(route.name.clone())
            .or_default();
        *route_entry.entry(format!("{ending} Ending")).or_default() += 1;
    }

    let user_records_lock = user_records.read().await;
    write_user_records(&user_records_lock)?;

    Ok(())
}

fn get_route(routes: &[Character]) -> Character {
    let mut rng = rand::rng();
    let random_numbers = rng.random_range(0..100);
    match random_numbers {
        x if (0..=14).contains(&x) => routes[0].clone(),
        x if (15..=19).contains(&x) => routes[1].clone(),
        x if (20..=22).contains(&x) => routes[7].clone(),
        x if (23..=38).contains(&x) => routes[2].clone(),
        x if (39..=53).contains(&x) => routes[3].clone(),
        x if (54..=68).contains(&x) => routes[4].clone(),
        x if (69..=83).contains(&x) => routes[5].clone(),
        x if (84..=99).contains(&x) => routes[6].clone(),
        _ => routes
            .choose(&mut rng)
            .cloned()
            .expect("Failed to choose a route."),
    }
}
