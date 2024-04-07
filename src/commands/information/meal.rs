use poise::CreateReply;
use serenity::all::{Color, CreateEmbedFooter};
use serenity::builder::CreateEmbed;

use crate::shared::structs::information::meal::MealData;
use crate::shared::structs::{Context, ContextError};

const ENDPOINT: &str = "http://www.themealdb.com/api/json/v1/1/random.php";

/// Get a random meal recipe.
#[poise::command(slash_command, category = "Information")]
pub async fn meal(ctx: Context<'_>) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! One moment..."))
        .await?;

    let meal_data: MealData = ctx
        .data()
        .http_client
        .get(ENDPOINT)
        .send()
        .await?
        .json()
        .await?;

    if let Some(meal_data) = meal_data.meals.get(0) {
        reply_handle
            .edit(
                ctx,
                CreateReply::default().embed(
                    CreateEmbed::new()
                        .color(Color::new(0xfd9b3b))
                        .description(if meal_data.str_instructions.len() >= 1900 {
                            &meal_data.str_instructions[0..1900]
                        } else {
                            &meal_data.str_instructions
                        })
                        .title(&meal_data.str_meal)
                        .image(&meal_data.str_meal_thumb)
                        .url(&meal_data.str_source)
                        .field("Category", &meal_data.str_category, true)
                        .field("Area", &meal_data.str_area, true)
                        .field("YouTube Video", &meal_data.str_youtube, true)
                        .footer(CreateEmbedFooter::new(
                            "Bon Appétit! Powered by TheMealDB.com.",
                        )),
                ),
            )
            .await?;
    } else {
        reply_handle
            .edit(
                ctx,
                CreateReply::default()
                    .content("Sorry, I can't seem to find any recipe for you for the time being!"),
            )
            .await?;
    }

    Ok(())
}
