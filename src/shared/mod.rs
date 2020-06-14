pub mod authentication;
pub mod character;
pub mod persistence;
pub mod validator;
pub use authentication::AUTHENTICATION_SERVICE;
pub use character::Character;
pub use persistence::PERSISTENCE_STORAGE;
pub use validator::validate_dialog;