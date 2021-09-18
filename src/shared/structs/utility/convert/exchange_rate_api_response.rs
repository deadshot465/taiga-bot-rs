use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ExchangeRateAPIResponse {
    pub success: bool,
    pub timestamp: u64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f32>,
}
