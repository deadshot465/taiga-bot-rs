use crate::shared::{Character, Oracle, ShipMessage, ConversionTable, UserRecords, SpecializedInfo};
use std::collections::HashMap;
use std::borrow::Borrow;

const VALID_SPECIALIZED_CHARACTERS: [&'static str; 7] = [
    "hiro", "taiga", "keitaro", "yoichi", "yuri", "kieran", "natsumi"
];

pub static mut PERSISTENCE_STORAGE: PersistenceStorage = PersistenceStorage {
    routes: None,
    valentines: None,
    is_loaded: false,
    dialog_backgrounds: None,
    dialog_characters: None,
    background_strings: String::new(),
    character_strings: String::new(),
    oracles: None,
    ship_messages: None,
    conversion_table: None,
    user_records: None,
    specialized_info: None
};

pub struct PersistenceStorage {
    pub routes: Option<Vec<Character>>,
    pub valentines: Option<Vec<Character>>,
    pub is_loaded: bool,
    pub dialog_backgrounds: Option<Vec<String>>,
    pub dialog_characters: Option<Vec<String>>,
    pub background_strings: String,
    pub character_strings: String,
    pub oracles: Option<Vec<Oracle>>,
    pub ship_messages: Option<Vec<ShipMessage>>,
    pub conversion_table: Option<ConversionTable>,
    pub user_records: Option<HashMap<String, UserRecords>>,
    pub specialized_info: Option<HashMap<String, SpecializedInfo>>
}

impl PersistenceStorage {
    pub async fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_loaded {
            return Ok(());
        }
        let raw_routes = std::fs::read("./persistence/routes.json")?;
        let raw_valentines = std::fs::read("./persistence/valentines.json")?;
        let raw_oracles = std::fs::read("./persistence/oracles.json")?;
        let raw_ship_messages = std::fs::read("./persistence/shipMessages.json")?;
        let raw_conversion_table = std::fs::read("./persistence/convert.json")?;
        let raw_user_records = std::fs::read("./persistence/userRecords.json")?;

        let routes: Vec<Character> = serde_json::from_slice(raw_routes.borrow())?;
        let valentines: Vec<Character> = serde_json::from_slice(raw_valentines.borrow())?;
        let oracles: Vec<Oracle> = serde_json::from_slice(raw_oracles.borrow())?;
        let ship_messages: Vec<ShipMessage> = serde_json::from_slice(raw_ship_messages.borrow())?;
        let conversion_table: ConversionTable = serde_json::from_slice(raw_conversion_table.borrow())?;
        let user_records: HashMap<String, UserRecords> = serde_json::from_slice(raw_user_records.borrow())?;
        self.routes = Some(routes);
        self.valentines = Some(valentines);
        self.oracles = Some(oracles);
        self.ship_messages = Some(ship_messages);
        self.conversion_table = Some(conversion_table);
        self.user_records = Some(user_records);

        self.load_dialog_data().await?;
        self.load_specialized_info().await?;
        self.is_loaded = true;
        Ok(())
    }

    async fn load_dialog_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get("https://tetsukizone.com/api/dialog")
            .await?
            .json::<HashMap<String, Vec<String>>>()
            .await?;
        let dialog_characters: Vec<String> = response["characters"].clone();
        let dialog_backgrounds: Vec<String> = response["backgrounds"].clone();
        self.dialog_characters = Some(dialog_characters);
        self.dialog_backgrounds = Some(dialog_backgrounds);
        self.background_strings = self.dialog_backgrounds.as_ref().unwrap().join(", ");
        self.character_strings = self.dialog_characters.as_ref().unwrap().join(", ");
        Ok(())
    }

    async fn load_specialized_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        self.specialized_info = Some(HashMap::new());
        let specialized_info = self.specialized_info.as_mut().unwrap();
        for character in VALID_SPECIALIZED_CHARACTERS.iter() {
            let response = client.get(format!("https://tetsukizone.com/api/dialog/{}", *character).as_str())
                .send()
                .await?;
            let data: SpecializedInfo = response.json().await?;
            specialized_info.insert(String::from(*character), data);
        }
        Ok(())
    }
}