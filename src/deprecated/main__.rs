extern crate taiga_bot_rs;
use env_logger::Builder;
use log::{error, LevelFilter};
use serenity::framework::standard::CommandGroup;
use serenity::{
    client::{bridge::gateway::GatewayIntents, Client},
    framework::standard::{macros::group, StandardFramework},
    http::Http,
};
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use taiga_bot_rs::{
    about::ABOUT_COMMAND, admin::channel_control::*, avatar::AVATAR_COMMAND, before,
    comic::COMIC_COMMAND, convert::CVT_COMMAND, dialog::DIALOG_COMMAND, dispatch_error, emote::*,
    enlarge::ENLARGE_COMMAND, games::*, guide::GUIDE_COMMAND, help::CUSTOM_HELP,
    image::IMAGE_COMMAND, meal::MEAL_COMMAND, message_received, oracle::ORACLE_COMMAND,
    owoify::OWOIFY_COMMAND, pick::PICK_COMMAND, ping::PING_COMMAND, remind::REMIND_COMMAND,
    route::ROUTE_COMMAND, say::*, ship::SHIP_COMMAND, stats::STATS_COMMAND, time::TIME_COMMAND,
    unknown_command, valentine::VALENTINE_COMMAND, Authentication, AuthenticationService,
    CommandGroupCollection, Handler, InterfaceService, InterfaceStorage, PersistenceService,
    PersistenceStorage,
};
use tokio::sync::{Mutex, RwLock};

#[group]
#[description = "Administrative commands. Can only be used by Administrators."]
#[only_in("guilds")]
#[required_permissions("ADMINISTRATOR")]
#[commands(allow, disable, enable, ignore, purge)]
struct Admin;

#[group]
#[description = "Fun commands that would probably make your day."]
#[only_in("guilds")]
#[commands(comic, dialog, emote, owoify, ship)]
struct Fun;

#[group]
#[description = "Play small games with Taiga, Kou, or your friends."]
#[prefixes("games")]
#[only_in("guilds")]
#[commands(quiz, hangman, tictactoe)]
struct Games;

#[group]
#[description = "Informative commands that will show information, suggestions, or query results."]
#[only_in("guilds")]
#[commands(about, guide, meal, oracle, ping, route, stats, time, valentine)]
struct Information;

#[group]
#[description = "Returns an image of various characters saying anything you want."]
#[prefixes("say")]
#[only_in("guilds")]
#[commands(
    eduardsay, hirosay, huntersay, leesay, mhirosay, taigasay, keitarosay, yoichisay, yurisay,
    kieransay, natsumisay
)]
struct Say;

#[group]
#[description = "Utility functions that basically serve as tools."]
#[commands(avatar, cvt, enlarge, image, pick, remind)]
struct Utilities;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    Builder::new()
        .filter(None, LevelFilter::Warn)
        .default_format()
        .init();
    let args = env::args().collect::<Vec<String>>();
    let args = args
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    let token = env::var("TOKEN").unwrap();
    let http = Http::new_with_token(token.as_str());
    let app_info = http.get_current_application_info().await?;
    let mut owners = HashSet::new();
    owners.insert(app_info.owner.id);

    let prefix = env::var("PREFIX").unwrap();

    let mut client = Client::builder(token)
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::GUILD_PRESENCES
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_TYPING
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::DIRECT_MESSAGES,
        )
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.owners(owners).prefix(&prefix))
                .before(before)
                .normal_message(message_received)
                .unrecognised_command(unknown_command)
                .on_dispatch_error(dispatch_error)
                .bucket("information", |l| l.delay(2))
                .await
                .bucket("say", |l| l.delay(10).time_span(30).limit(2))
                .await
                .bucket("games", |l| l.delay(5).time_span(30).limit(2))
                .await
                .help(&CUSTOM_HELP)
                .group(&ADMIN_GROUP)
                .group(&FUN_GROUP)
                .group(&GAMES_GROUP)
                .group(&INFORMATION_GROUP)
                .group(&SAY_GROUP)
                .group(&UTILITIES_GROUP),
        )
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        let interface = Arc::new(RwLock::new(InterfaceStorage::new()));
        let mut lock = interface.write().await;
        let is_kou = args.contains(&"kou".to_string());
        lock.load(is_kou).expect("Failed to load interface.");
        drop(lock);
        data.insert::<InterfaceService>(interface);
        data.insert::<PersistenceService>(Arc::new(RwLock::new(
            PersistenceStorage::new(is_kou).await?,
        )));
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
