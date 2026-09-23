use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum Novel {
    #[name = "Forged in Starlight"]
    ForgedInStarlight,
    Chronosplit,
}

#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, Default, poise::ChoiceParameter,
)]
#[serde(rename_all = "snake_case")]
pub enum CodexSummaryRequestedLanguage {
    ZhTw,
    JaJp,
    #[default]
    EnUs,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CodexSummaryRequest {
    pub keyword: String,
    pub word_count: i32,
    pub novel: Option<String>,
    pub additional_instructions: Option<String>,
    pub request_language: CodexSummaryRequestedLanguage,
    pub push_to_line: bool,
    pub schedule_polling: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CodexSummaryContainerResponse {
    pub container_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CodexSummaryContainerState {
    pub status: String
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CodexSummaryResponseLogs {
    pub errors: Vec<String>,
    pub outs: Vec<String>,
    pub console: Vec<String>,
    pub ins: Vec<String>,
    pub images: Vec<String>
}