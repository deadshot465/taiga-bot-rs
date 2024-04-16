use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageInfo {
    pub bot_id: String,
    pub user_id: String,
    pub user_name: Option<String>,
    pub generated_by: Option<String>,
    pub message: String,
    pub message_type: String,
    pub channel_id: String,
    pub post_at: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageRecordSimple {
    pub user_id: String,
    pub user_name: String,
    pub message: String,
    pub message_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompletionRecordSimple {
    pub message_type: String,
    pub message: String,
    pub generated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCompletionRequest {
    pub bot_id: String,
    pub user_id: String,
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCompletionResponse {
    pub bot_id: String,
    pub user_id: String,
    pub messages: Vec<CompletionRecordSimple>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMessageRequest {
    pub bot_id: String,
    pub channel_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMessageResponse {
    pub bot_id: String,
    pub messages: Vec<MessageRecordSimple>,
}
