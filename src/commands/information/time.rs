use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::config::configuration::CONFIGURATION;
use crate::shared::structs::information::time::{GeocodeResponse, TimeData, TimezoneResponse};
use crate::shared::utility::extract_string_option;
use chrono::{DateTime, FixedOffset};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

const WORLD_TIME_API_ENDPOINT: &str = "http://worldtimeapi.org/api/timezone/";

pub fn time_async(
    ctx: Context,
    command: ApplicationCommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(time(ctx, command))
}

async fn time(ctx: Context, command: ApplicationCommandInteraction) -> anyhow::Result<()> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.content("Alright! One second (pun intended)...")
            })
        })
        .await?;

    let city = extract_string_option(&command, 0);

    let timezone_name = match search_from_endpoint(city).await? {
        Some(s) => s,
        None => search_from_google(city).await?,
    };

    let time_data = HTTP_CLIENT
        .get(format!(
            "http://worldtimeapi.org/api/timezone/{}",
            &timezone_name
        ))
        .send()
        .await?
        .json::<TimeData>()
        .await?;

    match time_data.datetime.parse::<DateTime<FixedOffset>>() {
        Ok(result) => {
            command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(format!(
                        "The current local time of **{}** is: {}.",
                        timezone_name.replace("_", " "),
                        result.format("%Y-%m-%d %H:%M:%S").to_string()
                    ))
                })
                .await?;
        }
        Err(e) => {
            command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(format!("Sorry, an error occurred! Error: {}", e))
                })
                .await?;
        }
    }
    Ok(())
}

async fn search_from_endpoint(query: &str) -> anyhow::Result<Option<String>> {
    let city_name = query.replace(" ", "_").to_lowercase();

    let response = HTTP_CLIENT.get(WORLD_TIME_API_ENDPOINT).send().await?;

    let cities: Vec<String> = response.json().await.unwrap_or_default();
    Ok(cities
        .into_iter()
        .find(|s| s.to_lowercase().contains(&city_name)))
}

async fn search_from_google(query: &str) -> anyhow::Result<String> {
    let google_api_key = CONFIGURATION
        .get()
        .map(|c| c.google_api_key.as_str())
        .unwrap_or_default();

    let geocode = HTTP_CLIENT
        .get(format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            query, google_api_key
        ))
        .send()
        .await?
        .json::<GeocodeResponse>()
        .await?;

    if let Some(geocode_result) = geocode.results.get(0) {
        let location = &geocode_result.geometry.location;
        let elapsed_since_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs_f64();
        let timezone = HTTP_CLIENT
            .get(format!(
                "https://maps.googleapis.com/maps/api/timezone/json?location={},{}&timestamp={}&key={}",
                location.lat, location.lng, elapsed_since_epoch, google_api_key
            ))
            .send()
            .await?
            .json::<TimezoneResponse>()
            .await?;
        Ok(timezone.time_zone_id)
    } else {
        Err(anyhow::anyhow!(
            "Failed to find or deserialize geocode response."
        ))
    }
}
