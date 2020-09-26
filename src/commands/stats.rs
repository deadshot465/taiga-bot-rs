use serenity::framework::standard::{macros::{
    command
}, CommandResult, Args};
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::shared::UserRecords;
use crate::{InterfaceService, PersistenceService, PersistenceStorage};
use serenity::utils::Color;
use std::borrow::BorrowMut;
use std::sync::Arc;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};

#[command]
#[description = "This command will show your records with several commands."]
#[usage = "or stats <command>"]
#[example = "valentine"]
#[bucket = "information"]
pub async fn stats(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>().unwrap();
    let interface = lock.get::<InterfaceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    let _interface = Arc::clone(interface);
    drop(lock);
    let mut persistence_lock = _persistence.write().await;
    let interface_lock = _interface.read().await;

    let interface_strings = interface_lock.interface_strings.as_ref().unwrap();
    let interface_string = &interface_strings.stats;
    let user_id = msg.author.id.0.to_string();

    let user_records = persistence_lock
        .user_records
        .as_mut().unwrap();
    if !user_records.contains_key(&user_id) {
        user_records.insert(user_id.clone(), UserRecords::new());
    }
    drop(persistence_lock);

    let arg = args.single::<String>();
    match arg {
        Ok(s) => {
            let _arg = s.to_lowercase();
            match _arg.as_str() {
                "route" => {
                    let persistence_lock = _persistence.read().await;
                    show_route(context, msg, &user_id, &persistence_lock).await?;
                    drop(persistence_lock);
                },
                "valentine" => {
                    let persistence_lock = _persistence.read().await;
                    show_valentine(context, msg, &user_id, &persistence_lock).await?;
                    drop(persistence_lock);
                },
                "reply" => {
                    let persistence_lock = _persistence.read().await;
                    show_replies(context, msg, &user_id, &persistence_lock).await?;
                    drop(persistence_lock);
                },
                "reset" => {
                    let mut persistence_lock = _persistence.write().await;
                    reset(context, msg, args, &mut persistence_lock).await?;
                    drop(persistence_lock);
                },
                _ => {
                    let error_msg = interface_string.errors["no_such_command"].as_str();
                    msg.channel_id.say(&context.http, error_msg).await?;
                    return Ok(());
                }
            }
        },
        Err(_) => {
            let persistence_lock = _persistence.read().await;
            show_all(context, msg, &user_id, &persistence_lock).await?;
            drop(persistence_lock);
        }
    }
    drop(interface_lock);
    Ok(())
}

async fn show_route(context: &Context, msg: &Message, user_id: &str, persistence: &RwLockReadGuard<'_, PersistenceStorage>) -> CommandResult {
    let user_records = persistence
        .user_records
        .as_ref()
        .unwrap();
    let route_record = &user_records[user_id].route;
    let mut character_names: Vec<&str> = route_record.iter()
        .map(|r| r.0.as_str())
        .collect();
    character_names.sort();

    let member = msg.member(&context.http).await.unwrap();
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
    Ok(())
}

async fn show_valentine(context: &Context, msg: &Message, user_id: &str, persistence: &RwLockReadGuard<'_, PersistenceStorage>) -> CommandResult {
    let user_records = persistence
        .user_records
        .as_ref()
        .unwrap();
    let valentine_record = &user_records[user_id].valentine;
    let mut character_names: Vec<_> = valentine_record.iter()
        .map(|r| (r.0.as_str(), *r.1))
        .collect::<Vec<(&str, u16)>>();
    character_names.sort_by(|i, j| (*j).1.cmp(&(*i).1));

    let member = msg.member(&context.http).await.unwrap();
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
    Ok(())
}

async fn show_replies(context: &Context, msg: &Message, user_id: &str, persistence: &RwLockReadGuard<'_, PersistenceStorage>) -> CommandResult {
    let user_records = persistence
        .user_records
        .as_ref()
        .unwrap();
    let replies = &user_records[user_id].replies;
    let member = msg.member(&context.http).await.unwrap();
    let user_name = if let Some(s) = member.nick.as_ref() {
        s
    } else {
        &msg.author.name
    };
    let color = u32::from_str_radix("abcdef", 16).unwrap();
    msg.channel_id.send_message(&context.http, |m| m
        .embed(|e| e
            .author(|a| {
                if let Some(url) = msg.author.avatar_url() {
                    a.icon_url(&url);
                }
                a.name(&user_name)
            })
            .description(&format!("Here's {}'s records with `reply`", &user_name))
            .color(Color::from(color))
            .field("Special Replies", replies, false)))
        .await?;
    Ok(())
}

async fn show_all(context: &Context, msg: &Message, user_id: &str, persistence: &RwLockReadGuard<'_, PersistenceStorage>) -> CommandResult {
    let user_records = persistence
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

    let member = msg.member(&context.http).await.unwrap();
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
    Ok(())
}

async fn reset(context: &Context, msg: &Message, mut args: Args, persistence: &mut RwLockWriteGuard<'_, PersistenceStorage>) -> CommandResult {
    let stat_name = args.single::<String>();
    let user_id = msg.author.id.0.to_string();
    let user_records = persistence.user_records
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
            },
            "reply" => {
                let replies = user_records
                    .entry(user_id.clone())
                    .or_insert(UserRecords::new())
                    .replies.borrow_mut();
                *replies = 0;
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
        let replies = user_records
            .entry(user_id.clone())
            .or_insert(UserRecords::new())
            .replies.borrow_mut();
        *replies = 0;
        msg.channel_id.say(&context.http, "Record cleared.").await?;
    }
    Ok(())
}