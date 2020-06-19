use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct InterfaceStrings {
    pub presence: Vec<String>,
    pub about: CommandStrings,
    pub dialog: CommandStrings,
    pub enlarge: CommandStrings,
    pub meal: CommandStrings,
    pub oracle: CommandStrings,
    pub pick: CommandStrings,
    pub ping: CommandStrings,
    pub route: CommandStrings,
    pub ship: CommandStrings,
    pub valentine: CommandStrings
}

#[derive(Deserialize, Serialize)]
pub struct CommandStrings {
    pub description: String,
    pub usage: String,
    pub errors: HashMap<String, String>,
    pub infos: HashMap<String, String>,
    pub result: String
}