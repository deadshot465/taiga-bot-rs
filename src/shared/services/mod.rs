use once_cell::sync::Lazy;

//pub mod authentication;
//pub mod dialog_service;
//pub mod image_service;
//pub mod interface;
//pub mod persistence;

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());
