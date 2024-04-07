use poise::CreateReply;

use crate::shared::structs::utility::convert::exchange_rate_api_response::ExchangeRateAPIResponse;
use crate::shared::structs::utility::convert::length::Length;
use crate::shared::structs::utility::convert::temperature::Temperature;
use crate::shared::structs::utility::convert::weight::Weight;
use crate::shared::structs::utility::convert::ConverterType;
use crate::shared::structs::{Context, ContextError};

const EXCHANGE_RATE_API_BASE_URL: &str = "http://api.exchangeratesapi.io/v1/latest";

/// Helps to convert stuff.
#[poise::command(
    slash_command,
    subcommands("length", "weight", "temperature", "currency"),
    subcommand_required,
    category = "Utility"
)]
pub async fn convert(_: Context<'_>) -> Result<(), ContextError> {
    Ok(())
}

/// Convert length.
#[poise::command(slash_command)]
pub async fn length(
    ctx: Context<'_>,
    #[description = "The source length to convert from."] source_unit: Length,
    #[description = "The target length to convert to."] target_unit: Length,
    #[description = "The amount to convert."] amount: f32,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! One second..."))
        .await?;

    let converter_type = ConverterType::Length(source_unit, target_unit, amount);
    let result = compute_length_or_weight(ctx, converter_type);

    let is_kou = ctx.data().kou;

    reply_handle
        .edit(ctx, CreateReply::default().content(if is_kou {
            format!("<:KouBang:705054435667214367> Alright, that's the best calculation I got! {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        } else {
            format!("<:chibitaiga:697893400891883531> According to Lee's calculations, {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        })).await?;

    Ok(())
}

/// Convert weight.
#[poise::command(slash_command)]
pub async fn weight(
    ctx: Context<'_>,
    #[description = "The source weight to convert from."] source_unit: Weight,
    #[description = "The target weight to convert to."] target_unit: Weight,
    #[description = "The amount to convert."] amount: f32,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! One second..."))
        .await?;

    let converter_type = ConverterType::Weight(source_unit, target_unit, amount);
    let result = compute_length_or_weight(ctx, converter_type);

    let is_kou = ctx.data().kou;

    reply_handle
        .edit(ctx, CreateReply::default().content(if is_kou {
            format!("<:KouBang:705054435667214367> Alright, that's the best calculation I got! {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        } else {
            format!("<:chibitaiga:697893400891883531> According to Lee's calculations, {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        })).await?;

    Ok(())
}

/// Convert temperature.
#[poise::command(slash_command)]
pub async fn temperature(
    ctx: Context<'_>,
    #[description = "The source temperature to convert from."] source_unit: Temperature,
    #[description = "The target temperature to convert to."] target_unit: Temperature,
    #[description = "The amount to convert."] amount: f32,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! One second..."))
        .await?;

    let result = compute_temperature(ctx, source_unit, target_unit, amount);
    let is_kou = ctx.data().kou;

    let source_unit = replace_temperature_sign(source_unit);
    let target_unit = replace_temperature_sign(target_unit);

    reply_handle
        .edit(ctx, CreateReply::default().content(if is_kou {
            format!("<:KouBang:705054435667214367> Alright, that's the best calculation I got! {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        } else {
            format!("<:chibitaiga:697893400891883531> According to Lee's calculations, {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        })).await?;

    Ok(())
}

/// Convert currency.
#[poise::command(slash_command)]
pub async fn currency(
    ctx: Context<'_>,
    #[description = "The source currency type to convert from, e.g. USD."] source_unit: String,
    #[description = "The target currency type to convert to, e.g. JPY."] target_unit: String,
    #[description = "The amount to convert."] amount: f32,
) -> Result<(), ContextError> {
    let reply_handle = ctx
        .send(CreateReply::default().content("Alright! One second..."))
        .await?;

    let result = compute_currency(ctx, &source_unit, &target_unit, amount).await?;
    let is_kou = ctx.data().kou;

    reply_handle
        .edit(ctx, CreateReply::default().content(if is_kou {
            format!("<:KouBang:705054435667214367> Alright, that's the best calculation I got! {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        } else {
            format!("<:chibitaiga:697893400891883531> According to Lee's calculations, {}{} is {}{}.",
                    amount, source_unit, (result * 100.0).round() / 100.0, target_unit)
        })).await?;

    Ok(())
}

fn compute_length_or_weight(ctx: Context<'_>, converter_type: ConverterType) -> f32 {
    match converter_type {
        ConverterType::Length(s, t, n) => {
            let ratio = ctx
                .data()
                .conversion_table
                .length
                .get(&s.to_string())
                .and_then(|map| map.get(&t.to_string()))
                .copied()
                .unwrap_or(1.0);
            n / ratio
        }
        ConverterType::Weight(s, t, n) => {
            let ratio = ctx
                .data()
                .conversion_table
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

fn compute_temperature(
    ctx: Context<'_>,
    source: Temperature,
    target: Temperature,
    amount: f32,
) -> f32 {
    let ratio = ctx
        .data()
        .conversion_table
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

async fn compute_currency(
    ctx: Context<'_>,
    source: &str,
    target: &str,
    amount: f32,
) -> anyhow::Result<f32> {
    let token = ctx.data().config.exchange_rate_api_key.as_str();

    let source = source.to_uppercase();
    let target = target.to_uppercase();

    let url = format!(
        "{}?access_key={}&symbols={},{}",
        EXCHANGE_RATE_API_BASE_URL, token, &source, &target
    );

    let response: ExchangeRateAPIResponse = ctx
        .data()
        .http_client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

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

fn replace_temperature_sign(original: Temperature) -> &'static str {
    match original {
        Temperature::Celsius => "℃",
        Temperature::Fahrenheit => "℉",
        Temperature::Kelvin => "K",
    }
}
