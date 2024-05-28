use chrono::NaiveDateTime;
use wspr::state::prelude::*;

fn main() {
    let mut state = ClickHouseClient::init();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get);
    println!("\n[OUTPUT]: {:?}", state);

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    println!("\n[OUTPUT]: {:?}", state);

    let spots = vec![
        WsprSpot {
            id: 7126367466,
            time: NaiveDateTime::parse_from_str("2001-02-16 09:14:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            band: 7,
            rx_sign: "JE1JDL".to_string(),
            rx_lat: 36.479,
            rx_lon: 138.958,
            rx_loc: "PM96".to_string(),
            tx_sign: "K6MCS".to_string(),
            tx_lat: 38.688,
            tx_lon: -121.375,
            tx_loc: "CM98hq".to_string(),
            distance: 8291,
            azimuth: 305,
            rx_azimuth: 52,
            frequency: 7040060,
            power: 37,
            snr: -20,
            drift: 0,
            version: "1.9.1".to_string(),
            code: 1,
        },
        // Add more WsprSpot instances as needed
    ];

    let meta = vec![
        Meta {
            name: "id".to_string(),
            field_type: "UInt64".to_string(),
        },
        Meta {
            name: "time".to_string(),
            field_type: "DateTime".to_string(),
        },
        Meta {
            name: "band".to_string(),
            field_type: "Int16".to_string(),
        },
        Meta {
            name: "rx_sign".to_string(),
            field_type: "LowCardinality(String)".to_string(),
        },
        Meta {
            name: "rx_lat".to_string(),
            field_type: "Float32".to_string(),
        },
        Meta {
            name: "rx_lon".to_string(),
            field_type: "Float32".to_string(),
        },
        Meta {
            name: "rx_loc".to_string(),
            field_type: "LowCardinality(String)".to_string(),
        },
        Meta {
            name: "tx_sign".to_string(),
            field_type: "LowCardinality(String)".to_string(),
        },
        Meta {
            name: "tx_lat".to_string(),
            field_type: "Float32".to_string(),
        },
        Meta {
            name: "tx_lon".to_string(),
            field_type: "Float32".to_string(),
        },
        Meta {
            name: "tx_loc".to_string(),
            field_type: "LowCardinality(String)".to_string(),
        },
        Meta {
            name: "distance".to_string(),
            field_type: "UInt16".to_string(),
        },
        Meta {
            name: "azimuth".to_string(),
            field_type: "UInt16".to_string(),
        },
        Meta {
            name: "rx_azimuth".to_string(),
            field_type: "UInt16".to_string(),
        },
        Meta {
            name: "frequency".to_string(),
            field_type: "UInt32".to_string(),
        },
        Meta {
            name: "power".to_string(),
            field_type: "Int8".to_string(),
        },
        Meta {
            name: "snr".to_string(),
            field_type: "Int8".to_string(),
        },
        Meta {
            name: "drift".to_string(),
            field_type: "Int8".to_string(),
        },
        Meta {
            name: "version".to_string(),
            field_type: "LowCardinality(String)".to_string(),
        },
        Meta {
            name: "code".to_string(),
            field_type: "Int8".to_string(),
        },
    ];

    let statistics = Statistics {
        elapsed: 0.047260856,
        rows_read: 74,
        bytes_read: 3570,
    };

    let response = Response {
        meta,
        data: spots,
        rows: 5,
        rows_before_limit_at_least: 74,
        statistics,
    };

    let json_response = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", json_response);
}

/*

   [QUERY]

   wget -q -O - "http://db1.wspr.live/?query=SELECT * FROM wspr.rx LIMIT 5 FORMAT JSON;"

*/
