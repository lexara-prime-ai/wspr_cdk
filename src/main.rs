use chrono::NaiveDateTime;
use wspr::state::prelude::*;

fn main() {
    let mut state = ClickHouseClient::init();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get);
    println!("[OUTPUT]: {:?}", state);

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    println!("[OUTPUT]: {:?}", state);

    let spot = WsprSpot {
        id: 1,
        time: NaiveDateTime::parse_from_str("2023-05-26 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap(),
        band: 1,
        rx_sign: "RXCALL".to_string(),
        rx_lat: 52.5200,
        rx_lon: 13.4050,
        rx_loc: "JO62".to_string(),
        tx_sign: "TXCALL".to_string(),
        tx_lat: 40.7128,
        tx_lon: -74.0060,
        tx_loc: "FN31".to_string(),
        distance: 5000,
        azimuth: 180,
        rx_azimuth: 360,
        frequency: 14097000,
        power: 10,
        snr: -15,
        drift: 0,
        version: "v1.0".to_string(),
        code: 1,
    };

    println!("[OUTPUT]: {:#?}", spot);
}
