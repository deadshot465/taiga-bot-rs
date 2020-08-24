use rand::prelude::*;
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use tokio::time::Duration;
use crate::PersistenceService;
use serenity::utils::Color;

#[command]
#[description = "Play a hangman game with Kou. This command has to be prefixed with `games`."]
#[usage = ""]
#[example = ""]
#[bucket = "games"]
pub async fn hangman(context: &Context, msg: &Message) -> CommandResult {
    // Get member so we can get the user's nickname instead of the user's real name
    let member = context.cache
        .member(msg.guild_id.as_ref().unwrap(), &msg.author.id)
        .await;
    let http = &context.http;

    // Get user's avatar in advance
    let url = msg.author.avatar_url();

    // Construct the color of embed in advance
    let color = u32::from_str_radix("ffd43b", 16).unwrap();
    let color = Color::from(color);

    // Introduce the game
    let nick_name = member.as_ref().unwrap().nick.as_ref();
    let name: &String;
    if let Some(n) = nick_name {
        name = n;
    } else {
        name = &msg.author.name;
    }
    msg.reply(http, &format!("Hello {}! We are going to play hangman!", name))
        .await?;

    // Wait for 2 second
    tokio::time::delay_for(Duration::from_secs(2)).await;

    // Set the desired word
    let data = context.data.read().await;
    let persistence = data.get::<PersistenceService>().unwrap();
    let persistence_lock = persistence.read().await;
    let actual_word: String;
    {
        let mut rng = thread_rng();
        actual_word = persistence_lock.game_words
            .as_ref()
            .unwrap()
            .choose(&mut rng)
            .unwrap()
            .clone();
    }
    drop(persistence_lock);
    drop(data);

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
    let mut embed = msg.channel_id.send_message(http, |m| m.embed(|e| {
        e.author(|a| {
            if let Some(u) = &url {
                a.icon_url(u);
            }
            a.name(name)
        });
        e.color(color);
        e.description(format!("You have {} attempts left.", attempts));
        e.title(&message);
        e.thumbnail("https://cdn.discordapp.com/attachments/700003813981028433/736202279983513671/unnamed.png");
        e.footer(|f| f.text("Hangman original Python version made by: @Kirito#9286"))
    })).await?;

    // The guesses are stored in this variable
    let mut guesses = String::new();

    // The main game loop
    while attempts > 0 {
        let mut input_char: char;
        loop {
            let notice = msg.reply(http, "Input a letter:").await?;

            // Ask the user to input letters
            let input = &msg.author.await_reply(&context)
                .timeout(Duration::from_secs(15))
                .await;
            // Check if the user replies. If not, abort the game.
            if input.is_none() {
                msg.reply(http, "No input is provided. Game aborted.").await?;
                notice.delete(http).await?;
                return Ok(());
            }
            let input_result = input.as_ref().unwrap();
            input_char = input_result.content.chars().next().unwrap();
            // If the reply contains more than one character, or is not an alphabet, ask the user for the reply again.
            if input_char.is_alphabetic() && input_result.content.len() == 1 {
                input_result.delete(http).await?;
                notice.delete(http).await?;
                break;
            }
            input_result.delete(http).await?;
            notice.delete(http).await?;
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

        // If no letter matches, decrease number of attempts by 1
        if !actual_word.contains(&input_char) {
            attempts -= 1;
        }

        // Tell the user his past attempts
        let mut previous_guesses: Vec<char> = vec![];
        for letter in guesses.chars() {
            previous_guesses.push(letter);
        }
        previous_guesses.sort();
        previous_guesses.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
        let previous_guesses = previous_guesses.into_iter()
            .map(|c| format!("'{}', ", c))
            .collect::<String>();

        // Show the current progress, tell the user how many attempts left, and tell the user his past attempts.
        embed.edit(http, |m| m.embed(|e| {
            e.author(|a| {
                if let Some(u) = &url {
                    a.icon_url(u);
                }
                a.name(name)
            });
            e.color(color);
            e.description(format!("You have {} attempts left.\n{}", attempts, &previous_guesses));
            e.title(&response);
            e.thumbnail("https://cdn.discordapp.com/attachments/700003813981028433/736202279983513671/unnamed.png");
            e.footer(|f| f.text("Hangman original Python version made by: @Kirito#9286"))
        })).await?;

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
    }
    msg.reply(http, &format!("The answer is {}", &actual_word)).await?;
    Ok(())
}