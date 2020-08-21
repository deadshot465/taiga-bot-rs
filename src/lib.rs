#![allow(unused_assignments)]
#[macro_use]
extern crate lazy_static;
mod commands;
mod handlers;
mod shared;
pub use commands::{
    about, admin, avatar, comic, convert, dialog, enlarge, emote, games, guide, help, image, meal,
    oracle, owoify, pick, ping, route, remind, say, ship, stats, time, valentine
};
pub use dotenv::dotenv;
pub use handlers::*;
pub use shared::*;