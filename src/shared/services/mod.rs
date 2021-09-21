use once_cell::sync::Lazy;

pub mod dialog_service;
pub mod image_service;
pub mod ship_service;
//pub mod persistence;

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
