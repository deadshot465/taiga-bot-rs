use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Emote {
    pub name: String,
    pub id: u64,
    pub link: String,
    pub raw: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub emotes: Vec<Emote>
}

impl Emote {
    pub fn new(name: &str, id: u64, link: &str, raw: &str) -> Self {
        Emote {
            name: String::from(name),
            id,
            link: String::from(link),
            raw: String::from(raw)
        }
    }
}