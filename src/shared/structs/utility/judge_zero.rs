use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JudgeZeroRequestResult {
    Failed(JudgeZeroGetResponse),
    InProgress,
    Success(JudgeZeroGetResponse),
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct JudgeZeroPostRequest {
    pub language_id: u8,
    pub source_code: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct JudgeZeroPostResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct JudgeZeroGetResponse {
    pub status_id: i16,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub message: Option<String>,
    pub memory: Option<i32>,
    pub time: Option<String>,
    pub compile_output: Option<String>,
}
