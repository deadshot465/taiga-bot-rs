#[macro_use]
extern crate dotenv_codegen;
extern crate taiga_bot_rs;
use serenity::async_trait;
use serenity::client::Client;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{StandardFramework, macros::{
    group
}};
use taiga_bot_rs::{
    about::ABOUT_COMMAND, convert::CVT_COMMAND,
    dialog::DIALOG_COMMAND, enlarge::ENLARGE_COMMAND, help::CUSTOM_HELP,
    image::IMAGE_COMMAND, meal::MEAL_COMMAND,
    oracle::ORACLE_COMMAND, owoify::OWOIFY_COMMAND, pick::PICK_COMMAND,
    ping::PING_COMMAND, route::ROUTE_COMMAND, say::*,
    ship::SHIP_COMMAND, stats::STATS_COMMAND,
    time::TIME_COMMAND, valentine::VALENTINE_COMMAND,
    AUTHENTICATION_SERVICE, PERSISTENCE_STORAGE, INTERFACE_SERVICE
};
use serenity::model::gateway::{Ready, Activity};
use std::borrow::Borrow;
use rand::{thread_rng, Rng};
use serenity::model::user::OnlineStatus;

#[group]
#[commands(dialog, owoify, ship)]
struct Fun;

#[group]
#[commands(about, meal, oracle, ping, route, stats, time, valentine)]
struct Information;

#[group]
#[description = "Returns an image of various characters saying anything you want."]
#[prefixes("say")]
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
            let presences = INTERFACE_SERVICE
                .interface_strings
                .as_ref()
                .unwrap().presence.borrow();
            let activity = Activity::playing(presences[thread_rng().gen_range(0, presences.len())].as_str());
            let status = OnlineStatus::Online;
            context.set_presence(Some(activity), status);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        AUTHENTICATION_SERVICE.login().await?;
        let _ = PERSISTENCE_STORAGE.load().await?;
        INTERFACE_SERVICE.load(true)?;
    }

    let mut client = Client::new(dotenv!("TOKEN"))
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c
            .prefix(dotenv!("PREFIX")))
            .bucket("information", |l| l.delay(2)).await
            .bucket("say", |l| l.delay(10).time_span(30).limit(2)).await
            .help(&CUSTOM_HELP)
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
