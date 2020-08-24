use serenity::prelude::*;
use serenity::model::{
    prelude::*,
    channel::Message
};
use serenity::framework::standard::{macros::{
    command
}, CommandResult, CommandError};
use crate::PersistenceService;
use serenity::utils::Color;
use chrono::{Utc, Duration};
use serenity::collector::MessageCollectorBuilder;
use serenity::futures::StreamExt;
use std::sync::Arc;

enum TicTacToeResult {
    GameNotOver, CircleWin, CrossWin, Draw
}

const EMPTY_SLOT: &'static str = "□";
const CIRCLE: &'static str = "○";
const CROSS: &'static str = "×";

#[command]
#[description = "Play a tic-tac-toe game with another person. This command has to be prefixed with `games`."]
#[usage = ""]
#[example = ""]
#[bucket = "games"]
async fn tictactoe(context: &Context, msg: &Message) -> CommandResult {
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    drop(data);
    let persistence_lock = _persistence.read().await;

    let http = &context.http;

    let ongoing_tictactoes = persistence_lock
        .ongoing_tictactoes
        .as_ref()
        .expect("Failed to acquire ongoing tic-tac-toes.");
    if ongoing_tictactoes.contains(&msg.channel_id.0) {
        msg.channel_id.say(http, "A game is already running!").await?;
        return Ok(());
    }
    drop(persistence_lock);

    let color_value = u32::from_str_radix("306998", 16).unwrap();
    let color = Color::new(color_value);
    let (game_started, players) = join_game(context, msg, color)
        .await;
    // If game starts, wait for game result.
    if game_started {
        let _ = progress(context, msg, players.unwrap(), color).await;
    }
    end_game(context, msg).await;
    Ok(())
}

/// Handles player joining.
async fn join_game(context: &Context, msg: &Message, color: Color) -> (bool, Option<Vec<User>>) {
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let mut persistence_lock = persistence.write().await;
    // Add the current channel to ongoing quizzes.
    let ongoing_tictactoes = persistence_lock
        .ongoing_tictactoes
        .as_mut()
        .unwrap();
    let _ = ongoing_tictactoes.insert(msg.channel_id.0);
    drop(persistence_lock);
    drop(data);

    let http = &context.http;
    // Build welcoming messages and allow users to join.
    let mut users: Vec<User> = vec![];
    // Setting up the time limit for joining in the game.
    let end_joining_time = Utc::now() + Duration::seconds(10);
    // Setting up the initial embed so we will have an embed to edit.
    let mut description = format!("React below to start a tic-tac-toe!\nYou need 2 players (including yourself) to play this game.\nCurrent players:{}\n{} seconds left!", "", (end_joining_time - Utc::now()).num_seconds());
    let mut message = msg.channel_id.send_message(http, |m| m.embed(|e| {
        e.title("Tic-tac-toe Starting!");
        e.description(&description);
        e.color(color);
        e
    })).await.expect("Failed to send embed for tic-tac-toe.");
    // Initial reaction made by Kou, to denote the correct emoji that should be used.
    message.react(http, ReactionType::Unicode("✅".to_string())).await
        .expect("Failed to react to joining message.");
    // Loop until 10 seconds have already passed.
    loop {
        // Get user mentions from participating players.
        let user_mentions = users.iter()
            .map(|u| u.mention())
            .collect::<Vec<String>>();
        description = format!("React below to start a tic-tac-toe!\nYou need 2 players (including yourself) to play this game.\nCurrent players:{}\n{} seconds left!", user_mentions.join(", "), (end_joining_time - Utc::now()).num_seconds());
        // Edit the message to show current participants.
        message.edit(http, |m| m.embed(|e| {
            e.color(color);
            e.title("Tic-tac-toe Starting!");
            e.description(&description);
            e
        })).await.expect("Failed to edit the embed message.");
        // Collect added and removed reactions.
        if let Some(reaction) = message
            .await_reaction(&context)
            .timeout(tokio::time::Duration::from_secs(2))
            .removed(true)
            .await {
            let emoji = &reaction.as_inner_ref().emoji;
            // Pattern matching to get the reaction we need.
            match emoji.as_data().as_str() {
                "✅" => {
                    let user: User = reaction.as_inner_ref().user(http).await.expect("Failed to retrieve the user.");
                    if !user.bot {
                        users.push(user);
                    }
                }
                _ => ()
            };
        }
        // If there are already 2 players who joined,
        // Break the loop and enter the next game stage.
        if users.len() >= 2 {
            break;
        }
        // If 10 seconds have already passed,
        // Also break the loop and enter the next game stage.
        if Utc::now() > end_joining_time {
            break;
        }
    }

    // If nobody joins, cancel and unregister the game by returning false and None.
    if users.len() < 2 {
        message.edit(http, |m| m.embed(|e| {
            e.color(color.clone());
            e.title("Tic-tac-toe Cancelled!");
            e.description("Nobody joined...");
            e.thumbnail("https://cdn.discordapp.com/emojis/736061517534855201.png");
            e
        })).await.expect("Failed to send game failed message.");
        (false, None)
    }
    else {
        // Otherwise starts the game by returning true and valid player list.
        message.edit(http, |m| m.embed(|e| {
            e.color(color);
            e.title("Tic-tac-toe Started!");
            e.description("The game has begun!");
            e.thumbnail("https://cdn.discordapp.com/emojis/705182851754360912.png");
            e
        })).await.expect("Failed to send game started message.");
        (true, Some(users))
    }
}

async fn draw_board(context: &Context, msg: &Message, color: Color, players: &Vec<User>, board: &Vec<Vec<&str>>, edit: bool, message: Option<&mut Message>) -> std::result::Result<Option<Message>, CommandError> {
    let mut description = "  1 2 3\n".to_string();
    for y in 0..3 {
        let mut ordinal = (y + 1).to_string();
        ordinal += " ";
        description += ordinal.as_str();
        for x in 0..3 {
            let mut slot = format!("{}", board[y][x]);
            if x != 2 {
                slot += "|";
            }
            description += slot.as_str();
        }
        if y != 2 {
            description += "\n";
        }
    }
    if !edit {
        let _msg = msg.channel_id.send_message(&context.http, |m| m.embed(|e| {
            e.color(color);
            e.title("Current Board");
            let _description = format!("First: {}, Second: {}\n```{}```", players[0].mention(), players[1].mention(), &description);
            e.description(&_description);
            e.footer(|f| f.text("Tic-tac-toe original Python version made by: @Kirito#9286"));
            e
        })).await?;
        Ok(Some(_msg))
    }
    else {
        message.unwrap().edit(&context.http, |m| m.embed(|e| {
            e.color(color);
            e.title("Current Board");
            let _description = format!("First: {}, Second: {}\n```{}```", players[0].mention(), players[1].mention(), &description);
            e.description(&_description);
            e.footer(|f| f.text("Tic-tac-toe original Python version made by: @Kirito#9286"));
            e
        })).await?;
        Ok(None)
    }
}

/// The main game loop.
async fn progress(context: &Context, msg: &Message, players: Vec<User>, color: Color) -> std::result::Result<bool, CommandError> {
    let http = &context.http;
    // Fill the board with empty squares.
    let mut board = vec![];
    for y in 0..3 {
        board.push(vec![]);
        for _ in 0..3 {
            board[y].push(EMPTY_SLOT);
        }
    }
    let message = draw_board(context, msg, color, &players, &board, false, None::<&mut Message>).await?;
    if message.is_none() {
        let err_msg = msg.channel_id.say(http, "There is an error when creating the board! Game is aborted.").await?;
        tokio::time::delay_for(tokio::time::Duration::from_secs(3)).await;
        err_msg.delete(http).await.expect("Failed to delete message.");
        return Err(CommandError::from("Failed to create game board."));
    }
    let mut message = message.unwrap();
    let mut round = 0;
    let _players = players.to_vec();
    let mut player1_collector = MessageCollectorBuilder::new(&context)
        .channel_id(msg.channel_id.0)
        .guild_id(msg.guild_id.as_ref().unwrap().0)
        .filter(move |m| m.author.id.0 == _players[0].id.0)
        .await;
    let _players = players.to_vec();
    let mut player2_collector = MessageCollectorBuilder::new(&context)
        .channel_id(msg.channel_id.0)
        .guild_id(msg.guild_id.as_ref().unwrap().0)
        .filter(move |m| m.author.id.0 == _players[1].id.0)
        .await;
    loop {
        let current_player = &players[round % 2];
        let announcement = msg.channel_id.say(http, format!("{}'s turn!", current_player.mention())).await?;
        let sign = if round % 2 == 0 {
            CIRCLE
        } else {
            CROSS
        };
        let y_msg = msg
            .channel_id.say(http, format!("{}, please input the position in rows (1-3):", current_player.mention())).await?;
        let mut delay = tokio::time::delay_for(tokio::time::Duration::from_secs(30));
        let collector = if round % 2 == 0 {
            &mut player1_collector
        } else {
            &mut player2_collector
        };
        let mut y = 0_u8;
        loop {
            tokio::select! {
                _ = &mut delay => {
                    msg.channel_id.say(http, "Game is stale. Cancelling the game...").await?;
                    return Err(CommandError::from("Game is stale."));
                }
                v = collector.next() => {
                    if let Some(reply) = v {
                        let choice = reply.content.parse::<u8>();
                        if choice.is_ok() {
                            let number: u8 = choice.unwrap_or_default();
                            if number >= 1 && number <= 3 {
                                y = number;
                                y_msg.delete(http).await.expect("Failed to delete message.");
                                reply.delete(http).await.expect("Failed to delete message.");
                                break;
                            }
                        }
                    }
                }
            }
        }
        let x_msg = msg
            .channel_id.say(http, format!("{}, please input the position in columns (1-3):", current_player.mention())).await?;
        let mut x = 0_u8;
        let mut delay = tokio::time::delay_for(tokio::time::Duration::from_secs(30));
        loop {
            tokio::select! {
                _ = &mut delay => {
                    msg.channel_id.say(http, "Game is stale. Cancelling the game...").await?;
                    return Err(CommandError::from("Game is stale."));
                }
                v = collector.next() => {
                    if let Some(reply) = v {
                        let choice = reply.content.parse::<u8>();
                        if choice.is_ok() {
                            let number: u8 = choice.unwrap_or_default();
                            if number >= 1 && number <= 3 {
                                x = number;
                                x_msg.delete(http).await.expect("Failed to delete message.");
                                reply.delete(http).await.expect("Failed to delete message.");
                                break;
                            }
                        }
                    }
                }
            }
        }
        if board[(y - 1) as usize][(x - 1) as usize] != "□" {
            let err_msg = msg.channel_id.say(http, "The slot you selected is occupied!")
                .await?;
            tokio::time::delay_for(tokio::time::Duration::from_secs_f32(1.5)).await;
            err_msg.delete(http).await.expect("Failed to delete message.");
            continue;
        }
        announcement.delete(http).await.expect("Failed to delete the message.");
        board[(y - 1) as usize][(x - 1) as usize] = sign;
        let _ = draw_board(context, msg, color, &players, &board, true, Some(&mut message)).await?;
        match check_result(&board) {
            TicTacToeResult::CircleWin => {
                msg.channel_id.say(http, &format!("{} won the game!", players[0].mention())).await?;
                return Ok(true);
            },
            TicTacToeResult::CrossWin => {
                msg.channel_id.say(http, &format!("{} won the game!", players[1].mention())).await?;
                return Ok(true);
            },
            TicTacToeResult::Draw => {
                msg.channel_id.say(http, "It's a draw!").await?;
                return Ok(true);
            },
            _ => ()
        }
        round += 1;
    }
}

fn check_mark_is_equal(a: &str, b: &str, c: &str) -> bool {
    if a == EMPTY_SLOT || b == EMPTY_SLOT || c == EMPTY_SLOT {
        return false;
    }
    if a != b {
        return false;
    }
    if b != c {
        return false;
    }
    if a != c {
        return false;
    }
    true
}

fn check_result(board: &Vec<Vec<&str>>) -> TicTacToeResult {
    for y in 0..3 {
        if check_mark_is_equal(board[y][0], board[y][1], board[y][2]) {
            if board[y][0] == CIRCLE {
                return TicTacToeResult::CircleWin;
            }
            else if board[y][0] == CROSS {
                return TicTacToeResult::CrossWin;
            }
        }
        if check_mark_is_equal(board[0][y], board[1][y], board[2][y]) {
            if board[0][y] == CIRCLE {
                return TicTacToeResult::CircleWin;
            }
            else if board[0][y] == CROSS {
                return TicTacToeResult::CrossWin;
            }
        }
    }

    if check_mark_is_equal(board[2][0], board[1][1], board[0][2]) {
        if board[2][0] == CIRCLE {
            return TicTacToeResult::CircleWin;
        }
        else if board[2][0] == CROSS {
            return TicTacToeResult::CrossWin;
        }
    }

    if check_mark_is_equal(board[0][0], board[1][1], board[2][2]) {
        if board[0][0] == CIRCLE {
            return TicTacToeResult::CircleWin;
        }
        else if board[0][0] == CROSS {
            return TicTacToeResult::CrossWin;
        }
    }

    for y in 0..3 {
        for x in 0..3 {
            if board[y][x] == EMPTY_SLOT {
                return TicTacToeResult::GameNotOver;
            }
        }
    }
    TicTacToeResult::Draw
}

async fn end_game(context: &Context, msg: &Message) {
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let mut persistence_lock = persistence.write().await;
    let ongoing_tictactoes = persistence_lock
        .ongoing_tictactoes
        .as_mut()
        .unwrap();
    ongoing_tictactoes.remove(&msg.channel_id.0);
    drop(persistence_lock);
    drop(data);
}