use crate::commands::game::hangman::hangman;
use crate::commands::game::quiz::quiz;
use crate::shared::structs::{Context, ContextError};

pub mod hangman;
pub mod quiz;

/// Play mini games with Kou/Taiga.
#[poise::command(
    slash_command,
    subcommands("quiz", "hangman"),
    subcommand_required,
    category = "Game"
)]
pub async fn game(_: Context<'_>) -> Result<(), ContextError> {
    Ok(())
}
