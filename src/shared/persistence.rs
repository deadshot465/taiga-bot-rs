use crate::shared::Character;
use std::collections::HashMap;

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
    pub character_strings: String
}

pub struct PersistenceContainer {
    instance: Option<PersistenceStorage>
}

impl PersistenceStorage {
    async fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_routes = std::fs::read("./persistence/routes.json")?;
        let raw_valentines = std::fs::read("./persistence/valentines.json")?;
        let routes_data = String::from_utf8(raw_routes)?;
        let valentines_data = String::from_utf8(raw_valentines)?;
        let routes_str = routes_data.as_str();
        let valentines_str = valentines_data.as_str();
        self.routes = serde_json::from_str(routes_str)?;
        self.valentines = serde_json::from_str(valentines_str)?;
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
            self.instance = Some(PersistenceStorage{
                routes: vec![],
                valentines: vec![],
                is_loaded: false,
                dialog_backgrounds: vec![],
                dialog_characters: vec![],
                background_strings: String::new(),
                character_strings: String::new()
            });
            if let Some(v) = &mut self.instance {
                v.load().await.expect("Error loading routes and valentines data.");
            }
        }
        self.instance.as_ref().unwrap()
    }
}