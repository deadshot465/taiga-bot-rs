use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct InterfaceStrings {
    pub cool_down: String,
    pub emote: CommandStrings,
    pub say: HashMap<String, CommandStrings>,
    pub remind: CommandStrings,
}
