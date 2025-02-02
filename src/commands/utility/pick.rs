use std::collections::HashMap;

use num_traits::cast::FromPrimitive;
use poise::CreateReply;
use rand::prelude::*;

use crate::shared::structs::{Context, ContextError};

const KOU_NO_OPTIONS: &str = "Could you please provide me with options?";
const TAIGA_NO_OPTIONS: &str = "There's no option at all! What the heck?!";
const KOU_EMOJI: &str = "<:KouPoint:717505202651136051>";
const TAIGA_EMOJI: &str = "<:TaigaSmug:702210822310723614>";

/// Pick from several options.
#[poise::command(slash_command, category = "Utility")]
pub async fn pick(
    ctx: Context<'_>,
    #[description = "Times to pick. Negative numbers or numbers too big will be ignored."]
    times: Option<i32>,
    #[description = "Choices to pick from, separated by pipe (|)."] choices: String,
) -> Result<(), ContextError> {
    let times = times
        .map(|value| value as i64)
        .and_then(u64::from_i64)
        .map(|n| if n == 0 { 1 } else { n })
        .unwrap_or(1);

    let raw_string = choices.trim();
    let is_kou = ctx.data().kou;

    if raw_string.is_empty() {
        cancel_pick(ctx, is_kou).await?;
    } else {
        let choices = sanitize_options(raw_string);

        if times == 1 {
            ctx.send(CreateReply::default().content(format!(
                "{} | I pick **{}**!",
                if is_kou { KOU_EMOJI } else { TAIGA_EMOJI },
                single_pick(&choices)
            )))
            .await?;
        } else {
            let reply_handle = ctx
                .send(CreateReply::default().content("Counting..."))
                .await?;
            let choices = choices
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            let multiple_pick_join_handle =
                tokio::spawn(async move { multiple_pick(choices, times) });
            let (result, result_map) = multiple_pick_join_handle.await?;

            reply_handle
                .edit(
                    ctx,
                    CreateReply::default().content(build_message(result, result_map, is_kou)),
                )
                .await?;
        }
    }

    Ok(())
}

async fn cancel_pick(ctx: Context<'_>, is_kou: bool) -> anyhow::Result<()> {
    let no_options_msg = if is_kou {
        KOU_NO_OPTIONS
    } else {
        TAIGA_NO_OPTIONS
    };

    ctx.send(CreateReply::default().content(no_options_msg))
        .await?;

    Ok(())
}

fn sanitize_options(raw_string: &str) -> Vec<&str> {
    raw_string
        .split('|')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn single_pick<'a>(choices: &'a [&str]) -> &'a str {
    let mut rng = rand::rng();
    choices.choose(&mut rng).copied().unwrap_or_default()
}

fn multiple_pick(choices: Vec<String>, times: u64) -> (String, HashMap<String, u64>) {
    let mut result_map = choices
        .iter()
        .map(|s| (s.clone(), 0_u64))
        .collect::<HashMap<_, _>>();

    {
        let mut rng = rand::rng();
        for _ in 0..times {
            if let Some(opt) = choices.choose(&mut rng) {
                let entry = result_map
                    .entry(opt.clone())
                    .or_insert_with(Default::default);
                *entry += 1;
            }
        }
    }

    let result = result_map
        .iter()
        .max_by(|(_, count_1), (_, count_2)| (*count_1).cmp(*count_2))
        .map(|(choice, _)| choice.clone())
        .unwrap_or_default();

    (result, result_map)
}

fn build_message(result: String, result_map: HashMap<String, u64>, is_kou: bool) -> String {
    let mut result_map = result_map.into_iter().collect::<Vec<_>>();
    result_map.sort_by(|(_, count_1), (_, count_2)| count_2.cmp(count_1));

    let result_list: String = result_map
        .into_iter()
        .map(|(choice, count)| format!("- {} - {} times", choice, count))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{} | I pick **{}**!\n{}",
        if is_kou { KOU_EMOJI } else { TAIGA_EMOJI },
        result,
        result_list
    )
}
