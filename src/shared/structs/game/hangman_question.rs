use crate::shared::constants::ASSET_DIRECTORY;
use once_cell::sync::Lazy;

pub static HANGMAN_QUESTIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let hangman_questions_path = String::from(ASSET_DIRECTORY) + HANGMAN_QUESTIONS_FILE_NAME;
    if let Ok(json) = std::fs::read(hangman_questions_path) {
        serde_json::from_slice::<Vec<String>>(&json)
            .expect("Failed to load hangman words from local disk.")
    } else {
        vec![]
    }
});

const HANGMAN_QUESTIONS_FILE_NAME: &str = "/game/words.json";
