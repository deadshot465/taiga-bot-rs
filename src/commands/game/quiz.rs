#![allow(clippy::too_many_arguments)]
use crate::shared::constants::{KOU_COLOR, TAIGA_COLOR};
use crate::shared::structs::{Context, ContextData, ContextError};
use chrono::{Duration, Utc};
use once_cell::sync::OnceCell;
use poise::{ApplicationContext, CreateReply};
use rand::prelude::*;
use serenity::all::{
    Color, ComponentInteractionDataKind, CreateActionRow, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateSelectMenuKind,
    CreateSelectMenuOption, Mentionable, Message, ReactionType, User, UserId,
};
use serenity::builder::{CreateEmbed, CreateSelectMenu};
use serenity::collector::MessageCollector;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

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

const DEFAULT_ROUNDS: i32 = 7;
const STALE_TIMEOUT: u64 = 30;

static ONGOING_QUIZZES: OnceCell<RwLock<HashSet<u64>>> = OnceCell::new();

/// Play a fun quiz with your friends. Optionally specify rounds (default 7).
#[poise::command(slash_command)]
pub async fn quiz(
    ctx: Context<'_>,
    #[description = "Rounds you want to play."] rounds: Option<i32>,
) -> Result<(), ContextError> {
    let is_kou = ctx.data().kou;

    {
        let ongoing_quizzes = ONGOING_QUIZZES.get_or_init(|| RwLock::new(HashSet::new()));

        let ongoing_quizzes_read_lock = ongoing_quizzes.read().await;
        if ongoing_quizzes_read_lock.contains(&ctx.channel_id().get()) {
            ctx.send(CreateReply::default().content("A game is already running in this channel!"))
                .await?;
            return Ok(());
        }
    }

    if ctx.guild_id().is_none() {
        ctx.send(CreateReply::default().content("The quiz game can only be started in a guild!"))
            .await?;
        return Ok(());
    }

    let color = if is_kou { KOU_COLOR } else { TAIGA_COLOR };
    new_game(ctx, rounds, color, is_kou).await?;

    Ok(())
}

async fn new_game(
    ctx: Context<'_>,
    rounds: Option<i32>,
    color: Color,
    is_kou: bool,
) -> anyhow::Result<()> {
    let max_rounds = rounds.unwrap_or(DEFAULT_ROUNDS);
    ctx.send(
        CreateReply::default().content(format!("Starting a game with {} rounds...", max_rounds)),
    )
    .await?;

    if let Context::Application(app_context) = ctx {
        if let Ok(players) = join_game(ctx, color, is_kou).await {
            if let Ok(score_board) = progress_game(ctx, is_kou, &players, max_rounds).await {
                finalize(
                    ctx,
                    app_context,
                    color,
                    is_kou,
                    Some(score_board),
                    Some(&players),
                )
                .await?;
            } else {
                finalize(ctx, app_context, color, is_kou, None, Some(&players)).await?;
            }
        } else {
            finalize(ctx, app_context, color, is_kou, None, None).await?;
        }
    }

    Ok(())
}

async fn join_game(ctx: Context<'_>, color: Color, is_kou: bool) -> anyhow::Result<Vec<User>> {
    {
        let ongoing_quizzes = ONGOING_QUIZZES
            .get()
            .expect("Failed to get ongoing quizzes.");
        let mut ongoing_quizzes_write_lock = ongoing_quizzes.write().await;
        ongoing_quizzes_write_lock.insert(ctx.channel_id().get());
    }

    let joining_end_time = Utc::now() + Duration::seconds(10);
    let description = format!("React below to join the game!\nThis game may contain spoilers{}.\nCurrent players:{}\n{} seconds left!", if is_kou {
        ""
    } else {
        " or NSFW themes"
    }, "", (joining_end_time - Utc::now()).num_seconds());

    let embed = build_embed("Minigame Starting!", &description, color, None);
    if let Context::Application(app_context) = ctx {
        let sent_msg = app_context
            .interaction
            .create_followup(
                ctx.http(),
                CreateInteractionResponseFollowup::new().add_embed(embed),
            )
            .await?;
        sent_msg
            .react(ctx.http(), ReactionType::Unicode("🇴".to_string()))
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

            let embed = build_embed("Minigame Starting!", &description, color, None);

            let sent_msg = app_context
                .interaction
                .edit_followup(
                    ctx.http(),
                    sent_msg.id,
                    CreateInteractionResponseFollowup::new().embed(embed),
                )
                .await?;

            let reactions_collector = sent_msg
                .await_reactions(ctx)
                .timeout(std::time::Duration::from_secs(2));

            if let Some(reaction) = reactions_collector.next().await {
                if reaction.emoji.as_data() == "🇴" {
                    users = reaction
                        .users(
                            ctx.http(),
                            reaction.emoji.clone(),
                            None::<u8>,
                            None::<UserId>,
                        )
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
            cancel_game(ctx, app_context, color, is_kou, &sent_msg).await?;
            Err(anyhow::anyhow!("Nobody joined the game."))
        } else {
            start_game(ctx, app_context, color, is_kou, &sent_msg).await?;
            Ok(users)
        }
    } else {
        Ok(vec![])
    }
}

fn build_embed(
    title: &str,
    description: &str,
    color: Color,
    thumbnail: Option<&str>,
) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .title(title)
        .color(color)
        .description(description);
    if let Some(t) = thumbnail {
        embed.thumbnail(t)
    } else {
        embed
    }
}

async fn start_game(
    ctx: Context<'_>,
    app_context: ApplicationContext<'_, ContextData, ContextError>,
    color: Color,
    is_kou: bool,
    sent_msg: &Message,
) -> anyhow::Result<()> {
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

    app_context
        .interaction
        .edit_followup(
            ctx.http(),
            sent_msg.id,
            CreateInteractionResponseFollowup::new().embed(embed),
        )
        .await?;
    Ok(())
}

async fn cancel_game(
    ctx: Context<'_>,
    app_context: ApplicationContext<'_, ContextData, ContextError>,
    color: Color,
    is_kou: bool,
    sent_msg: &Message,
) -> anyhow::Result<()> {
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

    app_context
        .interaction
        .edit_followup(
            ctx.http(),
            sent_msg.id,
            CreateInteractionResponseFollowup::new().embed(embed),
        )
        .await?;
    Ok(())
}

async fn progress_game(
    ctx: Context<'_>,
    is_kou: bool,
    players: &[User],
    max_rounds: i32,
) -> anyhow::Result<HashMap<u64, u8>> {
    let player_ids = players.iter().map(|u| u.id.get()).collect::<Vec<_>>();
    let mut score_board = player_ids
        .iter()
        .map(|id| (*id, 0_u8))
        .collect::<HashMap<_, _>>();
    if let Context::Application(app_context) = ctx {
        let quiz_questions = {
            let mut rng = thread_rng();
            ctx.data()
                .quiz_questions
                .choose_multiple(&mut rng, max_rounds as usize)
                .zip(1..=max_rounds)
                .collect::<Vec<_>>()
        };

        let mut result: anyhow::Result<()> = Ok(());
        for (question, _) in quiz_questions.into_iter() {
            if question.question_type.as_str() == "FILL" {
                result = build_fill_question(
                    ctx,
                    app_context,
                    is_kou,
                    &mut score_board,
                    &question.question,
                    &question.answers,
                    &player_ids,
                )
                .await;
            } else if question.question_type.as_str() == "MULTIPLE" {
                result = build_multiple_choice_question(
                    ctx,
                    app_context,
                    is_kou,
                    &mut score_board,
                    &question.question,
                    &question.answers.first().cloned().unwrap_or_default(),
                    &question.wrong,
                    &player_ids,
                )
                .await;
            }

            if result.is_err() {
                return Err(anyhow::anyhow!("Game is cancelled."));
            }
        }
    }

    Ok(score_board)
}

async fn build_fill_question(
    ctx: Context<'_>,
    app_context: ApplicationContext<'_, ContextData, ContextError>,
    is_kou: bool,
    score_board: &mut HashMap<u64, u8>,
    question: &str,
    answers: &[String],
    player_ids: &[u64],
) -> anyhow::Result<()> {
    app_context
        .interaction
        .create_followup(
            ctx.http(),
            CreateInteractionResponseFollowup::new().content(question),
        )
        .await?;

    let delay = tokio::time::sleep(std::time::Duration::from_secs(STALE_TIMEOUT));
    tokio::pin!(delay);

    let player_ids = player_ids.to_vec();

    loop {
        let cloned_player_ids = player_ids.clone();
        let collector = MessageCollector::new(ctx)
            .channel_id(ctx.channel_id())
            .guild_id(ctx.guild_id().unwrap_or_default())
            .filter(move |m| cloned_player_ids.contains(&m.author.id.get()));

        tokio::select! {
            _ = &mut delay => {
                app_context
                    .interaction
                    .create_followup(ctx.http(), CreateInteractionResponseFollowup::new()
                    .content("Cancelling stale game..."))
                    .await?;
                return Err(anyhow::anyhow!("Game is cancelled."));
            }
            maybe_v = collector.next() => {
                if let Some(ref msg) = maybe_v {
                    if answers.iter()
                    .map(|s| s.to_lowercase())
                    .any(|s| s == msg.content.to_lowercase()) {
                        let random_response = get_random_response(is_kou);

                        app_context
                            .interaction
                            .create_followup(ctx.http(), CreateInteractionResponseFollowup::new()
                            .content(format!("{} {}", msg.author.mention(), random_response)))
                            .await?;
                        let score_entry = score_board.entry(msg.author.id.get()).or_default();
                        *score_entry += 1;
                        return Ok(());
                    }
                }
            }
        }
    }
}

async fn build_multiple_choice_question(
    ctx: Context<'_>,
    app_context: ApplicationContext<'_, ContextData, ContextError>,
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
        let mut rng = thread_rng();
        shuffled_answers.shuffle(&mut rng);
    }

    let answer_options = shuffled_answers
        .into_iter()
        .enumerate()
        .map(|(no, choice)| CreateSelectMenuOption::new(format!("{}. {}", no + 1, choice), choice))
        .collect::<Vec<_>>();

    let sent_msg = app_context
        .interaction
        .create_followup(
            ctx.http(),
            CreateInteractionResponseFollowup::new()
                .content(question)
                .components(vec![CreateActionRow::SelectMenu(
                    CreateSelectMenu::new(
                        "multiple_choice",
                        CreateSelectMenuKind::String {
                            options: answer_options,
                        },
                    )
                    .min_values(1)
                    .max_values(1)
                    .placeholder("Pick an answer!"),
                )]),
        )
        .await?;

    let delay = tokio::time::sleep(std::time::Duration::from_secs(STALE_TIMEOUT));
    tokio::pin!(delay);

    let player_ids = player_ids.to_vec();

    loop {
        let cloned_player_ids = player_ids.clone();
        let collector = sent_msg
            .await_component_interaction(ctx)
            .channel_id(ctx.channel_id())
            .guild_id(ctx.guild_id().unwrap_or_default())
            .filter(move |interaction| cloned_player_ids.contains(&interaction.user.id.get()));

        tokio::select! {
            _ = &mut delay => {
                app_context.interaction.create_followup(ctx.http(), CreateInteractionResponseFollowup::new().content("Cancelling stale game...")).await?;
                return Err(anyhow::anyhow!("Game is cancelled."));
            }
            maybe_v = collector.next() => {
                if let Some(ref interaction) = maybe_v {
                    if let ComponentInteractionDataKind::StringSelect {
                        values
                    } = interaction.data.kind.clone() {
                        let value = values[0].as_str();
                        if value == answer {
                            let random_response = get_random_response(is_kou);

                            interaction
                                .create_response(ctx.http(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                                .content(format!("{} {}", interaction.user.mention(), random_response))))
                                .await?;

                            let score_entry = score_board.entry(interaction.user.id.get()).or_default();
                            *score_entry += 1;
                            app_context
                                .interaction
                                .delete_followup(ctx.http(), sent_msg.id).await?;
                            return Ok(());
                        } else {
                            app_context
                                .interaction
                                .create_response(ctx.http(), CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                                .content(format!("{}, that's not the correct answer!", interaction.user.mention()))))
                                .await?;
                        }
                    }
                }
            }
        }
    }
}

async fn finalize(
    ctx: Context<'_>,
    app_context: ApplicationContext<'_, ContextData, ContextError>,
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
        ongoing_quizzes_write_lock.remove(&ctx.channel_id().get());
    }

    if let Some(board) = score_board {
        let players = players.expect("Failed to get participating players.");
        let mut score_board = board
            .into_iter()
            .map(|(user_id, score)| {
                (
                    players
                        .iter()
                        .find(|u| u.id.get() == user_id)
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
            .map(|(rank, (name, score))| format!("{}) {} with {} points", rank + 1, name, score))
            .collect::<Vec<_>>();

        app_context
            .interaction
            .create_followup(
                ctx.http(),
                CreateInteractionResponseFollowup::new().embed(
                    CreateEmbed::new()
                        .title("Minigame ended!")
                        .description(format!("Total points:\n{}", result_string.join("\n")))
                        .thumbnail(if is_kou {
                            "https://cdn.discordapp.com/emojis/717505202651136051.png"
                        } else {
                            "https://cdn.discordapp.com/emojis/706757435553218620.png"
                        })
                        .color(color),
                ),
            )
            .await?;
    }

    Ok(())
}

fn get_random_response(is_kou: bool) -> &'static str {
    let mut rng = thread_rng();
    if is_kou {
        KOU_RESPONSES.choose(&mut rng).cloned().unwrap_or_default()
    } else {
        TAIGA_RESPONSES
            .choose(&mut rng)
            .cloned()
            .unwrap_or_default()
    }
}
