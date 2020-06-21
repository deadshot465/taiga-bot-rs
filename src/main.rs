#[macro_use]
extern crate dotenv_codegen;
extern crate taiga_bot_rs;
use serenity::async_trait;
use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};
use taiga_bot_rs::{
    about::ABOUT_COMMAND, dialog::DIALOG_COMMAND, enlarge::ENLARGE_COMMAND,
    oracle::ORACLE_COMMAND, owoify::OWOIFY_COMMAND, pick::PICK_COMMAND,
    ping::PING_COMMAND, route::ROUTE_COMMAND, ship::SHIP_COMMAND,
    valentine::VALENTINE_COMMAND, AUTHENTICATION_SERVICE, PERSISTENCE_STORAGE, INTERFACE_SERVICE
};

#[group]
#[commands(about, dialog, enlarge, owoify, oracle, pick, ping, route, ship, valentine)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new(dotenv!("TOKEN"))
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c
            .prefix(dotenv!("PREFIX")))
            .group(&GENERAL_GROUP))
        .await
        .expect("Error creating client");

    unsafe {
        AUTHENTICATION_SERVICE.login().await?;
        let _ = PERSISTENCE_STORAGE.get_instance().await;
        INTERFACE_SERVICE.load(true)?;
    }

    if let Err(reason) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", reason);
    }

    Ok(())
}
