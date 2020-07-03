use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Comic {
    Dialog(Dialog), SpecializedDialog(SpecializedDialog)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dialog {
    #[serde(rename = "Background")]
    pub background: String,
    #[serde(rename = "Character")]
    pub character: String,
    #[serde(rename = "Text")]
    pub text: String
}

#[derive(Deserialize, Serialize, Debug)]
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
    pub text: String
}