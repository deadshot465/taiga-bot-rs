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
use taiga_bot_rs::{enlarge::ENLARGE_COMMAND, ping::PING_COMMAND, route::ROUTE_COMMAND, valentine::VALENTINE_COMMAND, AUTHENTICATION_SERVICE};

#[group]
#[commands(enlarge, ping, route, valentine)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let mut client = Client::new(dotenv!("TOKEN"))
        .event_handler(Handler)
        .framework(StandardFramework::new().configure(|c| c
            .prefix(dotenv!("PREFIX")))
            .group(&GENERAL_GROUP))
        .await
        .expect("Error creating client");

    unsafe {
        AUTHENTICATION_SERVICE.login().await;
    }

    if let Err(reason) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", reason);
    }
}
