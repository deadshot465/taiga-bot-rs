use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::{CommandStrings, ConversionTable};
use crate::{INTERFACE_SERVICE, PERSISTENCE_STORAGE};

const VALID_UNITS: [&'static str; 10] = [
    "km", "m", "cm", "in", "ft", "mi", "au", "c", "f", "k"
];

const VALID_LENGTHS: [&'static str; 7] = [
    "km", "m", "cm", "in", "ft", "mi", "au"
];

const VALID_TEMPERATURES: [&'static str; 3] = [
    "c", "f", "k"
];

lazy_static! {
    static ref CVT_REGEX: Regex = Regex::new(r"(-?[0-9.]+)(\D{1,2})").unwrap();
}

#[command]
pub async fn cvt(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    let prefix: &String;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.cvt;
        prefix = &interface_service.prefix;
    }

    let raw_target_unit = args.single::<String>();
    let raw_source_string = args.single::<String>();
    let mut target_unit = String::new();
    let mut source_string = String::new();

    if raw_target_unit.is_err() || raw_source_string.is_err() {
        let temps: String = VALID_TEMPERATURES.iter().map(|f| {
            f.to_string() + ", "
        }).collect();
        let lengths: String = VALID_LENGTHS.iter().map(|f| {
            f.to_string() + ", "
        }).collect();

        let error_msg = interface_string.errors["length_too_short"]
            .replace("{temps}", &temps[..temps.len() - 2])
            .replace("{heights}", &lengths[..lengths.len() - 2])
            .replace("{prefix}", &prefix);
        msg.channel_id.say(&context.http, &error_msg).await?;
        return Ok(());
    }
    else {
        target_unit = raw_target_unit.unwrap().to_lowercase();
        source_string = raw_source_string.unwrap().to_lowercase();
    }

    if !VALID_UNITS.contains(&target_unit.as_str()) {
        let units: String = VALID_UNITS.iter()
            .map(|f| {
                f.to_string() + ", "
            }).collect();

        let error_msg = interface_string.errors["invalid_unit"]
            .replace("{units}", &units[..units.len() - 2]);

        msg.channel_id.say(&context.http, &error_msg).await?;
        return Ok(());
    }

    let cvt_regex: &Regex = &*CVT_REGEX;
    if !cvt_regex.is_match(&source_string) {
        let error_msg = interface_string.errors["wrong_pattern"].as_str()
            .replace("{input}", &source_string);
        msg.channel_id.say(&context.http, &error_msg).await?;
        return Ok(());
    }

    let mut amount = cvt_regex.captures(&source_string)
        .unwrap()
        .get(1)
        .unwrap().as_str().parse::<f64>().unwrap();
    let source_unit = cvt_regex.captures(&source_string)
        .unwrap()
        .get(2)
        .unwrap().as_str();
    if (VALID_LENGTHS.contains(&target_unit.as_str()) && VALID_TEMPERATURES.contains(&source_unit)) ||
        (VALID_TEMPERATURES.contains(&target_unit.as_str()) && VALID_LENGTHS.contains(&source_unit)) {
        let error_msg = interface_string.errors["operation_not_possible"].as_str();
        msg.channel_id.say(&context.http, &error_msg).await?;
        return Ok(());
    }

    let conversion_table: &ConversionTable;
    unsafe {
        conversion_table = &PERSISTENCE_STORAGE.conversion_table.as_ref().unwrap();
    }

    let mut result = 0_f64;
    match target_unit.as_str() {
        "c" => {
            if source_unit == "f" {
                amount -= 32_f64;
            }
            else if source_unit == "k" {
                amount -= 273.15_f64;
            }
            result = conversion_table.temperature["c"][source_unit] * amount;
        },
        "f" => {
            result = conversion_table.temperature["f"][source_unit] * amount;
            if source_unit == "c" {
                result += 32_f64;
            }
            else if source_unit == "k" {
                result -= 459.67_f64;
            }
        },
        "k" => {
            if source_unit == "c" {
                amount += 273.15_f64;
            }
            else if source_unit == "f" {
                amount += 459.67_f64;
            }
            result = conversion_table.temperature["k"][source_unit] * amount;
        }
        _ => {
            result = conversion_table.length[&target_unit][source_unit] * amount;
        }
    }

    result = ((result * 100000_f64).round()) / 100000_f64;

    let result_message = interface_string.result
        .replace("{source}", &source_string)
        .replace("{amount}", &result.to_string())
        .replace("{target}", &target_unit);
    msg.channel_id.say(&context.http, &result_message).await?;
    Ok(())
}