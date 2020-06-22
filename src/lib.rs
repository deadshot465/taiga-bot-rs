#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate lazy_static;
mod commands;
mod shared;
pub use commands::{
    about, convert, dialog, enlarge, image, meal, owoify, oracle,
    pick, ping, route, ship, time, valentine
};
pub use shared::{
    AUTHENTICATION_SERVICE,
    get_image,
    INTERFACE_SERVICE,
    PERSISTENCE_STORAGE
};