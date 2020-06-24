use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SpecializedDialog {
    #[serde(rename = "Background")]
    pub background: String,
    #[serde(rename = "Clothes")]
    pub clothes: String,
    #[serde(rename = "Face")]
    pub face: String,
    #[serde(rename = "IsHiddenCharacter")]
    pub is_hidden_character: bool,
    #[serde(rename = "Pose")]
    pub pose: u8,
    #[serde(rename = "Text")]
    pub text: String
}