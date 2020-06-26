#[macro_use]
extern crate dotenv_codegen;
extern crate taiga_bot_rs;
use log::{debug, error, info};
use rand::{thread_rng, Rng};
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashSet;
use serenity::{
    async_trait,
    client::{
        bridge::gateway::GatewayIntents,
        Client,
    },
    framework::standard::{StandardFramework, macros::{
        group, check, hook
    }},
    http::Http,
    model::{
        channel::Message,
        gateway::{Ready, Activity},
        user::OnlineStatus,
        prelude::*
    },
    prelude::{EventHandler, Context}
};
use taiga_bot_rs::{
    about::ABOUT_COMMAND, convert::CVT_COMMAND,
    dialog::DIALOG_COMMAND, enlarge::ENLARGE_COMMAND, help::CUSTOM_HELP,
    image::IMAGE_COMMAND, meal::MEAL_COMMAND,
    oracle::ORACLE_COMMAND, owoify::OWOIFY_COMMAND, pick::PICK_COMMAND,
    ping::PING_COMMAND, route::ROUTE_COMMAND, say::*,
    ship::SHIP_COMMAND, stats::STATS_COMMAND,
    time::TIME_COMMAND, valentine::VALENTINE_COMMAND,

    admin::channel_control::*,
    AUTHENTICATION_SERVICE, PERSISTENCE_STORAGE, INTERFACE_SERVICE
};
use serenity::model::channel::ReactionType;

const ADMIN_COMMANDS: [&'static str; 5] = [
    "allow", "disable", "enable", "ignore", "purge"
];

lazy_static::lazy_static! {
    static ref ANIMATED_REGEX: Regex = Regex::new(r"(<a)").unwrap();
    static ref EMOTE_REGEX: Regex = Regex::new(r"<:(\w+):(\d+)>").unwrap();
}

#[group]
#[only_in("guilds")]
#[required_permissions("ADMINISTRATOR")]
#[commands(allow, disable, enable, ignore, purge)]
struct Admin;

#[group]
#[only_in("guilds")]
#[commands(dialog, owoify, ship)]
struct Fun;

#[group]
#[only_in("guilds")]
#[commands(about, meal, oracle, ping, route, stats, time, valentine)]
struct Information;

#[group]
#[description = "Returns an image of various characters saying anything you want."]
#[prefixes("say")]
#[only_in("guilds")]
#[commands(hirosay, mhirosay, taigasay, keitarosay, yoichisay, yurisay, kieransay, natsumisay)]
struct Say;

#[group]
#[commands(cvt, enlarge, image, pick)]
struct Utilities;

struct Handler;

fn hit_or_miss(chance: u8) -> bool {
    thread_rng().gen_range(0_u8, 100_u8) < chance
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        let bot_id: &str = dotenv!("BOT_ID");
        let mention_reaction_chance: u8 = dotenv!("MENTION_REACTION_CHANCE").parse::<u8>().unwrap();
        let reaction_chance: u8 = dotenv!("REACTION_CHANCE").parse::<u8>().unwrap();
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
            if msg.content.contains(bot_id) && hit_or_miss(mention_reaction_chance) {
                let english_msgs = &messages.messages["en"];
                let index = thread_rng().gen_range(0, english_msgs.len());
                msg.channel_id.say(&context.http, english_msgs[index].as_str())
                    .await;
            }

            // Randomly reacts to messages that contains certain keywords.
            if !msg.author.bot && hit_or_miss(reaction_chance) {
                for m in PERSISTENCE_STORAGE.random_messages.as_ref().unwrap().iter() {
                    let lower_case = msg.content.to_lowercase();
                    if !lower_case.contains(m.keyword.as_str()) {
                        continue;
                    }
                    if m.keyword.as_str() == "lee" && lower_case.contains("sleep") {
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
                        msg.react(&context.http, reaction_type).await;
                    }
                    else {
                        let reaction_type = ReactionType::Unicode(reaction.to_string());
                        msg.react(&context.http, reaction_type).await;
                    }
                }
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

#[hook]
async fn before(context: &Context, msg: &Message, command_name: &str) -> bool {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let token: &str = dotenv!("TOKEN");
    let http = Http::new_with_token(token);
    let app_info = http.get_current_application_info().await?;
    let mut owners = HashSet::new();
    owners.insert(app_info.owner.id.clone());

    unsafe {
        AUTHENTICATION_SERVICE.login().await?;
        INTERFACE_SERVICE.load(true)?;
        let _ = PERSISTENCE_STORAGE.load().await?;
    }

    let mut client = Client::new(token)
        .add_intent(GatewayIntents::GUILDS)
        .add_intent(GatewayIntents::GUILD_MEMBERS)
        .add_intent(GatewayIntents::GUILD_VOICE_STATES)
        .add_intent(GatewayIntents::GUILD_PRESENCES)
        .add_intent(GatewayIntents::GUILD_MESSAGES)
        .add_intent(GatewayIntents::GUILD_MESSAGE_REACTIONS)
        .add_intent(GatewayIntents::GUILD_MESSAGE_TYPING)
        .add_intent(GatewayIntents::GUILD_MESSAGE_REACTIONS)
        .add_intent(GatewayIntents::DIRECT_MESSAGES)
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c
            .owners(owners)
            .prefix(dotenv!("PREFIX")))
            .before(before)
            .bucket("information", |l| l.delay(2)).await
            .bucket("say", |l| l.delay(10).time_span(30).limit(2)).await
            .help(&CUSTOM_HELP)
            .group(&ADMIN_GROUP)
            .group(&FUN_GROUP)
            .group(&INFORMATION_GROUP)
            .group(&SAY_GROUP)
            .group(&UTILITIES_GROUP)
            )
        .await
        .expect("Error creating client");

    if let Err(reason) = client.start().await {
        error!("An error occurred while running the client: {:?}", reason);
    }

    Ok(())
}
