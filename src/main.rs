#[macro_use]
extern crate dotenv_codegen;
extern crate taiga_bot_rs;
use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};
use taiga_bot_rs::{ping::PING_COMMAND, route::ROUTE_COMMAND};

#[group]
#[commands(ping, route)]
struct General;

struct Handler;
impl EventHandler for Handler {}

fn main() {
    let mut client = Client::new(dotenv!("TOKEN"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(dotenv!("PREFIX")))
        .group(&GENERAL_GROUP));

    if let Err(reason) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", reason);
    }
}
