use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct AvailableSpecializedOptions {
    #[serde(rename = "Clothes")]
    pub clothes: Vec<String>,
    #[serde(rename = "Faces")]
    pub faces: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpecializedInfo {
    #[serde(rename = "Poses")]
    pub poses: HashMap<String, AvailableSpecializedOptions>,
}
