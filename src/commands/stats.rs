use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::{CommandStrings, UserRecords};
use crate::{INTERFACE_SERVICE, PERSISTENCE_STORAGE};
use serenity::utils::Color;
use crate::commands::valentine::valentine;
use std::borrow::BorrowMut;

#[command]
#[description = "This command will show your records with several commands."]
#[usage = "or stats <command>"]
#[example = "valentine"]
#[bucket = "information"]
pub async fn stats(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let interface_string: &CommandStrings;
    unsafe {
        let ref interface_service = INTERFACE_SERVICE;
        let interface = interface_service.interface_strings.as_ref().unwrap();
        interface_string = &interface.stats;
    }
    let user_id = msg.author.id.0.to_string();

    unsafe {
        let user_records = PERSISTENCE_STORAGE
            .user_records
            .as_mut().unwrap();
        if !user_records.contains_key(&user_id) {
            user_records.insert(user_id.clone(), UserRecords::new());
        }
    }

    let arg = args.single::<String>();
    match arg {
        Ok(s) => {
            let _arg = s.to_lowercase();
            match _arg.as_str() {
                "route" => show_route(context, msg, &user_id).await?,
                "valentine" => show_valentine(context, msg, &user_id).await?,
                "reset" => reset(context, msg, args).await?,
                _ => {
                    let error_msg = interface_string.errors["no_such_command"].as_str();
                    msg.channel_id.say(&context.http, error_msg).await?;
                    return Ok(());
                }
            }
        },
        Err(_) => {
            show_all(context, msg, &user_id).await?;
        }
    }

    Ok(())
}

async fn show_route(context: &Context, msg: &Message, user_id: &str) -> CommandResult {
    unsafe {
        let user_records = PERSISTENCE_STORAGE
            .user_records
            .as_ref()
            .unwrap();
        let route_record = &user_records[user_id].route;
        let mut character_names: Vec<&str> = route_record.iter()
            .map(|r| r.0.as_str())
            .collect();
        character_names.sort();

        let member = msg.member(&context.cache).await.unwrap();
        let user_name = if let Some(s) = member.nick.as_ref() {
            s
        } else {
            &msg.author.name
        };
        let color = u32::from_str_radix("abcdef", 16).unwrap();
        msg.channel_id.send_message(&context.http, |m| m
            .embed(|e| {
                e.author(|a| {
                    if let Some(url) = msg.author.avatar_url() {
                        a.icon_url(&url);
                    }
                    a.name(&user_name)
                })
                    .description(&format!("Here's {}'s records with `route`", &user_name));

                e.color(Color::from(color));

                for name in character_names.iter() {
                    let character_record = &route_record[*name];
                    let mut endings: Vec<&str> = character_record.iter()
                        .map(|r| r.0.as_str())
                        .collect();
                    endings.sort();
                    let result = endings.iter()
                        .map(|s| format!("__{}__: {}\n", *s, character_record[*s]))
                        .collect::<String>();
                    e.field(format!("**{}**", *name).as_str(), &result, true);
                }
                e
            })).await?;
    }
    Ok(())
}

async fn show_valentine(context: &Context, msg: &Message, user_id: &str) -> CommandResult {
    unsafe {
        let user_records = PERSISTENCE_STORAGE
            .user_records
            .as_ref()
            .unwrap();
        let valentine_record = &user_records[user_id].valentine;
        let mut character_names: Vec<_> = valentine_record.iter()
            .map(|r| (r.0.as_str(), *r.1))
            .collect::<Vec<(&str, u16)>>();
        character_names.sort_by(|i, j| (*j).1.cmp(&(*i).1));

        let member = msg.member(&context.cache).await.unwrap();
        let user_name = if let Some(s) = member.nick.as_ref() {
            s
        } else {
            &msg.author.name
        };
        let color = u32::from_str_radix("abcdef", 16).unwrap();
        msg.channel_id.send_message(&context.http, |m| m
            .embed(|e| {
                e.author(|a| {
                    if let Some(url) = msg.author.avatar_url() {
                        a.icon_url(&url);
                    }
                    a.name(&user_name)
                })
                    .description(&format!("Here's {}'s records with `valentine`", &user_name));

                e.color(Color::from(color));

                for name in character_names.iter() {
                    e.field(format!("**{}**", (*name).0).as_str(), (*name).1, true);
                }
                e
            })).await?;
    }
    Ok(())
}

async fn show_all(context: &Context, msg: &Message, user_id: &str) -> CommandResult {
    unsafe {
        let user_records = PERSISTENCE_STORAGE
            .user_records
            .as_ref()
            .unwrap();
        let route_record = &user_records[user_id].route;
        let valentine_record = &user_records[user_id].valentine;
        let mut route_names: Vec<&str> = route_record.iter()
            .map(|r| r.0.as_str())
            .collect();
        let mut valentine_names: Vec<_> = valentine_record.iter()
            .map(|r| (r.0.as_str(), *r.1))
            .collect::<Vec<(&str, u16)>>();
        route_names.sort();
        valentine_names.sort_by(|i, j| (*j).1.cmp(&(*i).1));

        let member = msg.member(&context.cache).await.unwrap();
        let user_name = if let Some(s) = member.nick.as_ref() {
            s
        } else {
            &msg.author.name
        };
        let color = u32::from_str_radix("abcdef", 16).unwrap();
        msg.channel_id.send_message(&context.http, |m| m
            .embed(|e| {
                e.author(|a| {
                    if let Some(url) = msg.author.avatar_url() {
                        a.icon_url(&url);
                    }
                    a.name(&user_name)
                })
                    .description(&format!("Here's {}'s records with `route, valentine`", &user_name));

                e.color(Color::from(color));
                e.field("**Route**", "Records for `route`", false);
                for name in route_names.iter() {
                    let character_record = &route_record[*name];
                    let mut endings: Vec<&str> = character_record.iter()
                        .map(|r| r.0.as_str())
                        .collect();
                    endings.sort();
                    let result = endings.iter()
                        .map(|s| format!("__{}__: {}\n", *s, character_record[*s]))
                        .collect::<String>();
                    e.field(*name, &result, true);
                }
                e.field("**Valentine**", "Records for `valentine`", false);
                for name in valentine_names.iter() {
                    e.field((*name).0, (*name).1, true);
                }

                e
            })).await?;
    }

    Ok(())
}

async fn reset(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let stat_name = args.single::<String>();
    let user_id = msg.author.id.0.to_string();
    unsafe {
        let user_records = PERSISTENCE_STORAGE.user_records
            .as_mut()
            .unwrap();
        if let Ok(stat) = stat_name.as_ref() {
            match stat.as_str() {
                "route" => {
                    let routes = user_records
                        .entry(user_id.clone())
                        .or_insert(UserRecords::new())
                        .route.borrow_mut();
                    routes.clear();
                    msg.channel_id.say(&context.http, "Record cleared.").await?;
                },
                "valentine" => {
                    let valentines = user_records
                        .entry(user_id.clone())
                        .or_insert(UserRecords::new())
                        .valentine.borrow_mut();
                    valentines.clear();
                    msg.channel_id.say(&context.http, "Record cleared.").await?;
                }
                _ => ()
            }
        }
        else {
            let routes = user_records
                .entry(user_id.clone())
                .or_insert(UserRecords::new())
                .route.borrow_mut();
            routes.clear();
            let valentines = user_records
                .entry(user_id.clone())
                .or_insert(UserRecords::new())
                .valentine.borrow_mut();
            valentines.clear();
            msg.channel_id.say(&context.http, "Record cleared.").await?;
        }
    }
    Ok(())
}