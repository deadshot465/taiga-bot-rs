use crate::shared::structs::game::hangman_question::HANGMAN_QUESTIONS;
use crate::shared::utility::{get_author_avatar, get_author_name};
use rand::prelude::*;
use serenity::all::{
    Color, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse,
};
use serenity::builder::CreateEmbed;
use serenity::futures::prelude::future::BoxFuture;
use serenity::model::application::CommandInteraction;
use serenity::model::prelude::User;
use serenity::prelude::*;
use serenity::FutureExt;
use std::future::Future;
use std::pin::Pin;

const HANGMAN_COLOR: Color = Color::new(0xffd43b);
const DEFAULT_MAX_ATTEMPTS: i32 = 10;
const HANGMAN_THUMBNAIL: &str =
    "https://cdn.discordapp.com/attachments/700003813981028433/736202279983513671/unnamed.png";
const HANGMAN_FOOTER: &str = "Hangman original Python version made by: @Kirito#9286";
const PROMPT_MESSAGE: &str = "input a letter:";
const INPUT_ERROR_MESSAGE: &str = "the answer has to be an English letter!";
const WIN_MESSAGE: &str = "you got the correct answer!";
const LOSE_MESSAGE: &str = "you lose!";

#[derive(Clone)]
struct HangmanData {
    ctx: Context,
    command: CommandInteraction,
    answer: String,
    author_name: String,
    author_avatar_url: String,
    guesses: Vec<char>,
    attempts_remained: i32,
    user: User,
    failures: i32,
}

#[derive(Copy, Clone, Debug)]
enum HangmanResult {
    Win,
    Lose,
    Aborted,
}

pub fn hangman_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(hangman(ctx, command))
}

async fn hangman(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    let member = command.member.clone().map(|m| *m.clone());
    let author_name = get_author_name(&command.user, &member);
    let author_avatar_url = get_author_avatar(&command.user);

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                format!("Hello {}! We are going to play hangman!", &author_name),
            )),
        )
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let answer = {
        let mut rng = rand::thread_rng();
        HANGMAN_QUESTIONS
            .choose(&mut rng)
            .map(|s| s.as_str())
            .unwrap_or_default()
    };

    command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().content(format!(
                "There are {} letters in this word.",
                answer.chars().count()
            )),
        )
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let word: String = answer.chars().map(|_| "\\_").collect::<Vec<_>>().join(" ");

    command
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new().embed(
                CreateEmbed::new()
                    .author(CreateEmbedAuthor::new(&author_name).icon_url(&author_avatar_url))
                    .description(format!("You have {} attempts left.", DEFAULT_MAX_ATTEMPTS))
                    .color(HANGMAN_COLOR)
                    .title(&word)
                    .thumbnail(HANGMAN_THUMBNAIL)
                    .footer(CreateEmbedFooter::new(HANGMAN_FOOTER)),
            ),
        )
        .await?;

    let user = command.user.clone();
    let hangman_data = HangmanData {
        ctx,
        command,
        answer: answer.to_string(),
        author_name,
        author_avatar_url,
        guesses: vec![],
        attempts_remained: DEFAULT_MAX_ATTEMPTS,
        user,
        failures: 0,
    };
    let hangman_data_clone = hangman_data.clone();
    tokio::spawn(async move {
        match hangman_loop(hangman_data).await {
            Ok(game_result) => match game_result {
                HangmanResult::Win => {
                    if let Err(e) = hangman_data_clone
                        .command
                        .create_followup(
                            &hangman_data_clone.ctx.http,
                            CreateInteractionResponseFollowup::new().content(format!(
                                "{}, {}\nThe answer is **{}**!",
                                &hangman_data_clone.author_name,
                                WIN_MESSAGE,
                                &hangman_data_clone.answer
                            )),
                        )
                        .await
                    {
                        tracing::error!("{}", e);
                    }
                }
                HangmanResult::Lose => {
                    if let Err(e) = hangman_data_clone
                        .command
                        .create_followup(
                            &hangman_data_clone.ctx.http,
                            CreateInteractionResponseFollowup::new().content(format!(
                                "{}, {}\nThe answer is **{}**!",
                                &hangman_data_clone.author_name,
                                LOSE_MESSAGE,
                                &hangman_data_clone.answer
                            )),
                        )
                        .await
                    {
                        tracing::error!("{}", e);
                    }
                }
                HangmanResult::Aborted => {
                    if let Err(e) = hangman_data_clone
                        .command
                        .create_followup(
                            &hangman_data_clone.ctx.http,
                            CreateInteractionResponseFollowup::new().content(format!(
                                "No input from {} is provided. Game aborted.",
                                &hangman_data_clone.author_name
                            )),
                        )
                        .await
                    {
                        tracing::error!("{}", e);
                    }
                }
            },
            Err(e) => {
                tracing::error!("An error occurred during a hangman game: {}", e);
            }
        }
    });

    Ok(())
}

fn hangman_loop(
    mut hangman_data: HangmanData,
) -> BoxFuture<'static, anyhow::Result<HangmanResult>> {
    async move {
        hangman_data.failures = 0;

        let sent_msg = hangman_data
            .command
            .create_followup(
                &hangman_data.ctx.http,
                CreateInteractionResponseFollowup::new()
                    .content(format!("{}, {}", &hangman_data.author_name, PROMPT_MESSAGE)),
            )
            .await?;

        let user_guess: char;
        loop {
            if let Some(user_reply) = hangman_data
                .user
                .await_reply(&hangman_data.ctx)
                .timeout(std::time::Duration::from_secs(60))
                .await
            {
                if let Some(char) = user_reply.content.chars().next() {
                    if char.is_ascii_alphabetic() {
                        user_guess = char.to_ascii_uppercase();
                        user_reply.delete(&hangman_data.ctx.http).await?;
                        break;
                    }

                    user_reply.delete(&hangman_data.ctx.http).await?;
                    hangman_data
                        .command
                        .edit_followup(
                            &hangman_data.ctx.http,
                            sent_msg.id,
                            CreateInteractionResponseFollowup::new().content(format!(
                                "{}, {}",
                                &hangman_data.author_name, INPUT_ERROR_MESSAGE
                            )),
                        )
                        .await?;
                    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                    hangman_data
                        .command
                        .edit_followup(
                            &hangman_data.ctx.http,
                            sent_msg.id,
                            CreateInteractionResponseFollowup::new().content(format!(
                                "{}, {}",
                                &hangman_data.author_name, PROMPT_MESSAGE
                            )),
                        )
                        .await?;
                }
            } else {
                hangman_data
                    .command
                    .delete_followup(&hangman_data.ctx.http, sent_msg.id)
                    .await?;
                return Ok(HangmanResult::Aborted);
            }
        }

        hangman_data.guesses.push(user_guess);
        hangman_data.guesses.dedup();
        hangman_data.guesses.sort_unstable();

        let word: String = hangman_data
            .answer
            .clone()
            .chars()
            .map(|c| {
                if hangman_data.guesses.contains(&c.to_ascii_uppercase()) {
                    c.to_ascii_uppercase().to_string()
                } else {
                    hangman_data.failures += 1;
                    "\\_".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        if !hangman_data.answer.contains(user_guess) {
            hangman_data.attempts_remained -= 1;
        }

        let previous_guesses: String = hangman_data
            .guesses
            .clone()
            .into_iter()
            .map(|c| format!("'{}'", c))
            .collect::<Vec<_>>()
            .join(", ");

        hangman_data
            .command
            .edit_followup(
                &hangman_data.ctx.http,
                sent_msg.id,
                CreateInteractionResponseFollowup::new().content("").embed(
                    CreateEmbed::new()
                        .author(
                            CreateEmbedAuthor::new(&hangman_data.author_name)
                                .icon_url(&hangman_data.author_avatar_url),
                        )
                        .description(format!(
                            "You have {} attempts left.\nYour previous guesses: {}.",
                            hangman_data.attempts_remained, previous_guesses
                        ))
                        .color(HANGMAN_COLOR)
                        .title(word)
                        .thumbnail(HANGMAN_THUMBNAIL)
                        .footer(CreateEmbedFooter::new(HANGMAN_FOOTER)),
                ),
            )
            .await?;

        if hangman_data.failures == 0 {
            return Ok(HangmanResult::Win);
        } else if hangman_data.attempts_remained == 0 {
            return Ok(HangmanResult::Lose);
        }

        hangman_loop(hangman_data).await
    }
    .boxed()
}
