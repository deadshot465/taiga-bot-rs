use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Oracle {
    pub no: u8,
    pub fortune: String,
    pub meaning: String,
    pub content: String
}