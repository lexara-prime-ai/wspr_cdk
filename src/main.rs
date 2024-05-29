/*
   <result_format> [options]:
       JSON
       JSONCompact
       JSONEachRow
*/

#![allow(unused)]

use chrono::NaiveDateTime;
use wspr::{services::prelude::*, state::prelude::*};

#[tokio::main]
async fn main() {
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, "2", "JSON").await;

    // println!("\n{:#?}\n", state);

    // ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    // println!("\n[OUTPUT]: {:?}", state);

    // let json_response = serde_json::to_string_pretty(&response).unwrap();
    // println!("{}", json_response);
}

/*

   [QUERY]

   wget -q -O - "http://db1.wspr.live/?query=SELECT * FROM wspr.rx LIMIT 5 FORMAT JSON;"

*/
