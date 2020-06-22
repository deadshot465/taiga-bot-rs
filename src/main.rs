#[macro_use]
extern crate dotenv_codegen;
extern crate taiga_bot_rs;
use serenity::async_trait;
use serenity::client::Client;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{StandardFramework, macros::{
    group, help
}, Args, HelpOptions, help_commands, CommandGroup, CommandResult};
use taiga_bot_rs::{
    about::ABOUT_COMMAND, convert::CVT_COMMAND,
    dialog::DIALOG_COMMAND, enlarge::ENLARGE_COMMAND,
    image::IMAGE_COMMAND, meal::MEAL_COMMAND,
    oracle::ORACLE_COMMAND, owoify::OWOIFY_COMMAND, pick::PICK_COMMAND,
    ping::PING_COMMAND, route::ROUTE_COMMAND, ship::SHIP_COMMAND,
    time::TIME_COMMAND, valentine::VALENTINE_COMMAND,
    AUTHENTICATION_SERVICE, PERSISTENCE_STORAGE, INTERFACE_SERVICE
};
use serenity::model::channel::Message;
use std::collections::HashSet;
use serenity::model::id::UserId;

#[help]
#[individual_command_tip = "Hello there."]
#[command_not_found_text = "Cannot find: `{}`."]
#[max_levenshtein_distance(3)]
async fn test_help(context: &Context, msg: &Message, args: Args,
                   help_options: &'static HelpOptions, groups: &[&'static CommandGroup],
                   owners: HashSet<UserId>) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await
}

#[group]
#[commands(oracle, route, valentine)]
struct Lottery;

#[group]
#[commands(about, cvt, dialog, enlarge, image, meal, owoify, oracle, pick, ping, route, ship, time, valentine)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new(dotenv!("TOKEN"))
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c
            .prefix(dotenv!("PREFIX")))
            .bucket("lottery", |l| l.delay(5)).await
            .help(&TEST_HELP)
            .group(&GENERAL_GROUP)
            .group(&LOTTERY_GROUP)
            )
        .await
        .expect("Error creating client");

    unsafe {
        AUTHENTICATION_SERVICE.login().await?;
        let _ = PERSISTENCE_STORAGE.load().await?;
        INTERFACE_SERVICE.load(true)?;
    }

    if let Err(reason) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", reason);
    }

    Ok(())
}
