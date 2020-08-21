use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::{
        command
    }
};
use crate::{Reminder, InterfaceService, PersistenceService};
use chrono::{DateTime, Duration, Local, TimeZone, ParseResult};
use std::sync::Arc;

const PREPOSITIONS: [&'static str; 2] = [
    "in", "on"
];

const UNITS: [&'static str; 6] = [
    "years", "months", "days", "hours", "minutes", "seconds"
];

#[command]
#[description = "Make Kou remind you in a specific time."]
#[usage = "in <amount> <unit> <message> to remind you after a period of time has passed. Run remind on <date> <message> to remind you on 0:00AM of a specific date. The format for a date is: `YYYY-MM-DD, YYYY/MM/DD, YYYY.MM.DD`."]
#[example = "in 1 <years|months|days|hours|minutes|seconds> or remind on 2020-07-30."]
#[bucket = "utilities"]
pub async fn remind(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let lock = context.data.read().await;
    let interface = lock.get::<InterfaceService>().unwrap();
    let persistence = lock.get::<PersistenceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    let _interface = Arc::clone(interface);
    drop(lock);
    let interface_lock = _interface.lock().await;
    let interface_strings = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface_strings.remind;

    if args.is_empty() || args.len() <= 1 {
        let error_msg = interface_string.errors["length_too_short"].as_str();
        msg.channel_id.say(&context.http, error_msg).await?;
        return Ok(());
    }

    let prep = args.single::<String>().unwrap();
    let units: String = UNITS.iter()
        .map(|s| format!("{}, ", s))
        .collect();
    if !PREPOSITIONS.contains(&prep.as_str()) {
        let error_msg = interface_string.errors["no_such_operation"].replace("{units}", &units[..units.len() - 2]);
        msg.channel_id.say(&context.http, &error_msg).await?;
        return Ok(());
    }

    match prep.as_str() {
        "in" => {
            let amount = args.single::<u32>();
            let unit = args.single::<String>();
            let message = args.remains();
            if let Err(_) = &amount {
                msg.channel_id.say(&context.http, "The amount of time you inputted is either incorrect, too big, or too small.")
                    .await?;
                return Ok(());
            }

            if let Err(_) = &unit {
                msg.channel_id.say(&context.http, format!("The unit you inputted is incorrect. Available units are: `{}`", &units[..units.len() - 2]).as_str())
                    .await?;
                return Ok(());
            }
            else if let Ok(u) = &unit {
                if !UNITS.contains(&u.as_str()) {
                    msg.channel_id.say(&context.http, format!("The unit you inputted is incorrect. Available units are: `{}`", &units[..units.len() - 2]).as_str())
                        .await?;
                    return Ok(());
                }
            }

            if message.is_none() {
                let error_msg = interface_string.errors["no_message"].as_str();
                msg.channel_id.say(&context.http, error_msg).await?;
                return Ok(());
            }
            let mut persistence_lock = _persistence.lock().await;
            let reminders = persistence_lock.reminders.as_mut().unwrap();
            let entry = reminders.entry(msg.author.id.0).or_insert(Reminder::new());
            match unit.as_ref().unwrap().as_str() {
                "years" => {
                    (*entry).datetime = Local::now() + Duration::days(365_i64 * (amount.unwrap() as i64));
                },
                "months" => {
                    (*entry).datetime = Local::now() + Duration::days(30_i64 * (amount.unwrap() as i64));
                },
                "days" => {
                    (*entry).datetime = Local::now() + Duration::days(amount.unwrap() as i64);
                },
                "hours" => {
                    (*entry).datetime = Local::now() + Duration::hours(amount.unwrap() as i64);
                },
                "minutes" => {
                    (*entry).datetime = Local::now() + Duration::minutes(amount.unwrap() as i64);
                },
                "seconds" => {
                    (*entry).datetime = Local::now() + Duration::seconds(amount.unwrap() as i64);
                },
                _ => ()
            }
            (*entry).message = message.unwrap().to_string();
            let result_msg = interface_string.result
                .replace("{time}", &Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
                .replace("{dueTime}", &(*entry).datetime.format("%Y-%m-%d %H:%M:%S").to_string());
            msg.channel_id.say(&context.http, &result_msg).await?;
            persistence_lock.write();
            drop(persistence_lock);
            return Ok(());
        },
        "on" => {
            let datetime = args.single::<String>();
            if let Err(_) = datetime {
                msg.channel_id.say(&context.http, "The inputted date and time is either incorrect or missing.")
                    .await?;
                return Ok(());
            }
            let sanitized = datetime.unwrap() + "00:00:00";
            let formats: [&str; 3] = [
                "%Y-%m-%d %H:%M:%S",
                "%Y/%m/%d %H:%M:%S",
                "%Y.%m.%d %H:%M:%S"
            ];
            let mut datetime: ParseResult<DateTime<Local>> = Ok(Local::now());
            for f in formats.iter() {
                datetime = Local.datetime_from_str(sanitized.as_str(), *f);
                if datetime.is_ok() {
                    break;
                }
            }
            if let Err(_) = datetime {
                msg.channel_id.say(&context.http, "The inputted format of the date and time is incorrect.")
                    .await?;
                return Ok(());
            }
            else if let Ok(dt) = datetime.as_ref() {
                if dt < &Local::now() {
                    let error_msg = interface_string.errors["past_time"].as_str();
                    msg.channel_id.say(&context.http, error_msg)
                        .await?;
                    return Ok(());
                }
            }

            let message = args.remains();
            if message.is_none() {
                let error_msg = interface_string.errors["no_message"].as_str();
                msg.channel_id.say(&context.http, error_msg).await?;
                return Ok(());
            }
            let mut persistence_lock = _persistence.lock().await;
            let reminders = persistence_lock.reminders.as_mut().unwrap();
            let entry = reminders.entry(msg.author.id.0).or_insert(Reminder::new());
            (*entry).datetime = datetime.unwrap();
            (*entry).message = message.unwrap().to_string();
            let result_msg = interface_string.result
                .replace("{time}", &Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
                .replace("{dueTime}", &(*entry).datetime.format("%Y-%m-%d %H:%M:%S").to_string());
            msg.channel_id.say(&context.http, &result_msg).await?;
            persistence_lock.write();
            drop(persistence_lock);
            return Ok(());
        }
        _ => ()
    }
    drop(interface_lock);
    Ok(())
}