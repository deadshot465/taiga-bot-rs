use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*#[derive(Deserialize, Serialize)]
pub struct Lengths {
    pub km: f64,
    pub m: f64,
    pub cm: f64,
    #[serde(rename = "in")]
    pub inch: f64,
    pub ft: f64,
    pub mi: f64,
    pub au: f64
}

#[derive(Deserialize, Serialize)]
pub struct Length {
    pub km: Lengths,
    pub m: Lengths,
    pub cm: Lengths,
    #[serde(rename = "in")]
    pub inch: Lengths,
    pub ft: Lengths,
    pub mi: Lengths,
    pub au: Lengths
}

#[derive(Deserialize, Serialize)]
pub struct Temperatures {
    pub c: f64,
    pub f: f64,
    pub k: f64
}

#[derive(Deserialize, Serialize)]
pub struct Temperature {
    pub c: Temperatures,
    pub f: Temperatures,
    pub k: Temperatures
}*/

#[derive(Deserialize, Serialize)]
pub struct ConversionTable {
    pub length: HashMap<String, HashMap<String, f64>>,
    pub temperature: HashMap<String, HashMap<String, f64>>
}