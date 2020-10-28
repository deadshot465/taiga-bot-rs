use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct QuizQuestion {
    #[serde(rename = "type")]
    pub _type: String,
    pub question: String,
    pub answers: Vec<String>,
    pub wrong: Vec<String>,
}
