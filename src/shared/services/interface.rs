use crate::shared::InterfaceStrings;
use serenity::prelude::TypeMapKey;
use std::borrow::Borrow;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct InterfaceService;
impl TypeMapKey for InterfaceService {
    type Value = Arc<RwLock<InterfaceStorage>>;
}

const TAIGA_STRING_PATH: &str = "./persistence/interfaceStringsTaiga.json";
const KOU_STRING_PATH: &str = "./persistence/interfaceStringsKou.json";

pub struct InterfaceStorage {
    pub interface_strings: Option<InterfaceStrings>,
    pub is_kou: bool,
    pub prefix: String,
}

impl InterfaceStorage {
    pub fn new() -> Self {
        InterfaceStorage {
            interface_strings: None,
            is_kou: false,
            prefix: String::new(),
        }
    }

    pub fn load(&mut self, kou: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.interface_strings.is_some() {
            return Ok(());
        }

        let raw_strings: Vec<u8>;
        if kou {
            raw_strings = std::fs::read(KOU_STRING_PATH)?;
        } else {
            raw_strings = std::fs::read(TAIGA_STRING_PATH)?;
        }

        let deserialized: InterfaceStrings = serde_json::from_slice(raw_strings.borrow())?;
        self.interface_strings = Some(deserialized);
        self.is_kou = kou;
        self.prefix = if kou { "k!".into() } else { "ta!".into() };

        Ok(())
    }
}

impl Default for InterfaceStorage {
    fn default() -> Self {
        Self::new()
    }
}
