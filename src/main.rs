#[macro_use]
extern crate dotenv_codegen;
extern crate taiga_bot_rs;
use serenity::async_trait;
use serenity::client::Client;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{StandardFramework, macros::{
    group, check, hook
}};
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
use serenity::model::gateway::{Ready, Activity};
use std::borrow::Borrow;
use rand::{thread_rng, Rng};
use serenity::model::user::OnlineStatus;
use serenity::model::channel::Message;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::http::Http;
use std::collections::HashSet;

const ADMIN_COMMANDS: [&'static str; 5] = [
    "allow", "disable", "enable", "ignore", "purge"
];

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

#[async_trait]
impl EventHandler for Handler {
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

    let token: &str = dotenv!("TOKEN");
    let http = Http::new_with_token(token);
    let app_info = http.get_current_application_info().await?;
    let mut owners = HashSet::new();
    owners.insert(app_info.owner.id.clone());

    unsafe {
        AUTHENTICATION_SERVICE.login().await?;
        let _ = PERSISTENCE_STORAGE.load().await?;
        INTERFACE_SERVICE.load(true)?;
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
        eprintln!("An error occurred while running the client: {:?}", reason);
    }

    Ok(())
}
