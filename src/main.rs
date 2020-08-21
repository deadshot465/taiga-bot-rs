extern crate taiga_bot_rs;
use log::{error};
use std::collections::HashSet;
use std::env;
use serenity::{
    client::{
        bridge::gateway::GatewayIntents,
        Client,
    },
    framework::standard::{StandardFramework, macros::{
        group
    }},
    http::Http,
};
use taiga_bot_rs::{about::ABOUT_COMMAND, avatar::AVATAR_COMMAND, comic::COMIC_COMMAND, convert::CVT_COMMAND, dialog::DIALOG_COMMAND, enlarge::ENLARGE_COMMAND, emote::*, games::*, help::CUSTOM_HELP, image::IMAGE_COMMAND, meal::MEAL_COMMAND, oracle::ORACLE_COMMAND, owoify::OWOIFY_COMMAND, pick::PICK_COMMAND, ping::PING_COMMAND, remind::REMIND_COMMAND, route::ROUTE_COMMAND, say::*, ship::SHIP_COMMAND, stats::STATS_COMMAND, time::TIME_COMMAND, valentine::VALENTINE_COMMAND, admin::channel_control::*, Handler, before, message_received, unknown_command, dispatch_error, PersistenceService, PersistenceStorage, InterfaceService, InterfaceStorage, AuthenticationService, Authentication};
use serenity::prelude::TypeMapKey;
use serenity::framework::standard::CommandGroup;
use std::sync::Arc;
use tokio::sync::Mutex;

#[group]
#[only_in("guilds")]
#[required_permissions("ADMINISTRATOR")]
#[commands(allow, disable, enable, ignore, purge)]
struct Admin;

#[group]
#[only_in("guilds")]
#[commands(comic, dialog, emote, owoify, ship)]
struct Fun;

#[group]
#[description = "Play a small game with Kou."]
#[prefixes("games")]
#[only_in("guilds")]
#[commands(quiz, hangman, tictactoe)]
struct Games;

#[group]
#[only_in("guilds")]
#[commands(about, meal, oracle, ping, route, stats, time, valentine)]
struct Information;

#[group]
#[description = "Returns an image of various characters saying anything you want."]
#[prefixes("say")]
#[only_in("guilds")]
#[commands(hirosay, huntersay, mhirosay, taigasay, keitarosay, yoichisay, yurisay, kieransay, natsumisay)]
struct Say;

#[group]
#[commands(avatar, cvt, enlarge, image, pick, remind)]
struct Utilities;

struct CommandGroupCollection;
impl TypeMapKey for CommandGroupCollection {
    type Value = Vec<&'static CommandGroup>;
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    let args = args.iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    let token = env::var("TOKEN").unwrap();
    let http = Http::new_with_token(token.as_str());
    let app_info = http.get_current_application_info().await?;
    let mut owners = HashSet::new();
    owners.insert(app_info.owner.id.clone());

    let prefix = env::var("PREFIX").unwrap();

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
            .prefix(&prefix))
            .before(before)
            .normal_message(message_received)
            .unrecognised_command(unknown_command)
            .on_dispatch_error(dispatch_error)
            .bucket("information", |l| l.delay(2)).await
            .bucket("say", |l| l.delay(10).time_span(30).limit(2)).await
            .bucket("games", |l| l.delay(5).time_span(30).limit(2)).await
            .help(&CUSTOM_HELP)
            .group(&ADMIN_GROUP)
            .group(&FUN_GROUP)
            .group(&GAMES_GROUP)
            .group(&INFORMATION_GROUP)
            .group(&SAY_GROUP)
            .group(&UTILITIES_GROUP)
            )
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        let interface = Arc::new(Mutex::new(InterfaceStorage::new()));
        let mut lock = interface.lock().await;
        let is_kou = if args.contains(&"kou".to_string()) {
            true
        } else {
            false
        };
        lock.load(is_kou).expect("Failed to load interface.");
        drop(lock);
        data.insert::<InterfaceService>(interface);
        data.insert::<PersistenceService>(Arc::new(Mutex::new(PersistenceStorage::new(is_kou).await)));
        data.insert::<AuthenticationService>(Arc::new(Mutex::new(Authentication::new().await)));

        let mut command_groups: Vec<&CommandGroup> = vec![];
        command_groups.push(&ADMIN_GROUP);
        command_groups.push(&FUN_GROUP);
        command_groups.push(&GAMES_GROUP);
        command_groups.push(&INFORMATION_GROUP);
        command_groups.push(&SAY_GROUP);
        command_groups.push(&UTILITIES_GROUP);
        data.insert::<CommandGroupCollection>(command_groups);
    }

    if let Err(reason) = client.start().await {
        error!("An error occurred while running the client: {:?}", reason);
    }

    Ok(())
}
