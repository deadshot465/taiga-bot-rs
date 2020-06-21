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
pub async fn meal(context: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new();
    let response = client.get("http://www.themealdb.com/api/json/v1/1/random.php")
        .send()
        .await?
        .json()
        .await?;
    let color = u32::from_str_radix("fd9b3b", 16).unwrap();

    Ok(())
}