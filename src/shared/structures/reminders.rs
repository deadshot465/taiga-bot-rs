use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Deserialize, Serialize, Debug)]
pub struct Reminder {
    pub datetime: DateTime<Local>,
    pub message: String
}

impl Reminder {
    pub fn new() -> Self {
        Reminder {
            datetime: Local::now(),
            message: String::new()
        }
    }
}