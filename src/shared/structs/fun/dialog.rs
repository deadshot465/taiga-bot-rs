use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Comic {
    Dialog(Dialog),
    SpecializedDialog(SpecializedDialog),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dialog {
    pub background: String,
    pub character: String,
    pub text: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SpecializedDialog {
    #[serde(rename = "Background")]
    pub background: String,
    #[serde(rename = "Character")]
    pub character: Option<String>,
    #[serde(rename = "Clothes")]
    pub clothes: String,
    #[serde(rename = "Face")]
    pub face: String,
    #[serde(rename = "IsHiddenCharacter")]
    pub is_hidden_character: bool,
    #[serde(rename = "Pose")]
    pub pose: u8,
    #[serde(rename = "Text")]
    pub text: String,
}
