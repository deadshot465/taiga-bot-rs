use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::structs::record::user_record::{UserRecord, USER_RECORDS};
use crate::shared::utility::{get_author_avatar, get_author_name};
use serenity::builder::CreateEmbed;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::Embed;
use serenity::prelude::*;
use serenity::utils::Color;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub fn stats_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(stats(ctx, command))
}

async fn stats(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let user_records = USER_RECORDS.get().expect("Failed to get all user records.");

    let user_record = {
        let mut user_records_write_lock = user_records.write().await;
        let record_entry = user_records_write_lock
            .entry(command.user.id.to_string())
            .or_default();
        (*record_entry).clone()
    };

    let is_kou = KOU.get().copied().unwrap_or(false);
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let author_name = get_author_name(&command.user, &command.member);
    let author_avatar_url = get_author_avatar(&command.user);

    let embed = if let Some(option) = command.data.options.get(0) {
        let value = option
            .value
            .as_ref()
            .expect("Failed to get option value.")
            .as_str()
            .unwrap_or_default();

        match value {
            "route" => build_route_records(author_name, author_avatar_url, color, user_record),
            "valentine" => {
                build_valentine_records(author_name, author_avatar_url, color, user_record)
            }
            _ => build_all(author_name, author_avatar_url, color, user_record),
        }
    } else {
        build_all(author_name, author_avatar_url, color, user_record)
    };

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.add_embed(embed))
        })
        .await?;

    Ok(())
}

fn build_route_records(
    author_name: String,
    author_avatar_url: String,
    color: Color,
    user_record: UserRecord,
) -> CreateEmbed {
    let route_record = &user_record.route;
    let mut character_names = route_record
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    character_names.sort_unstable();

    let mut embed = CreateEmbed::default();
    embed
        .author(|a| a.name(&author_name).icon_url(author_avatar_url))
        .color(color)
        .description(format!("Here's {}'s records with `route`", author_name));
    add_route_character_fields(&mut embed, character_names, route_record);
    embed
}

fn build_valentine_records(
    author_name: String,
    author_avatar_url: String,
    color: Color,
    user_record: UserRecord,
) -> CreateEmbed {
    let valentine_record = &user_record.valentine;
    let mut character_name_and_counts = valentine_record
        .iter()
        .map(|(name, count)| (name.as_str(), *count))
        .collect::<Vec<_>>();
    character_name_and_counts.sort_by(|(_, count_1), (_, count_2)| count_2.cmp(count_1));

    let mut embed = CreateEmbed::default();
    embed
        .author(|a| a.name(&author_name).icon_url(author_avatar_url))
        .color(color)
        .description(format!("Here's {}'s records with `valentine`", author_name));
    add_valentine_character_fields(&mut embed, character_name_and_counts);
    embed
}

fn build_all(
    author_name: String,
    author_avatar_url: String,
    color: Color,
    user_record: UserRecord,
) -> CreateEmbed {
    let route_record = &user_record.route;
    let valentine_record = &user_record.valentine;

    let mut route_names = route_record
        .iter()
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    route_names.sort_unstable();

    let mut valentine_name_and_counts = valentine_record
        .iter()
        .map(|(name, count)| (name.as_str(), *count))
        .collect::<Vec<_>>();
    valentine_name_and_counts.sort_by(|(_, count_1), (_, count_2)| count_2.cmp(count_1));

    let mut embed = CreateEmbed::default();
    embed
        .author(|a| a.name(&author_name).icon_url(author_avatar_url))
        .color(color)
        .description(format!(
            "Here's {}'s records with `route, valentine`",
            author_name
        ));

    embed.field("**Route**", "Records for `route`", false);
    add_route_character_fields(&mut embed, route_names, route_record);
    embed.field("**Valentine**", "Records for `valentine`", false);
    add_valentine_character_fields(&mut embed, valentine_name_and_counts);
    embed
}

fn add_route_character_fields(
    embed: &mut CreateEmbed,
    route_names: Vec<&str>,
    route_record: &HashMap<String, HashMap<String, u16>>,
) {
    for name in route_names.into_iter() {
        let character_record = &route_record[name];
        let mut endings = character_record
            .iter()
            .map(|(ending, _)| ending.as_str())
            .collect::<Vec<_>>();
        endings.sort_unstable();
        let result = endings
            .into_iter()
            .map(|s| format!("__{}__: {}\n", s, character_record[s]))
            .collect::<String>();
        embed.field(format!("**{}**", name), result, true);
    }
}

fn add_valentine_character_fields(
    embed: &mut CreateEmbed,
    valentine_name_and_counts: Vec<(&str, u16)>,
) {
    for (name, count) in valentine_name_and_counts.into_iter() {
        embed.field(format!("**{}**", name), count, true);
    }
}
