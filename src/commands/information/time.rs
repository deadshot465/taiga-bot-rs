use chrono::{DateTime, FixedOffset};
use poise::CreateReply;

use crate::shared::structs::information::time::{GeocodeResponse, TimeData, TimezoneResponse};
use crate::shared::structs::{Context, ContextError};

const WORLD_TIME_API_ENDPOINT: &str = "http://worldtimeapi.org/api/timezone/";

/// Query the time of a city based on a city name or an address.
#[poise::command(slash_command, category = "Information")]
pub async fn time(
    ctx: Context<'_>,
    #[description = "A city name or an address of which to query time."]
    city_name_or_address: String,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! One second (pun intended)..."))
        .await?;

    let timezone_name = if let Ok(opt) = search_from_endpoint(ctx, &city_name_or_address).await
        && let Some(s) = opt
    {
        s
    } else {
        search_from_google(ctx, &city_name_or_address).await?
    };

    let response = ctx
        .data()
        .http_client
        .get(format!(
            "http://worldtimeapi.org/api/timezone/{}",
            &timezone_name
        ))
        .send()
        .await;

    match response {
        Ok(res) => match res.json::<TimeData>().await {
            Ok(time_data) => match time_data.datetime.parse::<DateTime<FixedOffset>>() {
                Ok(result) => {
                    reply_handle
                        .edit(
                            ctx,
                            CreateReply::default().content(format!(
                                "The current local time of **{}** is: {}.",
                                timezone_name.replace('_', " "),
                                result.format("%Y-%m-%d %H:%M:%S")
                            )),
                        )
                        .await?;
                }
                Err(e) => {
                    reply_handle
                        .edit(
                            ctx,
                            CreateReply::default()
                                .content(format!("Sorry, an error occurred! Error: {e:?}")),
                        )
                        .await?;
                }
            },
            Err(e) => {
                reply_handle
                    .edit(
                        ctx,
                        CreateReply::default()
                            .content(format!("Sorry, an error occurred! Error: {e:?}")),
                    )
                    .await?;
            }
        },
        Err(e) => {
            reply_handle
                .edit(
                    ctx,
                    CreateReply::default()
                        .content(format!("Sorry, an error occurred! Error: {e:?}")),
                )
                .await?;
        }
    }

    Ok(())
}

async fn search_from_endpoint(ctx: Context<'_>, query: &str) -> anyhow::Result<Option<String>> {
    let city_name = query.replace(' ', "_").to_lowercase();

    let response = ctx
        .data()
        .http_client
        .get(WORLD_TIME_API_ENDPOINT)
        .send()
        .await?;

    let cities: Vec<String> = response.json().await.unwrap_or_default();
    Ok(cities
        .into_iter()
        .find(|s| s.to_lowercase().contains(&city_name)))
}

async fn search_from_google(ctx: Context<'_>, query: &str) -> anyhow::Result<String> {
    let google_api_key = ctx.data().config.google_api_key.as_str();

    let geocode = ctx
        .data()
        .http_client
        .get(format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={query}&key={google_api_key}"
        ))
        .send()
        .await?
        .json::<GeocodeResponse>()
        .await?;

    if let Some(geocode_result) = geocode.results.first() {
        let location = &geocode_result.geometry.location;
        let elapsed_since_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs_f64();
        let timezone = ctx
            .data()
            .http_client
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
