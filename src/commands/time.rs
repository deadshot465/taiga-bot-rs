use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::CommandStrings;
use crate::INTERFACE_SERVICE;
use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Deserialize, Serialize)]
struct TimeData {
    pub datetime: String,
    pub utc_datetime: String,
    pub timezone: String
}

#[command]
pub async fn time(context: &Context, msg: &Message, args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.time;
    }

    if args.is_empty() || args.len() == 0 {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    let mut city_name = args.remains().unwrap().trim().replace(" ", "_");
    city_name = city_name.to_lowercase();
    let client = reqwest::Client::new();
    let response = client.get("http://worldtimeapi.org/api/timezone/")
        .send()
        .await?;
    let cities: Vec<String> = response.json().await?;
    let search_result = cities
        .iter()
        .find(|s| {
            let temp = s.to_lowercase();
            temp.contains(&city_name.to_lowercase())
        });

    if let Some(s) = search_result {
        let response = client.get(format!("http://worldtimeapi.org/api/timezone/{}", s).as_str())
            .send()
            .await?;
        let data: TimeData = response.json().await?;
        let time = data.datetime.as_str().parse::<DateTime<FixedOffset>>();

        if let Ok(t) = time {
            let mut result_msg = interface_string.result.clone();
            result_msg = result_msg
                .replace("{city}", &s.replace("_", " "))
                .replace("{time}", &t.format("%Y-%m-%d %H:%M:%S").to_string());
            msg.channel_id.say(&context.http, &result_msg).await?;
            return Ok(());
        }
        else {
            let err = time.unwrap_err();
            msg.channel_id
                .say(&context.http, format!("Parsing time failed: {:?}", err).as_str())
                .await?;
            return Ok(());
        }
    }

    Ok(())
}