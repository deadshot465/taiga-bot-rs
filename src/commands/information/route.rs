use crate::shared::structs::information::character::{Character, ROUTES};
use crate::shared::structs::record::user_record::{write_user_records, USER_RECORDS};
use crate::shared::utility::{
    get_animated_emote_url, get_author_avatar, get_author_name, get_first_name,
};
use rand::prelude::*;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Color;
use std::future::Future;
use std::pin::Pin;

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

pub fn route_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(route(ctx, command))
}

async fn route(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let route = get_route();

    let footer = format!(
        "Play {}'s route next. All bois are best bois.",
        get_first_name(&route.name)
    );
    let color =
        u32::from_str_radix(&route.color, 16).expect("Failed to create a color from string.");
    let ending = if route.name.contains("Mature") || route.name.contains("Kou") {
        "Best"
    } else {
        let mut rng = rand::thread_rng();
        ENDINGS
            .choose(&mut rng)
            .expect("Failed to choose an ending.")
    };

    let author_name = get_author_name(&command.user, &command.member);
    let author_icon = get_author_avatar(&command.user);
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.create_embed(|embed| {
                    embed
                        .author(|author| author.name(&author_name).icon_url(&author_icon))
                        .color(color)
                        .field("Age", route.age, true)
                        .field("Birthday", &route.birthday, true)
                        .field("Animal Motif", &route.animal, true)
                        .footer(|f| f.text(footer))
                        .description(&route.description)
                        .thumbnail(if route.name.contains("Mature") {
                            let mut rng = rand::thread_rng();
                            let emote_id = MATURE_HIRO_EMOTE_IDS
                                .choose(&mut rng)
                                .cloned()
                                .expect("Failed to get an emote ID for mature Hiro.");
                            get_animated_emote_url(emote_id)
                        } else if route.name.contains("Kou") {
                            let mut rng = rand::thread_rng();
                            KOU_GIFS
                                .choose(&mut rng)
                                .expect("Failed to get an emote ID for Kou")
                                .to_string()
                        } else {
                            get_animated_emote_url(&route.emote_id)
                        })
                        .title(format!("Next: {}, {} Ending", &route.name, ending))
                })
            })
        })
        .await?;

    if let Some(user_records) = USER_RECORDS.get() {
        {
            let mut user_records_lock = user_records.write().await;
            let user_record_entry = user_records_lock
                .entry(command.user.id.0.to_string())
                .or_default();
            let route_entry = user_record_entry
                .route
                .entry(route.name.clone())
                .or_default();
            *route_entry.entry(format!("{} Ending", ending)).or_default() += 1;
        }
        let user_records_lock = user_records.read().await;
        write_user_records(&user_records_lock)?;
    }

    Ok(())
}

fn get_route() -> Character {
    let mut rng = rand::thread_rng();
    let random_numbers = rng.gen_range(0..100);
    match random_numbers {
        x if (0..=14).contains(&x) => ROUTES[0].clone(),
        x if (15..=19).contains(&x) => ROUTES[1].clone(),
        x if (20..=22).contains(&x) => ROUTES[7].clone(),
        x if (23..=38).contains(&x) => ROUTES[2].clone(),
        x if (39..=53).contains(&x) => ROUTES[3].clone(),
        x if (54..=68).contains(&x) => ROUTES[4].clone(),
        x if (69..=83).contains(&x) => ROUTES[5].clone(),
        x if (84..=99).contains(&x) => ROUTES[6].clone(),
        _ => ROUTES
            .choose(&mut rng)
            .cloned()
            .expect("Failed to choose a route."),
    }
}
