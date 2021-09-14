#![allow(clippy::clone_on_copy)]
use crate::{InterfaceService, PersistenceService, QuizQuestion};
use chrono::{Duration, Utc};
use rand::prelude::*;
use serenity::collector::MessageCollectorBuilder;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::futures::StreamExt;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::guild::Member;
use serenity::model::id::UserId;
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::utils::Color;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const TAIGA_RESPONSES: [&str; 5] = [
    "Nice one!",
    "That's my sidekick!",
    "Guess you're not an amateur after all! <:TaigaSmug:702210822310723614>",
    "Excellent!",
    "Great! <:TaigaHappy3:706757435553218620>",
];

const KOU_RESPONSES: [&str; 5] = [
    "Good job!",
    "I know you can do it! <:KouSmile2:705182851817144330>",
    "Nice work! <:KouCompassion:705054435696443532>",
    "Way to go! ",
    "Great! <:KouSmug:736061465848578091>",
];

#[command]
#[description = "Play a fun quiz with your friends. Optionally specify rounds (default 7). This command has to be prefixed with `games`."]
#[usage = "10"]
#[example = "10"]
#[bucket = "games"]
pub async fn quiz(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = context.data.read().await;
    let interface = data
        .get::<InterfaceService>()
        .expect("Failed to get interface service.");
    let persistence = data
        .get::<PersistenceService>()
        .expect("Failed to get assets service.");
    let interface_lock = interface.read().await;
    // Check if it's Kou or Taiga, since Taiga's quizzes may contain NSFW contents.
    let is_kou = interface_lock.is_kou;
    let _persistence = Arc::clone(persistence);
    drop(interface_lock);
    drop(data);
    let persistence_lock = _persistence.read().await;
    // Borrow http for use in subsequent messages.
    let http = &context.http;

    // Acquire ongoing quizzes first so we don't create another game in the same channel.
    {
        let ongoing_quizzes = persistence_lock
            .ongoing_quizzes
            .as_ref()
            .expect("Failed to acquire ongoing quizzes.");

        // Check if there's an existing game.
        if ongoing_quizzes.contains(&msg.channel_id.0) {
            // Tell user that a game is already running.
            msg.channel_id
                .say(http, "A game is already running!")
                .await?;
            return Ok(());
        }
    }
    drop(persistence_lock);

    // Build color to be used in embeds beforehand.
    let color_value = u32::from_str_radix(if is_kou { "e7a43a" } else { "e81615" }, 16)
        .expect("Failed to create u32 from string.");
    let color = Color::new(color_value);

    // Check if rounds are specified. If there's any error, sanitize it and set max rounds to 7.
    let max_rounds = args.single::<u8>().unwrap_or(7);
    // Limit max rounds to between 2 and 10.
    if max_rounds < 2 || max_rounds > 10 {
        msg.channel_id
            .say(
                http,
                "The number of rounds has to be greater than 1 and less than 11!",
            )
            .await?;
        return Ok(());
    }
    msg.channel_id
        .say(
            http,
            format!("Starting a game with {} rounds...", max_rounds).as_str(),
        )
        .await?;
    let (game_started, players) = join_game(context, msg, &color, is_kou).await;

    let mut result: Option<HashMap<u64, u8>> = None;
    // If game starts, wait for game result.
    if game_started {
        let progress_result = progress(
            context,
            msg,
            max_rounds,
            is_kou,
            &players.expect("Failed to get participating players."),
        )
        .await;
        if progress_result.is_ok() {
            result = Some(progress_result.expect("Failed to get progress result."));
        } else {
            result = None;
        }
    } else {
        // Otherwise clean up and unregister game.
        end_game(context, msg, false, None, is_kou, &color).await?;
        return Ok(());
    }

    // If the result is none, that means the game is aborted. Clean up and unregister game.
    if result.is_none() {
        end_game(context, msg, false, None, is_kou, &color).await?;
        return Ok(());
    }
    // Otherwise, show final results.
    end_game(context, msg, true, result.as_ref(), is_kou, &color).await?;
    Ok(())
}

/// A simple helper function for creating the initial embed.
async fn build_embed(
    context: &Context,
    msg: &Message,
    title: &str,
    description: &str,
    color: &Color,
) -> Message {
    let message = msg
        .channel_id
        .send_message(&context.http, |m| {
            m.embed(|e| {
                e.color(color.clone());
                e.title(title);
                e.description(description);
                e
            })
        })
        .await
        .expect("Failed to send embed for quiz.");
    message
}

/// Handles player joining.
async fn join_game(
    context: &Context,
    msg: &Message,
    color: &Color,
    is_kou: bool,
) -> (bool, Option<Vec<User>>) {
    let data = context.data.read().await;
    let persistence = data
        .get::<PersistenceService>()
        .expect("Failed to get assets service.");
    let mut persistence_lock = persistence.write().await;
    // Add the current channel to ongoing quizzes.
    let ongoing_quizzes = persistence_lock
        .ongoing_quizzes
        .as_mut()
        .expect("Failed to get ongoing quizzes.");
    let _ = ongoing_quizzes.insert(msg.channel_id.0);
    drop(persistence_lock);
    drop(data);

    let http = &context.http;
    // Build welcoming messages and allow users to join.
    let mut users: Vec<User> = vec![];
    // Setting up the time limit for joining in the game.
    let end_joining_time = Utc::now() + Duration::seconds(10);
    // Setting up the initial embed so we will have an embed to edit.
    let mut description = format!("React below to join the game!\nThis game may contain spoilers{}.\nCurrent players:{}\n{} seconds left!", if is_kou {
        ""
    } else {
        " or NSFW themes"
    }, "", (end_joining_time - Utc::now()).num_seconds());
    let mut message = build_embed(
        context,
        msg,
        "Minigame Starting!",
        description.as_str(),
        color,
    )
    .await;
    // Initial reaction made by Kou, to denote the correct emoji that should be used.
    message
        .react(http, ReactionType::Unicode("🇴".to_string()))
        .await
        .expect("Failed to react to joining message.");
    // Loop until 10 seconds have already passed.
    loop {
        // Get user mentions from participating players.
        let user_mentions = users
            .iter()
            .map(|u| u.mention().to_string())
            .collect::<Vec<String>>();
        // Kou will possibly only have SFW questions.
        // While Taiga might have NSFW questions.
        description = format!("React below to join the game!\nThis game may contain spoilers{}.\nCurrent players:{}\n{} seconds left!", if is_kou {
            ""
        } else {
            " or NSFW themes"
        }, user_mentions.join(", "), (end_joining_time - Utc::now()).num_seconds());
        // Edit the message to show current participants.
        message
            .edit(http, |m| {
                m.embed(|e| {
                    e.color(color.clone());
                    e.title("Minigame Starting!");
                    e.description(&description);
                    e
                })
            })
            .await
            .expect("Failed to edit the embed message.");
        // Collect added and removed reactions.
        if let Some(reaction) = message
            .await_reaction(&context)
            .timeout(tokio::time::Duration::from_secs(2))
            .removed(true)
            .await
        {
            let emoji = &reaction.as_inner_ref().emoji;
            // Pattern matching to get the reaction we need.
            if let "🇴" = emoji.as_data().as_str() {
                users = reaction
                    .as_inner_ref()
                    .users(http, emoji.clone(), None::<u8>, None::<UserId>)
                    .await
                    .unwrap_or_default();
                users = users.into_iter().filter(|u| !u.bot).collect::<Vec<User>>();
            };
        }
        // If 10 seconds have already passed,
        // Break the loop and enter the next game stage.
        if Utc::now() > end_joining_time {
            break;
        }
    }

    // If nobody joins, cancel and unregister the game by returning false and None.
    if users.is_empty() {
        message
            .edit(http, |m| {
                m.embed(|e| {
                    e.color(color.clone());
                    e.title("Minigame Cancelled!");
                    e.description("Nobody joined...");
                    e.thumbnail(if is_kou {
                        "https://cdn.discordapp.com/emojis/736061517534855201.png"
                    } else {
                        "https://cdn.discordapp.com/emojis/701226059726585866.png"
                    });
                    e
                })
            })
            .await
            .expect("Failed to send game failed message.");
        (false, None)
    } else {
        // Otherwise starts the game by returning true and valid player list.
        message
            .edit(http, |m| {
                m.embed(|e| {
                    e.color(color.clone());
                    e.title("Minigame Started!");
                    e.description("The game has begun!");
                    e.thumbnail(if is_kou {
                        "https://cdn.discordapp.com/emojis/705182851754360912.png"
                    } else {
                        "https://cdn.discordapp.com/emojis/702210822310723614.png"
                    });
                    e
                })
            })
            .await
            .expect("Failed to send game started message.");
        (true, Some(users))
    }
}

/// The main game loop.
async fn progress(
    context: &Context,
    msg: &Message,
    max_rounds: u8,
    is_kou: bool,
    players: &[User],
) -> std::result::Result<HashMap<u64, u8>, CommandError> {
    let data = context.data.read().await;
    let persistence = data
        .get::<PersistenceService>()
        .expect("Failed to get assets service.");
    let persistence_lock = persistence.read().await;
    let http = &context.http;
    let mut score_board = HashMap::<u64, u8>::new();
    let mut current_round = 1_u8;
    let mut quiz_questions: Vec<QuizQuestion>;
    // Map all users to a valid player id list.
    let player_ids = players.iter().map(|u| u.id.0).collect::<HashSet<u64>>();
    // The primary collector for collecting valid replies (answers).
    // This collector will limit to the game channel, the current guild, and valid players.
    // This means other users who didn't join the game can't reply.
    let mut collector = MessageCollectorBuilder::new(context)
        .channel_id(msg.channel_id.0)
        .guild_id(msg.guild_id.as_ref().expect("Failed to get guild ID.").0)
        .filter(move |m| player_ids.contains(&m.author.id.0))
        .await;

    // Get quiz questions from assets storage and shuffle it.
    let questions = persistence_lock
        .quiz_questions
        .as_ref()
        .expect("Failed to get quiz questions.");
    quiz_questions = questions.to_vec();
    {
        let mut rng = thread_rng();
        quiz_questions.shuffle(&mut rng);
    }
    drop(persistence_lock);
    drop(data);

    while current_round <= max_rounds {
        // Get a question.
        let current_question = quiz_questions.pop();
        // If the program runs out of questions, end the game.
        if current_question.is_none() {
            return Ok(score_board);
        }
        let mut current_question = current_question.expect("Failed to get current question.");
        // Map all answers to lowercase for comparison.
        if current_question._type == "FILL" {
            msg.channel_id
                .say(http, current_question.question.as_str())
                .await?;
            // Only wait for replies for 30 seconds; after that, consider the game stale.
            let delay = tokio::time::sleep(tokio::time::Duration::from_secs(30));
            tokio::pin!(delay);
            // This loop and tokio::select macro will pick the first future that completes.
            // That means either user replies in 30 seconds, or the delay completes.
            loop {
                tokio::select! {
                    _ = &mut delay => {
                        // If the game becomes stale, return an error indicating the game is aborted.
                        msg.channel_id.say(http, "Cancelling stale game...").await?;
                        return Err(CommandError::from("Game is cancelled."));
                    }
                    maybe_v = collector.next() => {
                        if let Some(message) = maybe_v {
                            let response: &str;
                            {
                                let mut rng = thread_rng();
                                response = if is_kou {
                                    KOU_RESPONSES.choose(&mut rng).expect("Failed to choose a random Kou response.")
                                } else {
                                    TAIGA_RESPONSES.choose(&mut rng).expect("Failed to choose a random Taiga response.")
                                }
                            }
                            if current_question.answers
                             .iter()
                             .map(|s| s.to_lowercase())
                             .any(|x| x == message.content.to_lowercase()) {
                                message.channel_id.say(http, format!("{} {}", message.author.mention(), response).as_str()).await?;
                                let score_entry = score_board.entry(message.author.id.0);
                                let score = score_entry.or_default();
                                *score += 1;
                                current_round += 1;
                                break;
                            }
                        }
                    }
                }
            }
        } else if current_question._type == "MULTIPLE" {
            // Clone all wrong answers to mutate them.
            let mut options = current_question.wrong.to_vec();
            // Push the answer to valid options.
            options.push(current_question.answers[0].clone());
            {
                let mut rng = thread_rng();
                options.shuffle(&mut rng);
            }
            // Enumerate the options and push the ordinal to valid answers.
            // This means the user can either type in the full answer text (but why would you do that?)
            // Or just type in the ordinal.
            let options = options
                .into_iter()
                .enumerate()
                .map(|item| {
                    let ordinal = item.0 + 1;
                    if current_question.answers.contains(&item.1) {
                        current_question.answers.push(ordinal.to_string());
                    }
                    return format!("{}) {}", ordinal, item.1);
                })
                .collect::<Vec<String>>();
            // Clone the question and join it with valid choices.
            let mut message = current_question.question.clone();
            message += "\n";
            let joined: String = options.join("\n");
            message += joined.as_str();
            msg.channel_id.say(http, message.as_str()).await?;
            // Again we use branch to select the first concurrent task that completes.
            let delay = tokio::time::sleep(tokio::time::Duration::from_secs(30));
            tokio::pin!(delay);
            loop {
                tokio::select! {
                    _ = &mut delay => {
                        msg.channel_id.say(http, "Cancelling stale game...").await?;
                        return Err(CommandError::from("Game is cancelled."));
                    }
                    maybe_v = collector.next() => {
                        if let Some(v) = maybe_v {
                            if current_question.answers.contains(&v.content) {
                                let response: &str;
                                {
                                    let mut rng = thread_rng();
                                    response = if is_kou {
                                        KOU_RESPONSES.choose(&mut rng).expect("Failed to choose a random Kou response.")
                                    } else {
                                        TAIGA_RESPONSES.choose(&mut rng).expect("Failed to choose a random Taiga response.")
                                    }
                                }
                                v.channel_id.say(http, format!("{} {}", v.author.mention(), response).as_str()).await?;
                                let score_entry = score_board.entry(v.author.id.0);
                                let score = score_entry.or_default();
                                *score += 1;
                                current_round += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(score_board)
}

async fn end_game(
    context: &Context,
    msg: &Message,
    show_scoreboard: bool,
    result: Option<&HashMap<u64, u8>>,
    is_kou: bool,
    color: &Color,
) -> CommandResult {
    let data = context.data.read().await;
    let persistence = data
        .get::<PersistenceService>()
        .expect("Failed to get assets service.");
    let mut persistence_lock = persistence.write().await;
    // Remove the game from ongoing quizzes.
    let ongoing_quizzes = persistence_lock
        .ongoing_quizzes
        .as_mut()
        .expect("Failed to get ongoing quizzes.");
    ongoing_quizzes.remove(&msg.channel_id.0);
    drop(persistence_lock);
    drop(data);

    // Build up the scoreboard.
    if show_scoreboard {
        let score_board = result.expect("Failed to get score board.");
        let guild_id = msg.guild_id.expect("Failed to get guild ID.");
        let guild = context
            .cache
            .guild(guild_id)
            .await
            .expect("Failed to get guild information.");
        let mut score_board = score_board
            .iter()
            .map(|item| {
                (
                    guild
                        .members
                        .get(&UserId(*item.0))
                        .expect("Failed to get member."),
                    *item.1,
                )
            })
            .collect::<Vec<(&Member, u8)>>();
        score_board.sort_by_key(|item| (*item).1);
        score_board.reverse();
        let score_board = score_board
            .into_iter()
            .enumerate()
            .map(|item| {
                let pair = item.1;
                let member = pair.0;
                return format!(
                    "{}) {} with {} points",
                    item.0 + 1,
                    member.mention(),
                    pair.1
                );
            })
            .collect::<Vec<String>>();
        msg.channel_id
            .send_message(&context.http, |m| {
                m.embed(|e| {
                    e.title("Minigame ended!");
                    let mut description = "Total points:\n".to_string();
                    let joined: String = score_board.join("\n");
                    description += joined.as_str();
                    e.description(description);
                    e.thumbnail(if is_kou {
                        "https://cdn.discordapp.com/emojis/717505202651136051.png"
                    } else {
                        "https://cdn.discordapp.com/emojis/706757435553218620.png"
                    });
                    e.color(color.clone());
                    e
                })
            })
            .await?;
    }
    Ok(())
}
