use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::information::meal::MealData;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Color;
use std::future::Future;
use std::pin::Pin;

const ENDPOINT: &str = "http://www.themealdb.com/api/json/v1/1/random.php";

pub fn meal_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(meal(ctx, command))
}

async fn meal(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| data.content("Alright! One moment..."))
        })
        .await?;

    let meal_data: MealData = HTTP_CLIENT.get(ENDPOINT).send().await?.json().await?;

    if let Some(meal_data) = meal_data.meals.get(0) {
        command
            .edit_original_interaction_response(&ctx.http, |response| {
                response.content("").embed(|embed| {
                    embed
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
                        .footer(|f| f.text("Bon Appétit! Powered by TheMealDB.com."))
                })
            })
            .await?;
    } else {
        command
            .edit_original_interaction_response(&ctx.http, |response| {
                response
                    .content("Sorry, I can't seem to find any recipe for you for the time being!")
            })
            .await?;
    }

    Ok(())
}
