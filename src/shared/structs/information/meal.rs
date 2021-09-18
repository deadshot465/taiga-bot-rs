use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MealData {
    pub meals: Vec<Meal>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Meal {
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
    pub str_youtube: String,
}
