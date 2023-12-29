use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::config::configuration::{CONFIGURATION, KOU};
use crate::shared::structs::utility::convert::conversion_table::CONVERSION_TABLE;
use crate::shared::structs::utility::convert::exchange_rate_api_response::ExchangeRateAPIResponse;
use crate::shared::structs::utility::convert::temperature::Temperature;
use crate::shared::structs::utility::convert::ConverterType;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::builder::EditInteractionResponse;
use serenity::model::application::CommandInteraction;
use serenity::prelude::*;
use std::future::Future;
use std::pin::Pin;

const EXCHANGE_RATE_API_BASE_URL: &str = "http://api.exchangeratesapi.io/v1/latest";

pub fn convert_async(
    ctx: Context,
    command: CommandInteraction,
) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + Send>> {
    Box::pin(convert(ctx, command))
}

async fn convert(ctx: Context, command: CommandInteraction) -> anyhow::Result<()> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Alright! One second..."),
            ),
        )
        .await?;

    let source_unit = command
        .data
        .options
        .get(0)
        .map(|opt| &opt.value)
        .and_then(|opt| opt.as_str())
        .unwrap_or_default();

    let target_unit = command
        .data
        .options
        .get(1)
        .map(|opt| &opt.value)
        .and_then(|opt| opt.as_str())
        .unwrap_or_default();

    let amount = command
        .data
        .options
        .get(2)
        .map(|opt| &opt.value)
        .and_then(|opt| opt.as_f64())
        .map(|f| f as f32)
        .unwrap_or_default();

    let converter_type = ConverterType::new(source_unit, target_unit, amount);
    let result = match converter_type {
        ConverterType::Temperature(s, t, n) => compute_temperature(s, t, n),
        ConverterType::Currency(n) => match compute_currency(source_unit, target_unit, n).await {
            Ok(res) => res,
            Err(e) => {
                command
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content(format!(
                            "Sorry, but I can't seem to find the currency you specified! {}",
                            e
                        )),
                    )
                    .await?;
                return Ok(());
            }
        },
        _ => compute_length_or_weight(converter_type),
    };

    let is_kou = KOU.get().copied().unwrap_or(false);

    let source_unit = replace_temperature_sign(source_unit);
    let target_unit = replace_temperature_sign(target_unit);
    command.edit_response(&ctx.http, EditInteractionResponse::new()
        .content(if is_kou {
            format!("<:KouBang:705054435667214367> Alright, that's the best calculation I got! {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        } else {
            format!("<:chibitaiga:697893400891883531> According to Lee's calculations, {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        })).await?;

    Ok(())
}

fn compute_length_or_weight(converter_type: ConverterType) -> f32 {
    match converter_type {
        ConverterType::Length(s, t, n) => {
            let ratio = CONVERSION_TABLE
                .length
                .get(&s.to_string())
                .and_then(|map| map.get(&t.to_string()))
                .copied()
                .unwrap_or(1.0);
            n / ratio
        }
        ConverterType::Weight(s, t, n) => {
            let ratio = CONVERSION_TABLE
                .weight
                .get(&s.to_string())
                .and_then(|map| map.get(&t.to_string()))
                .copied()
                .unwrap_or(1.0);
            n / ratio
        }
        _ => 0.0,
    }
}

fn compute_temperature(source: Temperature, target: Temperature, amount: f32) -> f32 {
    let ratio = CONVERSION_TABLE
        .temperature
        .get(&source.to_string())
        .and_then(|map| map.get(&target.to_string()))
        .copied()
        .unwrap_or(1.0);

    match source {
        Temperature::Celsius => {
            let adjustment: f32 = match target {
                Temperature::Fahrenheit => 32.0,
                Temperature::Kelvin => 273.15,
                _ => 0.0,
            };
            (amount / ratio) + adjustment
        }
        Temperature::Fahrenheit => {
            let adjustment: f32 = match target {
                Temperature::Celsius => -32.0,
                Temperature::Kelvin => 459.67,
                _ => 0.0,
            };
            (amount + adjustment) / ratio
        }
        Temperature::Kelvin => {
            let adjustment = match target {
                Temperature::Celsius => -273.15,
                Temperature::Fahrenheit => -459.67,
                _ => 0.0,
            };
            (amount / ratio) + adjustment
        }
    }
}

async fn compute_currency(source: &str, target: &str, amount: f32) -> anyhow::Result<f32> {
    let token = CONFIGURATION
        .get()
        .map(|c| c.exchange_rate_api_key.as_str())
        .unwrap_or_default();

    let source = source.to_uppercase();
    let target = target.to_uppercase();

    let url = format!(
        "{}?access_key={}&symbols={},{}",
        EXCHANGE_RATE_API_BASE_URL, token, &source, &target
    );

    let response: ExchangeRateAPIResponse = HTTP_CLIENT.get(&url).send().await?.json().await?;

    if !response.rates.contains_key(&source) || !response.rates.contains_key(&target) {
        Err(anyhow::anyhow!(
            "Failed to find corresponding currency type."
        ))
    } else {
        Ok(response
            .rates
            .get(&target)
            .copied()
            .map(|t| t / response.rates.get(&source).copied().unwrap_or(1.0))
            .map(|rate| amount * rate)
            .unwrap_or_default())
    }
}

fn replace_temperature_sign(original: &str) -> &str {
    match original {
        "c" => "℃",
        "f" => "℉",
        "k" => "K",
        _ => original,
    }
}
