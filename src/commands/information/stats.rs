use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;

use poise::CreateReply;
use serenity::all::{Color, CreateEmbedAuthor};
use serenity::builder::CreateEmbed;

use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::record::user_record::UserRecord;
use crate::shared::structs::{Context, ContextError};
use crate::shared::utility::{get_author_avatar, get_author_name};

#[derive(Debug, Copy, Clone, poise::ChoiceParameter)]
pub enum StatChoice {
    #[name = "route"]
    Route,
    #[name = "valentine"]
    Valentine,
}

/// This command will show your records with several commands.
#[poise::command(slash_command, category = "Information")]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "(Optional) The command of which you want to query the record."]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<StatChoice>,
) -> Result<(), ContextError> {
    let user_records = ctx.data().user_records.clone();
    let author = ctx.author();

    let user_record = {
        let mut user_records_write_lock = user_records.write().await;
        let record_entry = user_records_write_lock
            .entry(author.id.to_string())
            .or_default();
        (*record_entry).clone()
    };

    let is_kou = ctx.data().kou;
    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    let member = ctx.author_member().await.map(|member| match member {
        Cow::Borrowed(m) => m.clone(),
        Cow::Owned(m) => m,
    });
    let author_name = get_author_name(author, &member);
    let author_avatar_url = get_author_avatar(author);

    let embed = if let Some(cmd) = command {
        match cmd {
            StatChoice::Route => {
                build_route_records(author_name, author_avatar_url, color, user_record)
            }
            StatChoice::Valentine => {
                build_valentine_records(author_name, author_avatar_url, color, user_record)
            }
        }
    } else {
        build_all(author_name, author_avatar_url, color, user_record)
    };

    ctx.send(CreateReply::default().embed(embed)).await?;

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

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(&author_name).icon_url(author_avatar_url))
        .color(color)
        .description(format!("Here's {}'s records with `route`", author_name));
    add_route_character_fields(embed, character_names, route_record)
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

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(&author_name).icon_url(author_avatar_url))
        .color(color)
        .description(format!("Here's {}'s records with `valentine`", author_name));
    add_valentine_character_fields(embed, character_name_and_counts)
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

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(&author_name).icon_url(author_avatar_url))
        .color(color)
        .description(format!(
            "Here's {}'s records with `route, valentine`",
            author_name
        ));

    let embed = embed.field("**Route**", "Records for `route`", false);
    let embed = add_route_character_fields(embed, route_names, route_record);
    let embed = embed.field("**Valentine**", "Records for `valentine`", false);
    add_valentine_character_fields(embed, valentine_name_and_counts)
}

fn add_route_character_fields(
    embed: CreateEmbed,
    route_names: Vec<&str>,
    route_record: &HashMap<String, HashMap<String, u16>>,
) -> CreateEmbed {
    let fields = route_names
        .into_iter()
        .map(|name| {
            let character_record = &route_record[name];
            let mut endings = character_record
                .iter()
                .map(|(ending, _)| ending.as_str())
                .collect::<Vec<_>>();
            endings.sort_unstable();
            let result = endings
                .into_iter()
                .fold(String::new(), |mut output, ending| {
                    let _ = writeln!(output, "__{}__: {}", ending, character_record[ending]);
                    output
                });
            (format!("**{}**", name), result, true)
        })
        .collect::<Vec<_>>();

    embed.fields(fields)
}

fn add_valentine_character_fields(
    embed: CreateEmbed,
    valentine_name_and_counts: Vec<(&str, u16)>,
) -> CreateEmbed {
    let fields = valentine_name_and_counts
        .into_iter()
        .map(|(name, count)| (format!("**{}**", name), count.to_string(), true))
        .collect::<Vec<_>>();

    embed.fields(fields)
}
