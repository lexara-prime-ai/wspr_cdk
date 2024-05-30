#![allow(non_snake_case)]
#![allow(unused)]

use rocket::http::Status;
use rocket::launch;
use rocket::response::{content, status};
use rocket::State;

// [JSON] serialization & deserialization.
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// Required [modules].
use crate::{services::prelude::*, state::prelude::*};

/// Get all <wspr> spots.
#[get("/spots")]
async fn get_wspr_spots() -> Result<Json<Vec<WsprSpot>>, status::Custom<String>> {
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, "1", "JSON").await;

    // [DEBUG] logs.
    dbg!("\n{:#?}\n", state);

    todo!()
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![get_wspr_spots])
}
