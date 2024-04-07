use crate::shared::structs::authentication::Authentication;
use crate::shared::structs::config::channel_control::ChannelControl;
use crate::shared::structs::config::common_settings::CommonSettings;
use crate::shared::structs::config::configuration::Configuration;
use crate::shared::structs::config::server_info::ServerInfos;
use crate::shared::structs::fun::emote::EmoteList;
use crate::shared::structs::fun::qotd::QotdInfos;
use crate::shared::structs::fun::ship_message::ShipMessage;
use crate::shared::structs::game::quiz_question::QuizQuestion;
use crate::shared::structs::information::character::Character;
use crate::shared::structs::information::oracle::Oracle;
use crate::shared::structs::record::user_record::UserRecord;
use crate::shared::structs::smite::Smite;
use crate::shared::structs::utility::convert::conversion_table::ConversionTable;
use async_openai::config::OpenAIConfig;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod authentication;
pub mod config;
pub mod fun;
pub mod game;
pub mod information;
pub mod record;
pub mod smite;
pub mod utility;

#[derive(Debug, Clone)]
pub struct ContextData {
    pub config: Configuration,
    pub kou: bool,
    pub channel_control: Arc<RwLock<ChannelControl>>,
    pub user_records: Arc<RwLock<HashMap<String, UserRecord>>>,
    pub routes: Vec<Character>,
    pub valentines: Vec<Character>,
    pub oracles: Vec<Oracle>,
    pub http_client: Client,
    pub conversion_table: ConversionTable,
    pub authentication: Arc<RwLock<Authentication>>,
    pub emote_list: Arc<RwLock<EmoteList>>,
    pub server_infos: ServerInfos,
    pub qotd_infos: Arc<RwLock<QotdInfos>>,
    pub common_settings: CommonSettings,
    pub ship_messages: Vec<ShipMessage>,
    pub quiz_questions: Vec<QuizQuestion>,
    pub smite: Smite,
    pub openai_client: async_openai::Client<OpenAIConfig>,
}

pub type ContextError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, ContextData, ContextError>;
