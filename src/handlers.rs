use log::{info};
use rand::{
    prelude::*,
    Rng,
    thread_rng
};
use regex::Regex;
use serenity::{
    async_trait,
    framework::standard::macros::{
        hook
    },
    model::prelude::*,
    prelude::*
};
use crate::{get_image, get_dialog, UserRecords, Emote, PersistenceService, InterfaceService, CommandGroupCollection, search_user};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::env;
use chrono::{Utc, Local, Duration};
use serenity::framework::standard::DispatchError;
use std::sync::Arc;
use crate::commands::guide::build_embed;
use serenity::utils::Color;

const ADMIN_COMMANDS: [&'static str; 7] = [
    "allow", "cvt", "convert", "disable", "enable", "ignore", "purge"
];

const KOU_SERVER_ID: u64 = 705036924330704968_u64;

lazy_static::lazy_static! {
    static ref ANIMATED_REGEX: Regex = Regex::new(r"(<a)").unwrap();
    static ref EMOTE_REGEX: Regex = Regex::new(r"<a?:(\w+):(\d+)>").unwrap();
}

pub struct Handler;

fn hit_or_miss(chance: u8) -> bool {
    thread_rng().gen_range(0_u8, 100_u8) < chance
}

async fn handle_self_mentions(context: &Context, msg: &Message) {
    let bot_id = env::var("BOT_ID").unwrap();
    let mention_reaction_chance: u8 = env::var("MENTION_REACTION_CHANCE")
        .unwrap()
        .parse::<u8>().unwrap();
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>()
        .expect("Failed to retrieve persistence service.");
    let interface = lock.get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    let persistence_lock = persistence.read().await;
    let random_messages = persistence_lock.random_messages.as_ref().unwrap();
    let messages = random_messages.iter()
        .find(|m| {
            if is_kou {
                m.keyword.as_str() == "kou"
            }
            else {
                m.keyword.as_str() == "taiga"
            }
        }).unwrap().clone();
    drop(persistence_lock);
    drop(lock);
    // Randomly replies to messages that mention the bot.
    if msg.content.contains(&bot_id) && hit_or_miss(mention_reaction_chance) {
        let english_msgs = &messages.messages["en"];
        let index = thread_rng().gen_range(0, english_msgs.len());
        msg.channel_id.say(&context.http, english_msgs[index].as_str())
            .await
            .expect("Failed to reply to mention.");
    }
}

async fn handle_reactions(context: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }
    let reaction_chance: u8 = env::var("REACTION_CHANCE")
        .unwrap()
        .parse::<u8>().unwrap();
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>()
        .expect("Failed to retrieve persistence service.");
    let interface = lock.get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let _persistence = Arc::clone(persistence);
    let persistence_lock = _persistence.read().await;
    let random_messages = persistence_lock.random_messages.as_ref().unwrap();
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(lock);
    // Randomly reacts to messages that contains certain keywords.
    if hit_or_miss(reaction_chance) {
        for m in random_messages.iter() {
            let lower_case = msg.content.to_lowercase();
            if !lower_case.contains(m.keyword.as_str()) {
                continue;
            }
            if m.keyword.as_str() == "lee" && lower_case.contains("sleep") {
                continue;
            }
            if m.keyword.as_str() != "kou" && is_kou {
                continue;
            }
            if m.keyword.as_str() == "kou" && lower_case.contains("mikkou") {
                continue;
            }
            let index = thread_rng().gen_range(0, m.reactions.len());
            let reaction = m.reactions[index].as_str();
            let emote_regex = &*EMOTE_REGEX;
            let animated_regex = &*ANIMATED_REGEX;
            if emote_regex.is_match(reaction) {
                let animated = animated_regex.is_match(reaction);
                let captures = emote_regex.captures(reaction).unwrap();
                let emote_name = captures.get(1).unwrap().as_str();
                let emote_id = captures.get(2).unwrap().as_str().parse::<u64>().unwrap();
                let reaction_type = ReactionType::Custom {
                    animated,
                    id: EmojiId(emote_id),
                    name: Some(emote_name.to_string())
                };
                let res = msg.react(&context.http, reaction_type).await;
                if let Err(e) = res {
                    eprintln!("Failed to react: {}", e.to_string());
                }
            }
            else {
                let reaction_type = ReactionType::Unicode(reaction.to_string());
                msg.react(&context.http, reaction_type).await.expect("Failed to react.");
            }
        }
    }
    drop(persistence_lock);
}

async fn handle_user_replies(context: &Context, msg: &Message) {
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>()
        .expect("Failed to retrieve persistence service.");
    let interface = lock.get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let _persistence = Arc::clone(persistence);
    let persistence_lock = _persistence.read().await;
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(lock);
    if persistence_lock.channel_settings.as_ref().unwrap()
        .ignored_channels.contains(&msg.channel_id.0) {
        drop(persistence_lock);
        return;
    }
    if msg.author.bot {
        return;
    }
    if is_kou {
        return;
    }
    let user_replies = persistence_lock.user_replies.as_ref().unwrap();
    let user_ids = user_replies.iter()
        .map(|r| r.user)
        .collect::<Vec<u64>>();
    if !user_ids.contains(&msg.author.id.0) {
        return;
    }

    let reply_user_chance: u8 = env::var("RDM_REPLY_USER_CHANCE")
        .unwrap()
        .parse::<u8>().unwrap();
    if hit_or_miss(reply_user_chance) {
        let messages = user_replies.iter()
            .find_map(|r| {
                if r.user == msg.author.id.0 {
                    Some(&r.messages)
                }
                else {
                    None
                }
            }).unwrap();
        if msg.author.id.0 == 677249244842950684 {
            let average_probability = 1_f64 / (messages.len() as f64);
            let specialized_chance = (average_probability / 2_f64).floor();
            let reply = messages.iter()
                .find(|s| s.contains("You know what") && s.contains("moderate"))
                .unwrap();
            let hit = hit_or_miss(specialized_chance as u8);
            if hit {
                msg.reply(&context.http, reply)
                    .await
                    .expect("Failed to reply to the user.");
            }
            else {
                let _messages = messages.iter()
                    .filter(|s| !s.starts_with("You know what"))
                    .collect::<Vec<&String>>();
                let index = thread_rng().gen_range(0, _messages.len());
                msg.reply(&context.http, _messages[index])
                    .await
                    .expect("Failed to reply to the user.");
            }
        }
        else {
            let index = thread_rng().gen_range(0, messages.len());
            msg.reply(&context.http, messages[index].as_str())
                .await
                .expect("Failed to reply to the user.");
        }
        drop(persistence_lock);
        let mut persistence_lock = _persistence.write().await;
        let user_id = msg.author.id.0.to_string();
        let user_records = persistence_lock.user_records
            .as_mut()
            .unwrap()
            .entry(user_id)
            .or_insert(UserRecords::new());
        let reply_count = &mut user_records.replies;
        *reply_count += 1;
        drop(persistence_lock);
    }
    else {
        drop(persistence_lock);
    }
}

async fn handle_replies(context: &Context, msg: &Message) {
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>()
        .expect("Failed to retrieve persistence service.");
    let interface = lock.get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let _persistence = Arc::clone(persistence);
    let persistence_lock = _persistence.read().await;
    let interface_lock = interface.read().await;
    let is_kou = interface_lock.is_kou;
    drop(interface_lock);
    drop(lock);
    if persistence_lock.channel_settings.as_ref().unwrap()
        .ignored_channels.contains(&msg.channel_id.0) {
        return;
    }
    if msg.author.bot {
        return;
    }
    let lower_case = msg.content.to_lowercase();
    let all_messages = persistence_lock.random_messages.as_ref().unwrap();
    let random_reply_chance: u8 = env::var("RDM_REPLY_CHANCE")
        .unwrap()
        .parse::<u8>().unwrap();

    let should_reply = all_messages.iter()
        .any(|m| lower_case.contains(m.keyword.as_str()));
    if !should_reply {
        return;
    }

    let should_reply = hit_or_miss(random_reply_chance);
    if !should_reply {
        return;
    }

    if is_kou {
        if lower_case.contains("kou") && !lower_case.contains("mikkou") {
            let messages = all_messages
                .iter()
                .find(|m| m.keyword.as_str() == "kou")
                .unwrap();
            let english_msgs = &messages.messages["en"];
            let index = thread_rng().gen_range(0, english_msgs.len());
            msg.channel_id.say(&context.http, english_msgs[index].as_str())
                .await
                .expect("Failed to perform random reply.");
            return;
        }
    }
    else {
        let specialized_reply_chance: u8 = env::var("SPECIALIZED_CHANCE")
            .unwrap()
            .parse::<u8>().unwrap();
        if hit_or_miss(specialized_reply_chance) {
            let backgrounds = persistence_lock.dialog_backgrounds
                .as_ref()
                .unwrap();
            let index = thread_rng().gen_range(0, backgrounds.len());
            let background = backgrounds[index].as_str();
            if lower_case.contains("hiro") {
                let character = "taiga";
                let text = "Hiro will be terribly wrong if he thinks he can steal Keitaro from me!";
                let bytes = get_dialog(background, character, text, context).await.unwrap();
                let files: Vec<(&[u8], &str)> = vec![(bytes.borrow(), "result.png")];
                msg.channel_id.send_files(&context.http, files, |m| m.content(""))
                    .await
                    .expect("Failed to send specialized reply for Hiro.");

                return;
            }
            else if lower_case.contains("aiden")
            {
                let bytes = get_image("hamburger").await.unwrap();
                let files: Vec<(&[u8], &str)> = vec![(bytes.borrow(), "result.png")];
                msg.channel_id
                    .say(&context.http, "Three orders of double-quarter-pounder cheeseburgers! Two large fries and one large soda!\nBurger patties well-done, three slices of pickles for each! No mayonnaise! Just ketchup and mustard!")
                    .await
                    .expect("Failed to send specialized reply for Aiden.");
                msg.channel_id.send_files(&context.http, files, |m| m.content(""))
                    .await
                    .expect("Failed to send specialized photo for Aiden.");
                return;
            }
        }
        else {
            let mut shuffled_messages = (*all_messages).to_vec();
            {
                let mut rng = thread_rng();
                shuffled_messages.shuffle(&mut rng);
            }
            for message in shuffled_messages.iter() {
                if !lower_case.contains(&message.keyword) {
                    continue;
                }
                if message.keyword.as_str() == "lee" && lower_case.contains("sleep") {
                    continue;
                }
                if message.keyword.as_str() == "kou" && lower_case.contains("kou") {
                    continue;
                }
                let m = message.messages.get("en").unwrap();
                let index = thread_rng().gen_range(0, m.len());
                msg.channel_id.say(&context.http, m[index].as_str())
                    .await.expect("Failed to perform random reply.");
                break;
            }
        }
    }
    drop(persistence_lock);
}

async fn emote_command(context: &Context, msg: &Message, emote: &Emote) {
    let mut cmd = std::env::var("PREFIX").unwrap();
    cmd += emote.name.as_str();
    let remains: &str;
    if msg.content.len() > cmd.len() {
        remains = &msg.content[cmd.len() + 1..];
    }
    else {
        remains = "";
    }
    let count = remains.parse::<u8>();
    if let Ok(c) = count {
        let mut message = String::from(&emote.raw);
        for _ in 1..c {
            message += " ";
            message += &emote.raw;
        }
        msg.channel_id.say(&context.http, &message).await
            .expect("Failed to send emote messages.");
    }
    else {
        msg.channel_id.say(&context.http, &emote.link).await
            .expect("Failed to send the emote link.");
    }
}

async fn smite_command(context: &Context, msg: &Message)
{
    let valid_users = [
        263348633280315395,
        677249244842950684,
        617978701962805249,
        457393417287368706,
        297195101753573380,
        215526684797960192,
    ];
    if !valid_users.contains(&msg.author.id.0) {
        return;
    }
    let cmd = "/smite ";
    let user_query = &msg.content[cmd.len()..];
    let guild = msg.guild(&context.cache).await;
    if let Some(found_guild) = guild {
        let user = search_user(context, &found_guild, user_query)
            .await;
        match user {
            Ok(mut found_user) => {
                let user = &mut found_user[0];
                let due_time = Local::now() + chrono::Duration::days(1);
                let context_data = context.data.read().await;
                let persistence = context_data.get::<PersistenceService>()
                    .expect("Failed to get persistence service.");
                let persistence = persistence.clone();
                drop(context_data);
                let mut persistence_lock = persistence.write().await;
                let gif: String;
                {
                    let mut rng = thread_rng();
                    gif = persistence_lock.smite_links
                        .choose(&mut rng)
                        .expect("Failed to get smite gif link.")
                        .clone();
                }
                let entry = persistence_lock.smote_users
                    .entry(user.user.id.0)
                    .or_insert(Local::now());
                *entry = due_time;
                persistence_lock.write();
                drop(persistence_lock);
                drop(persistence);
                let role_ids = [
                    766023350287335465,
                    769101869489979422
                ];
                for role_id in role_ids.iter() {
                    let result: serenity::Result<()> = user
                        .add_role(&context.http, RoleId::from(*role_id))
                        .await;
                    if result.is_ok() {
                        break;
                    }
                }
                msg.channel_id
                    .say(&context.http, &format!("SMITE! {}", &gif))
                    .await
                    .expect("Failed to send message to the channel.");
            },
            Err(e) => {
                msg.channel_id
                    .say(&context.http, &format!("Error when searching for user: {}", e.to_string()))
                    .await
                    .expect("Failed to send message to the channel.");
            }
        }
    }
}

#[hook]
pub async fn unknown_command(context: &Context, msg: &Message, cmd: &str) {
    let data_lock = context.data.read().await;
    let persistence = data_lock.get::<PersistenceService>()
        .expect("Failed to retrieve persistence service.");
    let interface = data_lock.get::<InterfaceService>()
        .expect("Failed to retrieve interface service.");
    let persistence_clone = Arc::clone(persistence);
    let interface_clone = Arc::clone(interface);
    drop(data_lock);
    let persistence_lock = persistence_clone.read().await;
    let interface_lock = interface_clone.read().await;
    let config = persistence_lock.config.as_ref().unwrap();
    let emote_exist = config.emotes.iter()
        .find(|e| e.name == cmd);
    if let Some(e) = emote_exist {
        emote_command(context, msg, e).await;
        return;
    }
    drop(persistence_lock);

    let failed_messages: &Vec<String>;
    failed_messages = &interface_lock.interface_strings.as_ref().unwrap()
        .failed_messages;
    let index = thread_rng().gen_range(0, failed_messages.len());
    let response = failed_messages[index].replace("{command}", cmd);
    msg.channel_id.say(&context.http, &response).await
        .expect("Failed to show failed messages.");
    drop(interface_lock);
}

#[hook]
pub async fn message_received(context: &Context, msg: &Message) {
    let bot_user = context.cache.current_user().await;
    let guild = msg.guild(&context.cache).await;
    if msg.author.id.0 == bot_user.id.0 {
        return;
    }
    let channel = msg.channel(&context.cache).await;
    if channel.is_none() {
        return;
    }
    let private_channel = channel.unwrap().private();
    if private_channel.is_none() {
        let mut guild = guild
            .clone()
            .expect("Failed to get guild from cache.");
        let member = guild
            .members
            .get_mut(&UserId::from(msg.author.id.0))
            .expect("Failed to get member from cache.");
        let mut member = member.clone();

        if msg.channel_id.0 == 722824790972563547_u64 {
            let has_role: bool = msg.author
                .has_role(&context.http, msg.guild_id.clone().expect("Failed to get guild id from message."), RoleId(736534226945572884))
                .await
                .expect("Failed to check if user has required role.");
            if msg.content.as_str() == "I agree with the rule and Kou is the best boi." && !has_role {
                member.add_role(&context.http, RoleId(736534226945572884)).await
                    .expect("Failed to add a role to the user.");
                greeting(context, msg.guild_id.as_ref().unwrap(), &member).await;
                msg.delete(&context.http).await
                    .expect("Failed to delete the message.");
            }
            else if !has_role {
                msg.delete(&context.http).await
                    .expect("Failed to delete the message.");
                let _msg = msg.reply(&context.http, "Your answer is incorrect. Please try again.")
                    .await;
                tokio::time::delay_for(tokio::time::Duration::from_secs(5)).await;
                _msg.unwrap().delete(&context.http).await
                    .expect("Failed to delete the message.");
            }
        }
    }

    handle_self_mentions(context, msg).await;
    handle_reactions(context, msg).await;
    handle_replies(context, msg).await;
    handle_user_replies(context, msg).await;

    if msg.content.starts_with("/smite") {
        smite_command(context, msg).await;
    }

    let context_data = context.data.read().await;
    let persistence = context_data.get::<PersistenceService>()
        .expect("Failed to get persistence service.");
    let interface = context_data.get::<InterfaceService>()
        .expect("Failed to get interface service.");
    let persistence_clone = Arc::clone(persistence);
    let interface_clone = Arc::clone(interface);
    drop(context_data);
    let mut persistence_lock = persistence_clone.write().await;
    let interface_lock = interface_clone.read().await;

    // Update last modified time of persistence storage and write data every 5 minutes.
    let last_modified_time = persistence_lock.last_modified_time.as_ref().unwrap();
    if last_modified_time < &Utc::now() {
        persistence_lock.write();
        persistence_lock.last_modified_time = Some(Utc::now() + Duration::minutes(5));
    }

    // Update presence every 60 minutes.
    let presence_timer = persistence_lock.presence_timer.as_ref().unwrap();
    if presence_timer < &Utc::now() {
        let presences: &[String] = interface_lock
            .interface_strings
            .as_ref()
            .unwrap().presence.borrow();
        let activity = Activity::playing(presences[thread_rng().gen_range(0, presences.len())].as_str());
        let status = OnlineStatus::Online;
        context.set_presence(Some(activity), status).await;
        persistence_lock.presence_timer = Some(Utc::now() + Duration::hours(1));
    }
    drop(interface_lock);

    let mut is_persistence_changed = false;
    let reminders = persistence_lock.reminders.as_mut().unwrap();
    let mut user_id_to_remove = HashSet::new();
    // Remind users
    for (user_id, reminder) in reminders.iter() {
        if !(Local::now() > reminder.datetime) {
            continue;
        }
        let user = context.cache.user(UserId(*user_id)).await;
        if let Some(u) = user {
            let msg = reminder.message.clone();
            user_id_to_remove.insert(*user_id);
            u.direct_message(&context.http, |m| m
                .content(msg.as_str()))
                .await
                .expect("Failed to dm the user.");
            break;
        }
    }
    for id in user_id_to_remove.iter() {
        reminders.remove(id);
        is_persistence_changed = true;
    }

    // Revoke Smote role
    let smote_users = &mut persistence_lock.smote_users;
    let mut smote_users_to_remove = HashSet::new();
    for (user_id, due_time) in smote_users.iter_mut() {
        if Local::now() > *due_time {
            smote_users_to_remove.insert(*user_id);
        }
    }
    for id in smote_users_to_remove.iter() {
        let mut guild = guild
            .clone()
            .expect("Failed to get guild from cache.");
        let member = guild
            .members
            .get_mut(&UserId::from(msg.author.id.0))
            .expect("Failed to get member from cache.");
        member.remove_role(&context.http, RoleId::from(*id))
            .await
            .expect("Failed to remove role from the member.");
        smote_users.remove(id);
        is_persistence_changed = true;
    }
    if is_persistence_changed {
        persistence_lock.write();
    }
    drop(persistence_lock);
}

#[hook]
pub async fn before(context: &Context, msg: &Message, command_name: &str) -> bool {
    let lock = context.data.read().await;
    let persistence = lock.get::<PersistenceService>().unwrap();
    let _persistence = Arc::clone(persistence);
    drop(lock);
    let persistence_lock = _persistence.read().await;
    let channel = msg.channel(&context.cache).await.unwrap();
    let guild_channel = channel.guild();
    let enabled_channels = &persistence_lock.channel_settings.as_ref().unwrap().enabled_channels;
    if ADMIN_COMMANDS.contains(&command_name) {
        drop(persistence_lock);
        return true;
    }
    if !enabled_channels.contains(&msg.channel_id.0) && guild_channel.is_some() {
        drop(persistence_lock);
        return false;
    }
    true
}

#[hook]
pub async fn dispatch_error(context: &Context, msg: &Message, error: DispatchError) {
    let http = &context.http;
    match error {
        DispatchError::LackingRole => {
            msg.reply(http, "You don't have the required role to perform such operation.")
                .await
                .expect("Failed to indicate that the user needs proper roles to perform the operation.");
        },
        DispatchError::OnlyForGuilds => {
            msg.reply(http, "This command can only be used in a guild.")
                .await
                .expect("Failed to indicate that the command can only be used in a guild.");
        },
        DispatchError::OnlyForOwners => {
            msg.reply(http, "This command can only be used by the owners.")
                .await
                .expect("Failed to indicate that the command can only be used by the owners.");
        },
        DispatchError::Ratelimited(time) => {
            let lock = context.data.read().await;
            let interface = lock.get::<InterfaceService>().unwrap();
            let interface_lock = interface.read().await;
            let error_msg = interface_lock
                .interface_strings
                .as_ref()
                .unwrap()
                .cool_down.clone();
            drop(interface_lock);
            drop(lock);
            let seconds = time.as_secs().to_string();
            let error_msg = error_msg.replace("{timeLeft}", seconds.as_str());
            msg.reply(http, &error_msg).await
                .expect("Failed to show cool down message.");
        },
        _ => {
            msg.reply(http, "There was an error trying to execute that command...")
                .await
                .expect("Failed to reply to a generic error.");
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, context: Context, guild_id: GuildId, member: Member) {
        if guild_id.0 == KOU_SERVER_ID {
            return;
        }
        greeting(&context, &guild_id, &member).await;
    }

    async fn ready(&self, context: Context, ready: Ready) {
        let lock = context.data.read().await;
        let interface = lock.get::<InterfaceService>().unwrap();
        let interface_lock = interface.read().await;
        let presences = interface_lock
            .interface_strings
            .as_ref()
            .unwrap().presence.to_vec();
        drop(interface_lock);
        drop(lock);
        let activity = Activity::playing(presences[thread_rng().gen_range(0, presences.len())].as_str());
        let status = OnlineStatus::Online;
        context.set_presence(Some(activity), status).await;
        info!("{} is now online!", ready.user.name.as_str());
    }
}

async fn greeting(context: &Context, guild_id: &GuildId, member: &Member) {
    let data = context.data.read().await;
    let command_groups = data.get::<CommandGroupCollection>().unwrap().to_vec();
    let interface = data.get::<InterfaceService>().unwrap();
    let persistence = data.get::<PersistenceService>().unwrap();
    let interface_lock = interface.read().await;
    let persistence_lock = persistence.read().await;
    let is_kou = interface_lock.is_kou;
    let mut text = persistence_lock.guide_text.clone();
    let greetings = interface_lock.interface_strings
        .as_ref()
        .unwrap()
        .greetings
        .to_vec();
    drop(persistence_lock);
    drop(interface_lock);
    drop(data);
    let greeting: String;
    {
        let mut rng = thread_rng();
        greeting = greetings.choose(&mut rng).unwrap()
            .replace("{name}", format!("<@{}>", &member.user.id.0).as_str());
    }
    let mut general_channels: Vec<String> = vec![];
    general_channels.push(env::var("GENCHN").unwrap());
    general_channels.push(env::var("TESTGENCHN").unwrap());
    general_channels.push(env::var("KOUGENCHN").unwrap());
    general_channels.push(env::var("ECC_GENCHAN").unwrap());

    let guild_channels: HashMap<ChannelId, GuildChannel> = guild_id.channels(&context.http)
        .await.unwrap();
    for channel in general_channels.iter() {
        let guild = guild_channels
            .get(&ChannelId::from(channel.parse::<u64>().unwrap()));
        if let Some(c) = guild {
            c.say(&context.http, &greeting).await
                .expect("Failed to greet the newly added member.");
            break;
        }
    }

    text = text.replace("{user}", &member.user.mention());
    let guild_name = guild_id.name(&context.cache).await.unwrap_or_default();
    text = text.replace("{guildName}", &guild_name);
    let color_code = u32::from_str_radix(if is_kou {
        "a4d0da"
    } else {
        "e81615"
    }, 16).unwrap();
    let color = Color::new(color_code);

    build_embed(context, member, &command_groups, color, text.as_str(), is_kou, guild_name.as_str()).await
        .expect("Failed to send DM to the newcomer.");
}