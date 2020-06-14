#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate lazy_static;
mod commands;
mod shared;
pub use commands::enlarge;
pub use commands::ping;
pub use commands::route;
pub use commands::valentine;
pub use shared::AUTHENTICATION_SERVICE;
pub use shared::PERSISTENCE_STORAGE;