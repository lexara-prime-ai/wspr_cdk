use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsprSpot {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    #[serde(deserialize_with = "parse_time")]
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

/**********************************************************/
fn deserialize_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let id_str: String = Deserialize::deserialize(deserializer)?;
    id_str.trim().parse::<u64>().map_err(de::Error::custom)
}
/**********************************************************/
/*---------------------------------------------------------*/
/**********************************************************/
fn parse_time<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(serde::de::Error::custom)
}

/**********************************************************/

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
