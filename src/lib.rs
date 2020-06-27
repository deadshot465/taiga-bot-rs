#![allow(unused_assignments)]
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate lazy_static;
mod commands;
mod shared;
pub use commands::{
    about, admin, convert, dialog, enlarge, help, image, meal, oracle,
    owoify, pick, ping, route, say, ship, stats, time, valentine
};
pub use shared::*;