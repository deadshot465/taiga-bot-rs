use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ShipMessage {
    pub max_score: u8,
    pub message: String
}