use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use tokio::time::Duration;
use crate::{PERSISTENCE_STORAGE, INTERFACE_SERVICE};
use std::collections::HashSet;

#[command]
#[description = "Play a hangman game with Kou."]
#[usage = ""]
#[example = ""]
#[bucket = "games"]
pub async fn hangman(context: &Context, msg: &Message) -> CommandResult {
    unsafe {
        if !INTERFACE_SERVICE.is_kou {
            msg.reply(&context.http, "Sorry, this command is currently unavailable.").await?;
            return Ok(());
        }
    }

    // Get member so we can get the user's nickname instead of the user's real name
    let member = context.cache
        .member(msg.guild_id.as_ref().unwrap(), &msg.author.id)
        .await;
    let http = &context.http;

    // Introduce the game
    let nick_name = member.as_ref().unwrap().nick.as_ref();
    let name: &String;
    if let Some(n) = nick_name {
        name = n;
    }
    else {
        name = &msg.author.name;
    }
    msg.reply(http, &format!("Hello {}! We are going to play hangman!", name))
        .await?;

    // Wait for 2 second
    tokio::time::delay_for(Duration::from_secs(2)).await;
    // Set the desired word
    let actual_word: &String;
    unsafe {
        let mut rng = thread_rng();
        actual_word = PERSISTENCE_STORAGE.game_words
            .as_ref()
            .unwrap()
            .choose(&mut rng)
            .unwrap();
    }
    // Set max number of failed attempts to be 10
    let mut attempts: i32 = 10;
    // Tell the length of word
    msg.reply(http, &format!("There are {} letters in this word.", actual_word.len()))
        .await?;
    tokio::time::delay_for(Duration::from_secs(2)).await;
    // First print all letters to be "_"
    let mut message = String::new();
    for _ in actual_word.chars() {
        message += "\\_ ";
    }
    msg.reply(http, &message).await?;
    msg.reply(http, &format!("You have {} attempts left.", attempts)).await?;
    // The guesses are stored in this variable
    let mut guesses = String::new();
    while attempts > 0 {
        let mut input_char: char;
        loop {
            msg.reply(http, "Input a letter:").await?;
            // Ask the user to input letters
            let input = &msg.author.await_reply(&context)
                .timeout(Duration::from_secs(15))
                .await.unwrap();
            input_char = input.content.chars().next().unwrap();
            if input_char.is_alphabetic() && input.content.len() == 1 {
                break;
            }
        }
        // Check every letter position
        let input_char = input_char.to_uppercase().to_string();
        guesses += input_char.as_str();
        let mut failed = 0;
        let mut response = String::new();
        for letter in actual_word.chars() {
            // If the letter matches, print the letter, otherwise print "_"
            if guesses.contains(letter) {
                response.push(letter);
                response.push(' ');
            }
            else {
                response += "\\_ ";
                failed += 1;
            }
        }
        msg.reply(http,&response).await?;

        // If no letter matches, decrease number of attempts by 1
        if !actual_word.contains(&input_char) {
            attempts -= 1;
        }
        // If all letters are printed, that means the player won
        if failed == 0 {
            msg.reply(http,  "You got the correct answer!").await?;
            break;
        }
        else if attempts == 0 {
            // If no more attempts, the player lose
            msg.reply(http,  "You lose!").await?;
            break;
        }

        // Tell the user how many attempts left
        msg.reply(http, &format!("You have {} attempts left.", attempts)).await?;
        // Tell the user his past attempts
        let mut previous_guesses = HashSet::new();
        for letter in guesses.chars() {
            previous_guesses.insert(letter);
        }
        let previous_guesses = previous_guesses.into_iter()
            .map(|c| format!("'{}', ", c))
            .collect::<String>();

        msg.reply(http, &format!("Your previous guesses: {}", &previous_guesses)).await?;
    }
    msg.reply(http, &format!("This answer is {}", &actual_word)).await?;
    tokio::time::delay_for(Duration::from_secs(2)).await;
    Ok(())
}