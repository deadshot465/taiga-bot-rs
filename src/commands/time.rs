use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use std::env;
use crate::InterfaceService;
use chrono::prelude::*;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
struct TimeData {
    pub datetime: String,
    pub utc_datetime: String,
    pub timezone: String
}

#[derive(Deserialize, Serialize)]
struct Location {
    pub lat: f64,
    pub lng: f64
}

#[derive(Deserialize, Serialize)]
struct Geometry {
    pub location: Location
}

#[derive(Deserialize, Serialize)]
struct GeocodeResult {
    pub geometry: Geometry
}

#[derive(Deserialize, Serialize)]
struct GeocodeResponse {
    pub results: Vec<GeocodeResult>
}

#[derive(Deserialize, Serialize)]
struct TimezoneResponse {
    #[serde(rename = "dstOffset")]
    pub dst_offset: i32,
    #[serde(rename = "rawOffset")]
    pub raw_offset: i32,
    pub status: String,
    #[serde(rename = "timeZoneId")]
    pub time_zone_id: String,
    #[serde(rename = "timeZoneName")]
    pub time_zone_name: String
}

#[command]
#[aliases("clock")]
#[description = "Query the time of a city."]
#[usage = "<city name>"]
#[example = "Hong Kong"]
#[bucket = "information"]
pub async fn time(context: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = context.data.read().await;
    let interface = data.get::<InterfaceService>().unwrap();
    let _interface = Arc::clone(interface);
    drop(data);
    let interface_lock = _interface.read().await;
    let interface = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface.time;

    if args.is_empty() || args.len() == 0 {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    let client = reqwest::Client::new();
    let city_name = args.remains().unwrap().trim();
    let raw_city_name = String::from(city_name);
    let search_result = search_from_endpoint(city_name, &client).await;
    let mut timezone_name = String::new();

    if search_result.is_none() {
        timezone_name = search_from_google(&raw_city_name, &client).await.unwrap_or_default();
    }
    else {
        timezone_name = search_result.unwrap();
    }

    if timezone_name.is_empty() {
        msg.channel_id.say(&context.http, interface_string.errors["no_result"].as_str()).await?;
        return Ok(());
    }

    let response = client.get(format!("http://worldtimeapi.org/api/timezone/{}", &timezone_name).as_str())
        .send()
        .await?;
    let data: TimeData = response.json().await?;
    let time = data.datetime.as_str().parse::<DateTime<FixedOffset>>();

    if let Ok(t) = time {
        let mut result_msg = interface_string.result.clone();
        result_msg = result_msg
            .replace("{city}", &timezone_name.replace("_", " "))
            .replace("{time}", &t.format("%Y-%m-%d %H:%M:%S").to_string());
        msg.channel_id.say(&context.http, &result_msg).await?;
    }
    else {
        let err = time.unwrap_err();
        msg.channel_id
            .say(&context.http, format!("Parsing time failed: {:?}", err).as_str())
            .await?;
    }
    drop(interface_lock);
    Ok(())
}

async fn search_from_endpoint(city_name: &str, client: &reqwest::Client) -> Option<String> {
    let mut city_name = city_name.replace(" ", "_");
    city_name = city_name.to_lowercase();
    let response = client.get("http://worldtimeapi.org/api/timezone/")
        .send()
        .await
        .expect("Failed to query from the timezone endpoint.");
    let cities: Vec<String> = response.json().await.unwrap_or(vec![]);
    cities.iter()
        .find(|s| {
            let temp = s.to_lowercase();
            temp.contains(&city_name.to_lowercase())
        }).cloned()
}

async fn search_from_google(city_name: &str, client: &reqwest::Client) -> Option<String> {
    let google_api_key = env::var("GOOGLE_API_KEY").unwrap();
    let response = client.get(&format!("https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}", city_name, &google_api_key))
        .send()
        .await
        .expect("Failed to connect to Google's geocode API.");
    let geocode: GeocodeResponse = response.json().await.expect("Failed to translate response to JSON.");
    if geocode.results.is_empty() {
        return None;
    }
    let geocode_result = &geocode.results[0];
    let location = &geocode_result.geometry.location;
    let elapsed_since_epoch = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let response = client
        .get(&format!("https://maps.googleapis.com/maps/api/timezone/json?location={},{}&timestamp={}&key={}", location.lat, location.lng, elapsed_since_epoch, &google_api_key))
        .send()
        .await
        .expect("Failed to connect to Google's timezone API.");
    let time_zone_data: TimezoneResponse = response.json().await.expect("Failed to translate response to JSON.");
    Some(time_zone_data.time_zone_id)
}