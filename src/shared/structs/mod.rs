use std::collections::HashMap;
use std::sync::Arc;

use crate::shared::services::open_router_service::initialize_openai_compatible_client;
use crate::shared::structs::authentication::Authentication;
use crate::shared::structs::config::channel_control::ChannelControl;
use crate::shared::structs::config::common_settings::CommonSettings;
use crate::shared::structs::config::configuration::Configuration;
use crate::shared::structs::config::random_response::RandomResponse;
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
use tokio::sync::RwLock;

pub mod authentication;
pub mod config;
pub mod fun;
pub mod game;
pub mod information;
pub mod record;
pub mod smite;
pub mod utility;

const OPEN_ROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
const VOLC_ENGINE_BASE_URL: &str = "https://ark.cn-beijing.volces.com/api/v3";
const MOONSHOT_BASE_URL: &str = "https://api.moonshot.cn/v1";
const STEP_BASE_URL: &str = "https://api.stepfun.com/v1";
const ZHIPU_BASE_URL: &str = "https://open.bigmodel.cn/api/paas/v4";

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
    pub random_response: RandomResponse,
    pub forged_in_starlight_instructions: String,
    pub chronosplit_instructions: String,
    pub openai_compatible_clients: Arc<OpenAICompatibleClients>,
}

#[derive(Debug, Clone)]
pub struct OpenAICompatibleClients {
    pub open_router_client: async_openai::Client<OpenAIConfig>,
    pub volc_engine_client: async_openai::Client<OpenAIConfig>,
    pub moonshot_client: async_openai::Client<OpenAIConfig>,
    pub step_client: async_openai::Client<OpenAIConfig>,
    pub zhipu_client: async_openai::Client<OpenAIConfig>,
}

pub type ContextError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, ContextData, ContextError>;

impl OpenAICompatibleClients {
    pub fn new(config: &Configuration) -> Self {
        OpenAICompatibleClients {
            open_router_client: initialize_openai_compatible_client(
                OPEN_ROUTER_BASE_URL,
                &config.open_router_api_key,
            ),
            volc_engine_client: initialize_openai_compatible_client(
                VOLC_ENGINE_BASE_URL,
                &config.volc_engine_api_key,
            ),
            moonshot_client: initialize_openai_compatible_client(
                MOONSHOT_BASE_URL,
                &config.moonshot_api_key,
            ),
            step_client: initialize_openai_compatible_client(STEP_BASE_URL, &config.step_api_key),
            zhipu_client: initialize_openai_compatible_client(
                ZHIPU_BASE_URL,
                &config.zhipu_api_key,
            ),
        }
    }
}
