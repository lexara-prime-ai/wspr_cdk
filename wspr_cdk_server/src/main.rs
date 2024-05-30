#![allow(non_snake_case)]

#[macro_use]
extern crate wspr_cdk;

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::launch;
use rocket::response::{content, status};
use rocket::State;

// [JSON] serialization & deserialization.
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// Required [modules].
use wspr_cdk::{services::prelude::*, state::prelude::*};

/// Get all <wspr> spots.
#[get("/spots")]
async fn get_wspr_spots() {
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, "1", "JSON").await;

    // [DEBUG] logs.
    dbg!("\n{:#?}\n", state);

    // todo!()
}

#[launch]
#[rocket::main]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![get_wspr_spots])
}

/*
#[rocket::main]
async fn main() {
    tokio::spawn(async {
        let start = Instant::now();
        let mut interval = interval_at(start, tokio::time::Duration::from_secs(5));

        loop {
            interval.tick().await;
            println!("Other scheduled work");
        }
    });

    let config = Config {
        port: 8001,
        address: std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Config::debug_default()
    };

    rocket::custom(&config)
        .mount("/", rocket::routes![test])
        .launch()
        .await;
}
*/
