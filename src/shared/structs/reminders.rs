use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Reminder {
    pub datetime: DateTime<Local>,
    pub message: String,
}

impl Reminder {
    pub fn new() -> Self {
        Reminder {
            datetime: Local::now(),
            message: String::new(),
        }
    }
}

impl Default for Reminder {
    fn default() -> Self {
        Self::new()
    }
}
