use crate::shared::constants::ASSET_DIRECTORY;
use crate::shared::structs::config::configuration::KOU;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

const QUIZ_QUESTIONS_FILE_NAME_KOU: &str = "/game/quiz_kou.json";
const QUIZ_QUESTIONS_FILE_NAME_TAIGA: &str = "/game/quiz_taiga.json";

pub static QUIZ_QUESTIONS: Lazy<Vec<QuizQuestion>> = Lazy::new(|| {
    let is_kou = KOU.get().copied().unwrap_or(false);

    let quiz_questions_path = String::from(ASSET_DIRECTORY)
        + (if is_kou {
            QUIZ_QUESTIONS_FILE_NAME_KOU
        } else {
            QUIZ_QUESTIONS_FILE_NAME_TAIGA
        });

    let json = std::fs::read(quiz_questions_path).expect("Failed to read quiz questions.");
    serde_json::from_slice(&json).expect("Failed to deserialize quiz questions.")
});

#[derive(Deserialize, Serialize, Clone)]
pub struct QuizQuestion {
    #[serde(rename = "type")]
    pub question_type: String,
    pub question: String,
    pub answers: Vec<String>,
    pub wrong: Vec<String>,
}
