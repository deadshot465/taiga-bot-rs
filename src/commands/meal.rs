use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::{
    command
}, CommandResult};
use serenity::prelude::Context;
use serenity::model::channel::Message;

#[derive(Deserialize, Serialize)]
struct MealData {
    pub meals: Vec<Meal>
}

#[derive(Deserialize, Serialize)]
struct Meal {
    #[serde(rename = "strMeal")]
    pub str_meal: String,
    #[serde(rename = "strInstructions")]
    pub str_instructions: String,
    #[serde(rename = "strMealThumb")]
    pub str_meal_thumb: String,
    #[serde(rename = "strSource")]
    pub str_source: String,
    #[serde(rename = "strCategory")]
    pub str_category: String,
    #[serde(rename = "strArea")]
    pub str_area: String,
    #[serde(rename = "strYoutube")]
    pub str_youtube: String
}

#[command]
#[aliases("food")]
#[description = "Get a random meal recipe."]
#[usage = ""]
#[only_in("guilds")]
#[example = ""]
#[bucket = "information"]
pub async fn meal(context: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let response = client.get("http://www.themealdb.com/api/json/v1/1/random.php")
        .send()
        .await?;
    let data: MealData = response.json().await?;
    let meal = &data.meals[0];
    let color = u32::from_str_radix("fd9b3b", 16).unwrap();

    msg.channel_id.say(&context.http, "Here you go!").await?;
    msg.channel_id.send_message(&context.http, |m| m.embed(|e| e
        .color(color)
        .description(if meal.str_instructions.len() >= 2048 {
            &meal.str_instructions[0..2047]
        }
        else {
            &meal.str_instructions
        })
        .title(&meal.str_meal)
        .image(&meal.str_meal_thumb)
        .url(&meal.str_source)
        .field("Category", &meal.str_category, true)
        .field("Area", &meal.str_area, true)
        .field("YouTube Video", &meal.str_youtube, true)
        .footer(|f| f.text("Bon Appétit! Powered by TheMealDB.com."))))
        .await?;

    Ok(())
}