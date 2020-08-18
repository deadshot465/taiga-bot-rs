use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::{Message, ReactionType};
use tokio::time::Duration;
use crate::{PERSISTENCE_STORAGE, INTERFACE_SERVICE};
use serenity::utils::Color;
use chrono::{DateTime, Utc};

enum GameState {
    Starting, InProgress, Ending
}

#[command]
#[description = "Play a fun quiz with your friends. Optionally specify rounds (default 7)."]
#[usage = "10"]
#[example = "10"]
#[bucket = "games"]
pub async fn quiz(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Borrow http for use in subsequent messages.
    let http = &context.http;

    // Acquire ongoing quizzes first so we don't create another game in the same channel.
    unsafe {
        let ongoing_quizzes = PERSISTENCE_STORAGE
            .ongoing_quizzes
            .as_mut()
            .expect("Failed to acquire ongoing quizzes.");

        // Check if there's an existing game.
        if ongoing_quizzes.contains_key(&msg.channel_id.0) {
            // Get game entry.
            let entry = ongoing_quizzes
                .entry(msg.channel_id.0)
                .or_insert(DateTime::from(Utc::now()));
            // Cancel stale games.
            if Utc::now() - *entry > Duration::from_secs(30) {
                msg.channel_id.say(http, "Cancelling stale game...").await?;
            }
            else {
                // Tell user that a game is already running.
                msg.channel_id.say(http, "A game is already running!").await?;
                return Ok(());
            }
        }

        // Check if it's Kou or Taiga, since Taiga's quizzes may contain NSFW contents.
        let is_kou = INTERFACE_SERVICE.is_kou;

        // Check if rounds are specified. If there's any error, sanitize it and set max rounds to 7.
        let max_rounds = args.single::<u8>().unwrap_or(7);
        msg.channel_id.say(http, format!("Starting a game with {} rounds...", max_rounds).as_str())
            .await?;
        let mut current_round = 1_u8;
        let mut game_state = GameState::Starting;

        // Build welcoming messages and allow users to join.
        let mut user_mentions: Vec<String> = vec![];
        for i in (0..11).rev() {
            let description = format!("React below to join the game!\nThis game may contain spoilers{}.\nPlease run `skip` to skip a question.\nCurrent players:{}\n{} seconds left!", if is_kou {
                ""
            } else {
                " or NSFW themes"
            }, user_mentions.join(", "), i);
            let message = build_embed(context, msg, "Minigame Starting!", description.as_str())
                .await;
            message.react(http, ReactionType::Unicode("🇴".to_string())).await
                .expect("Failed to react to joining message.");
            if let Some(reaction) = message
                .await_reaction(&ctx)
                .timeout(Duration::from_secs(i))
                .await {
                let emoji = &reaction.as_inner_ref().emoji;
                match emoji.as_data().as_str() {
                    "🇴" => {
                        
                    }
                }
            }
        }
    }
    Ok(())
}

async fn build_embed(context: &Context, msg: &Message, title: &str, description: &str) -> Message {
    let message = msg.channel_id.send_message(&context.http, |m| m.embed(|e| {
        e.title(title);
        e.description(description);
        e
    })).await.expect("Failed to send embed for quiz.");
    message
}