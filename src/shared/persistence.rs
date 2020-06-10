use crate::shared::Character;

pub static mut PERSISTENCE_STORAGE: PersistenceContainer = PersistenceContainer{
    instance: None
};

pub struct PersistenceStorage {
    pub routes: Vec<Character>,
    pub valentines: Vec<Character>,
    pub is_loaded: bool
}

pub struct PersistenceContainer {
    instance: Option<PersistenceStorage>
}

impl PersistenceStorage {
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_routes = std::fs::read("./persistence/routes.json")?;
        let raw_valentines = std::fs::read("./persistence/valentines.json")?;
        let routes_data = String::from_utf8(raw_routes)?;
        let valentines_data = String::from_utf8(raw_valentines)?;
        let routes_str = routes_data.as_str();
        let valentines_str = valentines_data.as_str();
        self.routes = serde_json::from_str(routes_str)?;
        self.valentines = serde_json::from_str(valentines_str)?;
        self.is_loaded = true;
        Ok(())
    }
}

impl PersistenceContainer {
    pub fn get_instance(&mut self) -> &PersistenceStorage {
        if self.instance.is_none() {
            self.instance = Some(PersistenceStorage{
                routes: vec![],
                valentines: vec![],
                is_loaded: false
            });
            if let Some(v) = &mut self.instance {
                v.load().expect("Error loading routes and valentines data.");
            }
        }
        self.instance.as_ref().unwrap()
    }
}