pub mod authentication;
pub mod character;
pub mod oracle;
pub mod persistence;
pub mod ship_message;
pub mod utility;
pub mod validator;
pub use authentication::AUTHENTICATION_SERVICE;
pub use character::Character;
pub use oracle::Oracle;
pub use persistence::PERSISTENCE_STORAGE;
pub use ship_message::ShipMessage;
pub use utility::{
    search_user
};
pub use validator::validate_dialog;