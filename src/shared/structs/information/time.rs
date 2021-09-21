use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TimeData {
    pub datetime: String,
    pub utc_datetime: String,
    pub timezone: String,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Geometry {
    pub location: Location,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct GeocodeResult {
    pub geometry: Geometry,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GeocodeResponse {
    pub results: Vec<GeocodeResult>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TimezoneResponse {
    #[serde(rename = "dstOffset")]
    pub dst_offset: i32,
    #[serde(rename = "rawOffset")]
    pub raw_offset: i32,
    pub status: String,
    #[serde(rename = "timeZoneId")]
    pub time_zone_id: String,
    #[serde(rename = "timeZoneName")]
    pub time_zone_name: String,
}
