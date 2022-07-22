use crate::shared::structs::config::configuration::KOU;
use rand::prelude::*;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

const KOU_NO_OPTIONS: &str = "Could you please provide me with options?";
const TAIGA_NO_OPTIONS: &str = "There's no option at all! What the heck?!";
const KOU_EMOJI: &str = "<:KouPoint:717505202651136051>";
const TAIGA_EMOJI: &str = "<:TaigaSmug:702210822310723614>";

pub fn pick_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(pick(ctx, command))
}

async fn pick(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let times = command
        .data
        .options
        .get(0)
        .and_then(|opt| opt.value.as_ref())
        .and_then(|value| value.as_u64())
        .map(|n| if n == 0 { 1 } else { n })
        .unwrap_or(1);

    let available_choices_raw = command
        .data
        .options
        .get(1)
        .and_then(|opt| opt.value.as_ref())
        .and_then(|value| value.as_str())
        .map(|s| s.trim());

    let is_kou = KOU.get().copied().unwrap_or(false);

    if let Some(raw_string) = available_choices_raw {
        if raw_string.is_empty() {
            cancel_pick(&ctx, &command, is_kou).await?;
        } else {
            let choices = sanitize_options(raw_string);

            if times == 1 {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response.interaction_response_data(|data| {
                            data.content(format!(
                                "{} | I pick **{}**!",
                                if is_kou { KOU_EMOJI } else { TAIGA_EMOJI },
                                single_pick(&choices)
                            ))
                        })
                    })
                    .await?;
            } else {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response.interaction_response_data(|data| data.content("Counting..."))
                    })
                    .await?;
                let choices = choices
                    .into_iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                let multiple_pick_join_handle =
                    tokio::spawn(async move { multiple_pick(choices, times) });
                let (result, result_map) = multiple_pick_join_handle.await?;
                command
                    .edit_original_interaction_response(&ctx.http, |response| {
                        response.content(build_message(result, result_map, is_kou))
                    })
                    .await?;
            }
        }
    } else {
        cancel_pick(&ctx, &command, is_kou).await?;
    }

    Ok(())
}

async fn cancel_pick(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    is_kou: bool,
) -> anyhow::Result<()> {
    let no_options_msg = if is_kou {
        KOU_NO_OPTIONS
    } else {
        TAIGA_NO_OPTIONS
    };

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content(no_options_msg))
        })
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
    let mut rng = rand::thread_rng();
    choices.choose(&mut rng).copied().unwrap_or_default()
}

fn multiple_pick(choices: Vec<String>, times: u64) -> (String, HashMap<String, u64>) {
    let mut result_map = choices
        .iter()
        .map(|s| (s.clone(), 0_u64))
        .collect::<HashMap<_, _>>();

    {
        let mut rng = rand::thread_rng();
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
    let mut result_map = result_map
        .into_iter()
        .map(|(choice, count)| (choice, count))
        .collect::<Vec<_>>();
    result_map.sort_by(|(_, count_1), (_, count_2)| count_2.cmp(count_1));

    let result_list: String = result_map
        .into_iter()
        .map(|(choice, count)| format!("⬤{} - {} times", choice, count))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "{} | I pick **{}**!\n{}",
        if is_kou { KOU_EMOJI } else { TAIGA_EMOJI },
        result,
        result_list
    )
}
