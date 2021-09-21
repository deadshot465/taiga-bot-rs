use crate::shared::structs::ChannelSettings;
use crate::shared::{
    Character, ConversionTable, Oracle, ShipMessage, SpecializedInfo, UserRecords,
};
use crate::{AuthenticationService, Config, QuizQuestion, RandomMessage, Reminder, UserReply};
use chrono::{DateTime, Local, Utc};
use serenity::client::Context;
use serenity::prelude::TypeMapKey;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

const VALID_SPECIALIZED_CHARACTERS: [&str; 10] = [
    "hiro", "taiga", "keitaro", "yoichi", "yuri", "kieran", "natsumi", "hunter", "eduard", "lee",
];

const REMINDER_PATH: &str = "./assets/reminders.json";

pub struct PersistenceStorage {
    pub specialized_info: Option<HashMap<String, SpecializedInfo>>,
    pub last_modified_time: Option<DateTime<Utc>>,
    pub reminders: Option<HashMap<u64, Reminder>>,
    pub user_replies: Option<Vec<UserReply>>,
    pub ongoing_tictactoes: Option<HashSet<u64>>,
}
