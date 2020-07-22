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
use crate::{PERSISTENCE_STORAGE, INTERFACE_SERVICE, get_image, get_dialog, UserRecords};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use chrono::{Utc, Local, Duration};

const ADMIN_COMMANDS: [&'static str; 7] = [
    "allow", "cvt", "convert", "disable", "enable", "ignore", "purge"
];

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
    unsafe {
        let random_messages = PERSISTENCE_STORAGE.random_messages.as_ref().unwrap();
        let messages = random_messages.iter()
            .find(|m| {
                if INTERFACE_SERVICE.is_kou {
                    m.keyword.as_str() == "kou"
                }
                else {
                    m.keyword.as_str() == "taiga"
                }
            }).unwrap();

        // Randomly replies to messages that mention the bot.
        if msg.content.contains(&bot_id) && hit_or_miss(mention_reaction_chance) {
            let english_msgs = &messages.messages["en"];
            let index = thread_rng().gen_range(0, english_msgs.len());
            msg.channel_id.say(&context.http, english_msgs[index].as_str())
                .await
                .expect("Failed to reply to mention.");
        }
    }
}

async fn handle_reactions(context: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }
    let reaction_chance: u8 = env::var("REACTION_CHANCE")
        .unwrap()
        .parse::<u8>().unwrap();
    unsafe {
        // Randomly reacts to messages that contains certain keywords.
        if hit_or_miss(reaction_chance) {
            for m in PERSISTENCE_STORAGE.random_messages.as_ref().unwrap().iter() {
                let lower_case = msg.content.to_lowercase();
                if !lower_case.contains(m.keyword.as_str()) {
                    continue;
                }
                if m.keyword.as_str() == "lee" && lower_case.contains("sleep") {
                    continue;
                }
                if m.keyword.as_str() != "kou" && INTERFACE_SERVICE.is_kou {
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
                    msg.react(&context.http, reaction_type).await.expect("Failed to react.");
                }
                else {
                    let reaction_type = ReactionType::Unicode(reaction.to_string());
                    msg.react(&context.http, reaction_type).await.expect("Failed to react.");
                }
            }
        }
    }
}

async fn handle_user_replies(context: &Context, msg: &Message) {
    unsafe {
        if PERSISTENCE_STORAGE.channel_settings.as_ref().unwrap()
            .ignored_channels.contains(&msg.channel_id.0) {
            return;
        }
        if msg.author.bot {
            return;
        }
        if INTERFACE_SERVICE.is_kou {
            return;
        }
        let user_replies = PERSISTENCE_STORAGE.user_replies.as_ref().unwrap();
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
            let index = thread_rng().gen_range(0, messages.len());
            msg.reply(&context.http, messages[index].as_str())
                .await
                .expect("Failed to reply to the user.");
            let user_id = msg.author.id.0.to_string();
            let user_records = PERSISTENCE_STORAGE.user_records
                .as_mut()
                .unwrap()
                .entry(user_id)
                .or_insert(UserRecords::new());
            let reply_count = &mut user_records.replies;
            *reply_count += 1;
        }
    }
}

async fn handle_replies(context: &Context, msg: &Message) {
    unsafe {
        if PERSISTENCE_STORAGE.channel_settings.as_ref().unwrap()
            .ignored_channels.contains(&msg.channel_id.0) {
            return;
        }
    }
    if msg.author.bot {
        return;
    }
    let lower_case = msg.content.to_lowercase();
    unsafe {
        let all_messages = PERSISTENCE_STORAGE.random_messages.as_ref().unwrap();
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

        if INTERFACE_SERVICE.is_kou {
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
                let backgrounds = PERSISTENCE_STORAGE.dialog_backgrounds
                    .as_ref()
                    .unwrap();
                let index = thread_rng().gen_range(0, backgrounds.len());
                let background = backgrounds[index].as_str();
                if lower_case.contains("hiro") {
                    let character = "taiga";
                    let text = "Hiro will be terribly wrong if he thinks he can steal Keitaro from me!";
                    let bytes = get_dialog(background, character, text).await.unwrap();
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
                    let m = message.messages.get("en").unwrap();
                    let index = thread_rng().gen_range(0, m.len());
                    msg.channel_id.say(&context.http, m[index].as_str())
                        .await.expect("Failed to perform random reply.");
                    break;
                }
            }
        }
    }
}

#[hook]
pub async fn unknown_command(context: &Context, msg: &Message, cmd: &str) {
    let failed_messages: &Vec<String>;
    unsafe {
        failed_messages = &INTERFACE_SERVICE.interface_strings.as_ref().unwrap()
            .failed_messages;
    }
    let index = thread_rng().gen_range(0, failed_messages.len());
    let response = failed_messages[index].replace("{command}", cmd);
    msg.channel_id.say(&context.http, &response).await
        .expect("Failed to show failed messages.");
}

#[hook]
pub async fn message_received(context: &Context, msg: &Message) {
    unsafe {
        handle_self_mentions(context, msg).await;
        handle_reactions(context, msg).await;
        handle_replies(context, msg).await;
        handle_user_replies(context, msg).await;

        // Update last modified time of persistence storage and write data every 5 minutes.
        let last_modified_time = PERSISTENCE_STORAGE.last_modified_time.as_ref().unwrap();
        if last_modified_time < &Utc::now() {
            let persistence = &mut PERSISTENCE_STORAGE;
            persistence.write();
            persistence.last_modified_time = Some(Utc::now() + Duration::minutes(5));
        }

        // Update presence every 60 minutes.
        let presence_timer = PERSISTENCE_STORAGE.presence_timer.as_ref().unwrap();
        if presence_timer < &Utc::now() {
            let presences: &[String] = INTERFACE_SERVICE
                .interface_strings
                .as_ref()
                .unwrap().presence.borrow();
            let activity = Activity::playing(presences[thread_rng().gen_range(0, presences.len())].as_str());
            let status = OnlineStatus::Online;
            context.set_presence(Some(activity), status).await;
            let persistence = &mut PERSISTENCE_STORAGE;
            persistence.presence_timer = Some(Utc::now() + Duration::hours(1));
        }

        // Remind users
        for (user_id, reminder) in PERSISTENCE_STORAGE.reminders.as_ref().unwrap().iter() {
            if !(Local::now() > reminder.datetime) {
                continue;
            }
            let user = context.cache.user(UserId(*user_id)).await;
            if let Some(u) = user {
                let msg = reminder.message.clone();
                let reminders = PERSISTENCE_STORAGE.reminders.as_mut().unwrap();
                reminders.remove(user_id);
                PERSISTENCE_STORAGE.write();
                u.direct_message(&context.http, |m| m
                    .content(msg.as_str()))
                    .await
                    .expect("Failed to dm the user.");
                break;
            }
        }
    }
}

#[hook]
pub async fn before(context: &Context, msg: &Message, command_name: &str) -> bool {
    let channel = msg.channel(&context.cache).await.unwrap();
    let guild_channel = channel.guild();
    unsafe {
        let enabled_channels = &PERSISTENCE_STORAGE.channel_settings.as_ref().unwrap().enabled_channels;
        if ADMIN_COMMANDS.contains(&command_name) {
            return true;
        }
        if !enabled_channels.contains(&msg.channel_id.0) && guild_channel.is_some() {
            return false;
        }
    }

    true
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, context: Context, guild_id: GuildId, member: Member) {
        let greetings: &Vec<String>;
        unsafe {
            greetings = &INTERFACE_SERVICE.interface_strings
                .as_ref()
                .unwrap()
                .greetings;
        }
        let greeting: String;
        {
            let mut rng = thread_rng();
            greeting = greetings[rng.gen_range(0, greetings.len())]
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
                return;
            }
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        unsafe {
            let presences: &[String] = INTERFACE_SERVICE
                .interface_strings
                .as_ref()
                .unwrap().presence.borrow();
            let activity = Activity::playing(presences[thread_rng().gen_range(0, presences.len())].as_str());
            let status = OnlineStatus::Online;
            context.set_presence(Some(activity), status).await;
        }
        info!("{} is now online!", ready.user.name.as_str());
    }
}