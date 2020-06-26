pub mod authentication;
pub mod image_service;
pub mod interface;
pub mod persistence;
pub use authentication::AUTHENTICATION_SERVICE;
pub use image_service::get_image;
pub use interface::INTERFACE_SERVICE;
pub use persistence::PERSISTENCE_STORAGE;