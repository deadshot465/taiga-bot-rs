#![allow(unused_assignments)]
#[macro_use]
extern crate lazy_static;
mod commands;
mod handlers;
mod protos;
mod shared;
pub use commands::{
    about, admin, avatar, comic, convert, dialog, emote, enlarge, games, guide, help, image, meal,
    oracle, owoify, pick, ping, remind, route, say, ship, stats, time, valentine,
};
pub use dotenv::dotenv;
pub use handlers::*;
pub use protos::discord_bot_service;
pub use shared::*;
