use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
