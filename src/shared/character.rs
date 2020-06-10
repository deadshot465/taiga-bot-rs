use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub age: u8,
    pub birthday: String,
    pub animal: String,
    pub color: String,
    pub emote_id: String
}