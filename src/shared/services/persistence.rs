use crate::shared::structures::ChannelSettings;
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

const USER_RECORDS_PATH: &str = "./persistence/userRecords.json";
const CHANNEL_SETTINGS_PATH: &str = "./persistence/channelSettings.json";
const REMINDER_PATH: &str = "./persistence/reminders.json";
const CONFIG_PATH: &str = "./persistence/config.json";
const SMOTE_USER_PATH: &str = "./persistence/smite.json";
const TAIGA_QUIZ_PATH: &str = "./persistence/game/quiz_taiga.json";
const KOU_QUIZ_PATH: &str = "./persistence/game/quiz_kou.json";

pub struct PersistenceService;
impl TypeMapKey for PersistenceService {
    type Value = Arc<RwLock<PersistenceStorage>>;
}

pub struct PersistenceStorage {
    pub routes: Option<Vec<Character>>,
    pub valentines: Option<Vec<Character>>,
    pub is_loaded: bool,
    pub dialog_backgrounds: Option<Vec<String>>,
    pub dialog_characters: Option<Vec<String>>,
    pub background_strings: String,
    pub character_strings: String,
    pub oracles: Option<Vec<Oracle>>,
    pub ship_messages: Option<Vec<ShipMessage>>,
    pub conversion_table: Option<ConversionTable>,
    pub user_records: Option<HashMap<String, UserRecords>>,
    pub specialized_info: Option<HashMap<String, SpecializedInfo>>,
    pub channel_settings: Option<ChannelSettings>,
    pub random_messages: Option<Vec<RandomMessage>>,
    pub last_modified_time: Option<DateTime<Utc>>,
    pub presence_timer: Option<DateTime<Utc>>,
    pub reminders: Option<HashMap<u64, Reminder>>,
    pub user_replies: Option<Vec<UserReply>>,
    pub game_words: Option<Vec<String>>,
    pub config: Option<Config>,
    pub quiz_questions: Option<Vec<QuizQuestion>>,
    pub ongoing_quizzes: Option<HashSet<u64>>,
    pub ongoing_tictactoes: Option<HashSet<u64>>,
    pub smite_links: Vec<String>,
    pub smote_users: HashMap<u64, DateTime<Local>>,
    pub guide_text: String,
}

impl PersistenceStorage {
    pub async fn new(is_kou: bool) -> anyhow::Result<Self> {
        use std::fs;
        let raw_smite_links = fs::read("./persistence/gif/smite_links.json")?;
        let raw_smote_users = fs::read(SMOTE_USER_PATH)?;
        let smite_links: Vec<String> = serde_json::from_slice(raw_smite_links.as_slice())?;
        let smote_users: HashMap<u64, DateTime<Local>> =
            serde_json::from_slice(raw_smote_users.as_slice())?;

        let mut entity = PersistenceStorage {
            routes: None,
            valentines: None,
            is_loaded: false,
            dialog_backgrounds: None,
            dialog_characters: None,
            background_strings: String::new(),
            character_strings: String::new(),
            oracles: None,
            ship_messages: None,
            conversion_table: None,
            user_records: None,
            specialized_info: None,
            channel_settings: None,
            random_messages: None,
            last_modified_time: None,
            presence_timer: None,
            reminders: None,
            user_replies: None,
            game_words: None,
            config: None,
            quiz_questions: None,
            ongoing_quizzes: None,
            ongoing_tictactoes: None,
            guide_text: String::new(),
            smite_links,
            smote_users,
        };
        entity
            .load(is_kou)
            .await
            .expect("Failed to initialize persistence storage.");
        Ok(entity)
    }

    pub async fn load(&mut self, is_kou: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_loaded {
            return Ok(());
        }

        use std::fs;
        let raw_routes = fs::read("./persistence/routes.json")?;
        let raw_valentines = fs::read("./persistence/valentines.json")?;
        let raw_oracles = fs::read("./persistence/oracles.json")?;
        let raw_ship_messages = fs::read("./persistence/shipMessages.json")?;
        let raw_conversion_table = fs::read("./persistence/convert.json")?;
        let raw_user_records = fs::read(USER_RECORDS_PATH)?;
        let raw_channel_settings = fs::read(CHANNEL_SETTINGS_PATH)?;
        let raw_random_messages = fs::read("./persistence/messages.json")?;
        let raw_reminders = fs::read(REMINDER_PATH)?;
        let raw_user_replies = fs::read("./persistence/userReplies.json")?;
        let raw_words = fs::read("./persistence/game/words.json")?;
        let raw_config = fs::read(CONFIG_PATH)?;

        let raw_quiz_questions: Vec<u8>;
        if is_kou {
            raw_quiz_questions = fs::read(KOU_QUIZ_PATH)?;
            self.guide_text = fs::read_to_string("./persistence/kou_intro.txt")?;
        } else {
            raw_quiz_questions = fs::read(TAIGA_QUIZ_PATH)?;
            self.guide_text = fs::read_to_string("./persistence/taiga_intro.txt")?;
        }

        let routes: Vec<Character> = serde_json::from_slice(raw_routes.borrow())?;
        let valentines: Vec<Character> = serde_json::from_slice(raw_valentines.borrow())?;
        let oracles: Vec<Oracle> = serde_json::from_slice(raw_oracles.borrow())?;
        let ship_messages: Vec<ShipMessage> = serde_json::from_slice(raw_ship_messages.borrow())?;
        let conversion_table: ConversionTable =
            serde_json::from_slice(raw_conversion_table.borrow())?;
        let user_records: HashMap<String, UserRecords> =
            serde_json::from_slice(raw_user_records.borrow())?;
        let random_messages: Vec<RandomMessage> =
            serde_json::from_slice(raw_random_messages.borrow())?;
        let user_replies: Vec<UserReply> = serde_json::from_slice(raw_user_replies.borrow())?;
        let game_words: Vec<String> = serde_json::from_slice(raw_words.borrow())?;
        let config: Config = serde_json::from_slice(raw_config.borrow())?;
        let quiz_questions: Vec<QuizQuestion> =
            serde_json::from_slice(raw_quiz_questions.borrow())?;
        self.routes = Some(routes);
        self.valentines = Some(valentines);
        self.oracles = Some(oracles);
        self.ship_messages = Some(ship_messages);
        self.conversion_table = Some(conversion_table);
        self.user_records = Some(user_records);
        self.random_messages = Some(random_messages);
        self.user_replies = Some(user_replies);
        self.game_words = Some(game_words);
        self.config = Some(config);
        self.quiz_questions = Some(quiz_questions);
        self.ongoing_quizzes = Some(HashSet::new());
        self.ongoing_tictactoes = Some(HashSet::new());

        if !raw_channel_settings.is_empty() {
            let channel_settings: ChannelSettings =
                serde_json::from_slice(raw_channel_settings.borrow())?;
            self.channel_settings = Some(channel_settings);
        } else {
            self.channel_settings = Some(ChannelSettings::new());
        }

        if !raw_reminders.is_empty() {
            let reminders: HashMap<u64, Reminder> = serde_json::from_slice(raw_reminders.borrow())?;
            self.reminders = Some(reminders);
        } else {
            self.reminders = Some(HashMap::new());
        }

        self.load_dialog_data(is_kou).await?;
        self.load_specialized_info().await?;
        self.is_loaded = true;
        self.last_modified_time = Some(Utc::now());
        self.presence_timer = Some(Utc::now());
        Ok(())
    }

    async fn load_dialog_data(&mut self, is_kou: bool) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get("https://tetsukizone.com/api/dialog")
            .await?
            .json::<HashMap<String, Vec<String>>>()
            .await?;
        let dialog_characters: Vec<String> = response["characters"].clone();
        let dialog_backgrounds: Vec<String> = response["backgrounds"].clone();
        self.dialog_characters = Some(dialog_characters);
        self.dialog_backgrounds = Some(dialog_backgrounds);

        if is_kou {
            let characters = self.dialog_characters.as_mut().unwrap();
            characters.push("kou".to_string());
            characters.push("kou2".to_string());
        }

        self.background_strings = self.dialog_backgrounds.as_ref().unwrap().join(", ");
        self.character_strings = self.dialog_characters.as_ref().unwrap().join(", ");
        Ok(())
    }

    async fn load_specialized_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        self.specialized_info = Some(HashMap::new());
        let specialized_info = self.specialized_info.as_mut().unwrap();
        for character in VALID_SPECIALIZED_CHARACTERS.iter() {
            let response = client
                .get(format!("https://tetsukizone.com/api/dialog/{}", *character).as_str())
                .send()
                .await?;
            let data: SpecializedInfo = response.json().await?;
            specialized_info.insert(String::from(*character), data);
        }
        Ok(())
    }

    pub fn write(&self) {
        log::info!("Writing persistence data...");
        let serialized_user_records: Vec<u8> =
            serde_json::to_vec_pretty(self.user_records.as_ref().unwrap()).unwrap();
        let serialized_user_records_data: &[u8] = serialized_user_records.borrow();
        let io_res = std::fs::write(USER_RECORDS_PATH, serialized_user_records_data);
        if let Err(e) = io_res {
            log::error!("Error when writing user records: {:?}", e);
        }

        let serialized_channel_settings: Vec<u8> =
            serde_json::to_vec_pretty(self.channel_settings.as_ref().unwrap()).unwrap();
        let serialized_channel_settings_data: &[u8] = serialized_channel_settings.borrow();
        let io_res = std::fs::write(CHANNEL_SETTINGS_PATH, serialized_channel_settings_data);
        if let Err(e) = io_res {
            log::error!("Error when writing channel settings: {:?}", e);
        }

        let serialized_reminders: Vec<u8> =
            serde_json::to_vec_pretty(self.reminders.as_ref().unwrap()).unwrap();
        let serialized_reminders_data: &[u8] = serialized_reminders.borrow();
        let io_res = std::fs::write(REMINDER_PATH, serialized_reminders_data);
        if let Err(e) = io_res {
            log::error!("Error when writing reminders: {:?}", e);
        }

        let serialized_config: Vec<u8> =
            serde_json::to_vec_pretty(self.config.as_ref().unwrap()).unwrap();
        let serialized_config_data: &[u8] = serialized_config.borrow();
        let io_res = std::fs::write(CONFIG_PATH, serialized_config_data);
        if let Err(e) = io_res {
            log::error!("Error when writing config: {:?}", e);
        }

        let serialized_smote_users = serde_json::to_vec_pretty(&self.smote_users)
            .expect("Failed to serialize smote users to bytes.");
        let io_res = std::fs::write(SMOTE_USER_PATH, serialized_smote_users.as_slice());
        if let Err(e) = io_res {
            log::error!("Error when writing config: {:?}", e);
        }
    }

    pub async fn update_credits(
        &self,
        context: &Context,
        user_id: u64,
        channel_id: u64,
        amount: i16,
        action: &str,
    ) {
        let context_data = context.data.read().await;
        let authentication = context_data.get::<AuthenticationService>().unwrap();
        let mut authentication_lock = authentication.lock().await;
        authentication_lock
            .login()
            .await
            .expect("Failed to login authentication service.");
        let token = authentication_lock.token.clone();
        drop(authentication_lock);
        drop(context_data);

        let client = reqwest::Client::new();
        let response = client
            .patch(&format!(
                "https://tetsukizone.com/api/credit/{}/{}",
                user_id, action
            ))
            .json(&serde_json::json!({
                "Credit": amount,
                "ChannelId": channel_id.to_string()
            }))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token.as_str()))
            .send()
            .await;

        if let Err(e) = response.as_ref() {
            eprintln!(
                "Error when sending patch request to the server: {}",
                e.to_string()
            )
        }
    }
}
