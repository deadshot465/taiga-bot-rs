use serde::{Deserialize, Serialize};

use crate::shared::constants::ASSET_DIRECTORY;

const QUIZ_QUESTIONS_FILE_NAME_KOU: &str = "/game/quiz_kou.json";
const QUIZ_QUESTIONS_FILE_NAME_TAIGA: &str = "/game/quiz_taiga.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuizQuestion {
    #[serde(rename = "type")]
    pub question_type: String,
    pub question: String,
    pub answers: Vec<String>,
    pub wrong: Vec<String>,
}

pub fn initialize_quiz_questions(is_kou: bool) -> Vec<QuizQuestion> {
    let quiz_questions_path = String::from(ASSET_DIRECTORY)
        + (if is_kou {
            QUIZ_QUESTIONS_FILE_NAME_KOU
        } else {
            QUIZ_QUESTIONS_FILE_NAME_TAIGA
        });

    let json = std::fs::read(quiz_questions_path).expect("Failed to read quiz questions.");
    serde_json::from_slice(&json).expect("Failed to deserialize quiz questions.")
}
