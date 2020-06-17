use crate::shared::{Character, Oracle, ShipMessage};
use std::collections::HashMap;
use std::borrow::Borrow;

pub static mut PERSISTENCE_STORAGE: PersistenceContainer = PersistenceContainer{
    instance: None
};

pub struct PersistenceStorage {
    pub routes: Vec<Character>,
    pub valentines: Vec<Character>,
    pub is_loaded: bool,
    pub dialog_backgrounds: Vec<String>,
    pub dialog_characters: Vec<String>,
    pub background_strings: String,
    pub character_strings: String,
    pub oracles: Vec<Oracle>,
    pub ship_messages: Vec<ShipMessage>
}

pub struct PersistenceContainer {
    instance: Option<PersistenceStorage>
}

impl PersistenceStorage {
    async fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_routes = std::fs::read("./persistence/routes.json")?;
        let raw_valentines = std::fs::read("./persistence/valentines.json")?;
        let raw_oracles = std::fs::read("./persistence/oracles.json")?;
        let raw_ship_messages = std::fs::read("./persistence/shipMessages.json")?;

        self.routes = serde_json::from_slice(raw_routes.borrow())?;
        self.valentines = serde_json::from_slice(raw_valentines.borrow())?;
        self.oracles = serde_json::from_slice(raw_oracles.borrow())?;
        self.ship_messages = serde_json::from_slice(raw_ship_messages.borrow())?;

        self.load_dialog_data().await?;
        self.is_loaded = true;
        Ok(())
    }

    async fn load_dialog_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get("https://tetsukizone.com/api/dialog")
            .await?
            .json::<HashMap<String, Vec<String>>>()
            .await?;
        self.dialog_characters = response["characters"].clone();
        self.dialog_backgrounds = response["backgrounds"].clone();
        self.background_strings = self.dialog_backgrounds.join(", ");
        self.character_strings = self.dialog_characters.join(", ");
        Ok(())
    }
}

impl PersistenceContainer {
    pub async fn get_instance(&mut self) -> &PersistenceStorage {
        if self.instance.is_none() {
            self.instance = Some(PersistenceStorage {
                routes: vec![],
                valentines: vec![],
                is_loaded: false,
                dialog_backgrounds: vec![],
                dialog_characters: vec![],
                background_strings: String::new(),
                character_strings: String::new(),
                oracles: vec![],
                ship_messages: vec![]
            });
            if let Some(v) = &mut self.instance {
                v.load().await.expect("Error loading persistence data.");
            }
        }
        self.instance.as_ref().unwrap()
    }
}