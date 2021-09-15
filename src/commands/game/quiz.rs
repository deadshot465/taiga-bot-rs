use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::config::configuration::KOU;
use crate::shared::structs::game::quiz_question::QUIZ_QUESTIONS;
use chrono::{Duration, Utc};
use once_cell::sync::OnceCell;
use rand::prelude::*;
use serenity::builder::CreateEmbed;
use serenity::collector::{MessageCollector, MessageCollectorBuilder};
use serenity::futures::StreamExt;
use serenity::model::prelude::application_command::*;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;

const TAIGA_RESPONSES: [&str; 5] = [
    "Nice one!",
    "That's my sidekick!",
    "Guess you're not an amateur after all! <:TaigaSmug:702210822310723614>",
    "Excellent!",
    "Great! <:TaigaHappy3:887795984803979314>",
];

const KOU_RESPONSES: [&str; 5] = [
    "Good job!",
    "I know you can do it! <:KouSmile2:705182851817144330>",
    "Nice work! <:KouCompassion:705054435696443532>",
    "Way to go! ",
    "Great! <:KouSmug:736061465848578091>",
];

const DEFAULT_ROUNDS: u64 = 7;
const STALE_TIMEOUT: u64 = 30;

static ONGOING_QUIZZES: OnceCell<RwLock<HashSet<u64>>> = OnceCell::new();

pub fn quiz_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(quiz(ctx, command))
}

async fn quiz(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    let is_kou = KOU.get().copied().unwrap_or(false);

    {
        let ongoing_quizzes = ONGOING_QUIZZES.get_or_init(|| RwLock::new(HashSet::new()));

        let ongoing_quizzes_read_lock = ongoing_quizzes.read().await;
        if ongoing_quizzes_read_lock.contains(&command.channel_id.0) {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|data| {
                        data.content("A game is already running in this channel!")
                    })
                })
                .await?;
            return Ok(());
        }
    }

    if command.guild_id.is_none() {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|data| {
                    data.content("The quiz game can only be started in a guild!")
                })
            })
            .await?;
        return Ok(());
    }

    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    new_game(&ctx, &command, color, is_kou).await?;

    Ok(())
}

async fn new_game(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    color: Color,
    is_kou: bool,
) -> anyhow::Result<()> {
    let max_rounds = extract_rounds(command);
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.content(format!("Starting a game with {} rounds...", max_rounds))
            })
        })
        .await?;

    if let Ok(players) = join_game(ctx, command, color, is_kou).await {
        if let Ok(score_board) = progress_game(ctx, command, is_kou, &players, max_rounds).await {
            finalize(
                ctx,
                command,
                color,
                is_kou,
                Some(score_board),
                Some(&players),
            )
            .await?;
        } else {
            finalize(ctx, command, color, is_kou, None, Some(&players)).await?;
        }
    } else {
        finalize(ctx, command, color, is_kou, None, None).await?;
    }

    Ok(())
}

fn extract_rounds(command: &ApplicationCommandInteraction) -> u64 {
    if let Some(opt) = command.data.options.get(0) {
        let value = opt
            .options
            .get(0)
            .and_then(|value| value.value.as_ref())
            .and_then(|value| value.as_u64())
            .unwrap_or(DEFAULT_ROUNDS);
        if value < 2 || value > 10 {
            DEFAULT_ROUNDS
        } else {
            value
        }
    } else {
        DEFAULT_ROUNDS
    }
}

async fn join_game(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    color: Color,
    is_kou: bool,
) -> anyhow::Result<Vec<User>> {
    {
        let ongoing_quizzes = ONGOING_QUIZZES
            .get()
            .expect("Failed to get ongoing quizzes.");
        let mut ongoing_quizzes_write_lock = ongoing_quizzes.write().await;
        ongoing_quizzes_write_lock.insert(command.channel_id.0);
    }

    let joining_end_time = Utc::now() + Duration::seconds(10);
    let description = format!("React below to join the game!\nThis game may contain spoilers{}.\nCurrent players:{}\n{} seconds left!", if is_kou {
        ""
    } else {
        " or NSFW themes"
    }, "", (joining_end_time - Utc::now()).num_seconds());

    let embed = build_embed("Minigame Starting!", &description, color, None);
    let sent_msg = command
        .create_followup_message(&ctx.http, |response| response.add_embed(embed))
        .await?;
    sent_msg
        .react(&ctx.http, ReactionType::Unicode("🇴".to_string()))
        .await?;

    let mut users: Vec<User> = vec![];
    loop {
        let user_mentions = users
            .iter()
            .map(|u| u.mention().to_string())
            .collect::<Vec<_>>();

        let description = format!("React below to join the game!\nThis game may contain spoilers{}.\nCurrent players:{}\n{} seconds left!", if is_kou {
            ""
        } else {
            " or NSFW themes"
        }, user_mentions.join(", "), (joining_end_time - Utc::now()).num_seconds());

        let sent_msg = command
            .edit_followup_message(&ctx.http, &sent_msg.id, |response| {
                let embed = build_embed("Minigame Starting!", &description, color, None);
                response.embeds(vec![embed])
            })
            .await?;

        let mut reactions_collector = sent_msg
            .await_reactions(&ctx)
            .timeout(std::time::Duration::from_secs(2))
            .removed(true)
            .await;

        while let Some(reaction) = reactions_collector.next().await {
            let emoji = &reaction.as_inner_ref().emoji;
            if emoji.as_data().as_str() == "🇴" {
                users = reaction
                    .as_inner_ref()
                    .users(&ctx.http, emoji.clone(), None::<u8>, None::<UserId>)
                    .await
                    .unwrap_or_default();
                users = users.into_iter().filter(|u| !u.bot).collect::<Vec<_>>();
            }
        }

        if Utc::now() > joining_end_time {
            break;
        }
    }

    if users.is_empty() {
        cancel_game(ctx, command, color, is_kou, &sent_msg).await?;
        Err(anyhow::anyhow!("Nobody joined the game."))
    } else {
        start_game(ctx, command, color, is_kou, &sent_msg).await?;
        Ok(users)
    }
}

fn build_embed(
    title: &str,
    description: &str,
    color: Color,
    thumbnail: Option<&str>,
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(title).color(color).description(description);
    if let Some(t) = thumbnail {
        embed.thumbnail(t);
    }
    embed
}

async fn start_game(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    color: Color,
    is_kou: bool,
    sent_msg: &Message,
) -> anyhow::Result<()> {
    command
        .edit_followup_message(&ctx.http, &sent_msg.id, |response| {
            let embed = build_embed(
                "Minigame Started!",
                "The game has begun!",
                color,
                if is_kou {
                    Some("https://cdn.discordapp.com/emojis/705182851754360912.png")
                } else {
                    Some("https://cdn.discordapp.com/emojis/702210822310723614.png")
                },
            );
            response.embeds(vec![embed])
        })
        .await?;
    Ok(())
}

async fn cancel_game(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    color: Color,
    is_kou: bool,
    sent_msg: &Message,
) -> anyhow::Result<()> {
    command
        .edit_followup_message(&ctx.http, &sent_msg.id, |response| {
            let embed = build_embed(
                "Minigame Cancelled!",
                "Nobody joined...",
                color,
                if is_kou {
                    Some("https://cdn.discordapp.com/emojis/736061517534855201.png")
                } else {
                    Some("https://cdn.discordapp.com/emojis/701226059726585866.png")
                },
            );
            response.embeds(vec![embed])
        })
        .await?;
    Ok(())
}

async fn progress_game(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    is_kou: bool,
    players: &[User],
    max_rounds: u64,
) -> anyhow::Result<HashMap<u64, u8>> {
    let mut score_board = HashMap::new();

    let player_ids = players.iter().map(|u| u.id.0).collect::<Vec<_>>();
    let cloned_player_ids = player_ids.clone();
    let mut collector = MessageCollectorBuilder::new(ctx)
        .channel_id(command.channel_id.0)
        .guild_id(command.guild_id.unwrap_or_default().0)
        .filter(move |m| cloned_player_ids.contains(&m.author.id.0))
        .await;

    let quiz_questions = {
        let rounds = (1..=max_rounds).collect::<Vec<_>>();
        let mut rng = rand::thread_rng();
        QUIZ_QUESTIONS
            .choose_multiple(&mut rng, max_rounds as usize)
            .zip(rounds.into_iter())
            .collect::<Vec<_>>()
    };

    let mut result: anyhow::Result<()> = Ok(());
    for (question, _) in quiz_questions.into_iter() {
        if question.question_type.as_str() == "FILL" {
            result = build_fill_question(
                ctx,
                command,
                is_kou,
                &mut score_board,
                &question.question,
                &question.answers,
                &mut collector,
            )
            .await;
        } else if question.question_type.as_str() == "MULTIPLE" {
            result = build_multiple_choice_question(
                ctx,
                command,
                is_kou,
                &mut score_board,
                &question.question,
                &question.answers.get(0).cloned().unwrap_or_default(),
                &question.wrong,
                &player_ids,
            )
            .await;
        }

        if result.is_err() {
            return Err(anyhow::anyhow!("Game is cancelled."));
        }
    }

    Ok(score_board)
}

async fn build_fill_question(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    is_kou: bool,
    score_board: &mut HashMap<u64, u8>,
    question: &str,
    answers: &[String],
    collector: &mut MessageCollector,
) -> anyhow::Result<()> {
    command
        .create_followup_message(&ctx.http, |response| response.content(question))
        .await?;

    let delay = tokio::time::sleep(std::time::Duration::from_secs(STALE_TIMEOUT));
    tokio::pin!(delay);

    loop {
        tokio::select! {
            _ = &mut delay => {
                command.create_followup_message(&ctx.http, |response| {
                    response.content("Cancelling stale game...")
                }).await?;
                return Err(anyhow::anyhow!("Game is cancelled."));
            }
            maybe_v = collector.next() => {
                if let Some(msg) = maybe_v {
                    if answers.iter()
                    .map(|s| s.to_lowercase())
                    .any(|s| s == msg.content.to_lowercase()) {
                        let random_response = get_random_response(is_kou);

                        command.create_followup_message(&ctx.http, |response| {
                            response.content(format!("{} {}", msg.author.mention(), random_response))
                        }).await?;
                        let score_entry = score_board.entry(msg.author.id.0).or_default();
                        *score_entry += 1;
                        return Ok(());
                    }
                }
            }
        }
    }
}

async fn build_multiple_choice_question(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    is_kou: bool,
    score_board: &mut HashMap<u64, u8>,
    question: &str,
    answer: &str,
    wrong_answers: &[String],
    player_ids: &[u64],
) -> anyhow::Result<()> {
    let mut shuffled_answers = wrong_answers.to_vec();
    shuffled_answers.push(answer.into());
    {
        let mut rng = rand::thread_rng();
        shuffled_answers.shuffle(&mut rng);
    }

    let sent_msg = command
        .create_followup_message(&ctx.http, |response| {
            response.content(question).components(|component| {
                component.create_action_row(|row| {
                    row.create_select_menu(|menu| {
                        menu.min_values(1)
                            .max_values(1)
                            .placeholder("Pick an answer!")
                            .custom_id("multiple_choice")
                            .options(|opts| {
                                for (no, choice) in shuffled_answers.iter().enumerate() {
                                    opts.create_option(|opt| {
                                        opt.label(format!("{}. {}", no + 1, choice)).value(choice)
                                    });
                                }
                                opts
                            })
                    })
                })
            })
        })
        .await?;

    let delay = tokio::time::sleep(std::time::Duration::from_secs(STALE_TIMEOUT));
    tokio::pin!(delay);

    loop {
        let cloned_player_ids = player_ids.to_vec();
        let collector = sent_msg
            .await_component_interaction(ctx)
            .channel_id(command.channel_id.0)
            .guild_id(command.guild_id.unwrap_or_default().0)
            .filter(move |interaction| cloned_player_ids.contains(&interaction.user.id.0));

        tokio::select! {
            _ = &mut delay => {
                command.create_followup_message(&ctx.http, |response| {
                    response.content("Cancelling stale game...")
                }).await?;
                return Err(anyhow::anyhow!("Game is cancelled."));
            }
            maybe_v = collector => {
                if let Some(interaction) = maybe_v {
                    if let Some(value) = interaction.data.values.get(0) {
                        if value.as_str() == answer {
                            let random_response = get_random_response(is_kou);

                            command.create_followup_message(&ctx.http, |response| {
                                response.content(format!("{} {}", interaction.user.mention(), random_response))
                            }).await?;

                            let score_entry = score_board.entry(interaction.user.id.0).or_default();
                            *score_entry += 1;
                            command.delete_followup_message(&ctx.http, &sent_msg.id).await?;
                            return Ok(());
                        } else {
                            command.create_followup_message(&ctx.http, |response| {
                                response.content(format!("{}, that's not the correct answer!", interaction.user.mention()))
                            }).await?;
                        }
                    }
                }
            }
        }
    }
}

async fn finalize(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    color: Color,
    is_kou: bool,
    score_board: Option<HashMap<u64, u8>>,
    players: Option<&[User]>,
) -> anyhow::Result<()> {
    {
        let ongoing_quizzes = ONGOING_QUIZZES
            .get()
            .expect("Failed to get ongoing quizzes.");
        let mut ongoing_quizzes_write_lock = ongoing_quizzes.write().await;
        ongoing_quizzes_write_lock.remove(&command.channel_id.0);
    }

    if let Some(board) = score_board {
        let players = players.expect("Failed to get participating players.");
        let mut score_board = board
            .into_iter()
            .map(|(user_id, score)| {
                (
                    players
                        .iter()
                        .find(|u| (*u).id.0 == user_id)
                        .expect("Failed to map user ID to an user.")
                        .mention()
                        .to_string(),
                    score,
                )
            })
            .collect::<Vec<_>>();

        score_board.sort_by(|(_, score_a), (_, score_b)| score_b.cmp(score_a));

        let result_string = score_board
            .into_iter()
            .enumerate()
            .into_iter()
            .map(|(rank, (name, score))| format!("{}) {} with {} points", rank + 1, name, score))
            .collect::<Vec<_>>();

        command
            .create_followup_message(&ctx.http, |response| {
                response.create_embed(|embed| {
                    embed
                        .title("Minigame ended!")
                        .description(format!("Total points:\n{}", result_string.join("\n")))
                        .thumbnail(if is_kou {
                            "https://cdn.discordapp.com/emojis/717505202651136051.png"
                        } else {
                            "https://cdn.discordapp.com/emojis/706757435553218620.png"
                        })
                        .color(color)
                })
            })
            .await?;
    }

    Ok(())
}

fn get_random_response(is_kou: bool) -> &'static str {
    let mut rng = rand::thread_rng();
    if is_kou {
        KOU_RESPONSES.choose(&mut rng).cloned().unwrap_or_default()
    } else {
        TAIGA_RESPONSES
            .choose(&mut rng)
            .cloned()
            .unwrap_or_default()
    }
}
