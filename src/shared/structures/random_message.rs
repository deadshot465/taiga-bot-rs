use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RandomMessage {
    pub keyword: String,
    pub messages: HashMap<String, Vec<String>>,
    pub reactions: Vec<String>
}