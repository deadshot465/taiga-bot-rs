use once_cell::sync::Lazy;

pub mod credit_service;
pub mod dialog_service;
pub mod image_service;
pub mod judge_zero_service;
pub mod openai_service;
pub mod ship_service;

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);
