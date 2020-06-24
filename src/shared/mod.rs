pub mod authentication;
pub mod character;
pub mod convert_table;
pub mod interface;
pub mod interface_strings;
pub mod image_service;
pub mod oracle;
pub mod persistence;
pub mod ship_message;
pub mod specialized_dialog;
pub mod specialized_info;
pub mod user_records;
pub mod utility;
pub mod validator;
pub use authentication::AUTHENTICATION_SERVICE;
pub use character::Character;
pub use convert_table::ConversionTable;
pub use interface::INTERFACE_SERVICE;
pub use interface_strings::{
    CommandStrings, InterfaceStrings
};
pub use image_service::get_image;
pub use oracle::Oracle;
pub use persistence::PERSISTENCE_STORAGE;
pub use ship_message::ShipMessage;
pub use specialized_dialog::SpecializedDialog;
pub use specialized_info::*;
pub use user_records::UserRecords;
pub use utility::{
    search_user
};
pub use validator::{
    TextError, validate_dialog, validate_text
};