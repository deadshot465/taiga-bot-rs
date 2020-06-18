#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate lazy_static;
mod commands;
mod shared;
pub use commands::{
    dialog, enlarge, oracle, pick, ping, route, ship, valentine
};
pub use shared::AUTHENTICATION_SERVICE;
pub use shared::PERSISTENCE_STORAGE;