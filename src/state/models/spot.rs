use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WsprSpot {
    pub id: u64,
    pub time: chrono::NaiveDateTime,
    pub band: i16,
    pub rx_sign: String,
    pub rx_lat: f32,
    pub rx_lon: f32,
    pub rx_loc: String,
    pub tx_sign: String,
    pub tx_lat: f32,
    pub tx_lon: f32,
    pub tx_loc: String,
    pub distance: u16,
    pub azimuth: u16,
    pub rx_azimuth: u16,
    pub frequency: u32,
    pub power: i8,
    pub snr: i8,
    pub drift: i8,
    pub version: String,
    pub code: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub meta: Vec<Meta>,
    pub data: Vec<WsprSpot>,
    pub rows: usize,
    pub rows_before_limit_at_least: usize,
    pub statistics: Statistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub elapsed: f64,
    pub rows_read: usize,
    pub bytes_read: usize,
}
