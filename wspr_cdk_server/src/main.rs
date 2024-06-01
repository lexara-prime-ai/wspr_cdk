#![allow(non_snake_case)]
#![allow(unused)]

#[macro_use]
extern crate rocket;

////////////////////////////////////
/////// [CORS] dependencies ////////
////////////////////////////////////
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

//////////////////////////////////////
/////// [rocket] dependencies ////////
//////////////////////////////////////
use rocket::http::Status;
use rocket::launch;
use rocket::response::{content, status};
use rocket::State;

// [JSON] serialization & deserialization.
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// Required [modules].
use wspr_cdk::{services::prelude::*, state::prelude::*};

// [CORS] setup.
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add [CORS] headers to <responses>",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        res.set_header(Header::new(
            "Content-Security-Policy",
            "default-src 'self'; connect-src 'self' http://localhost:8000",
        ));
    }
}

// Handle [RESPONSE] to preflight [REQUESTS].
#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {}

#[catch(500)]
fn internal_error() -> &'static str {
    "Internal Server Error. Please try again later."
}

#[catch(404)]
fn not_found() -> &'static str {
    "Resource not found. Please check the URL."
}

/// Get all <wspr> spots.
#[get("/api/spots")]
async fn get_wspr_spots() -> Result<Json<Vec<WsprSpot>>, status::Custom<String>> {
    let mut state = ClickHouseClient::init();
    let _session = session_manager::SessionManager::new();
    /////////////////////////////////////
    /////////// [DEBUG] logs. ///////////
    //// dbg!("\n{:#?}\n", state); //////
    /////////////////////////////////////
    match ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, "10", "JSON").await {
        Ok(Some(spots)) => Ok(Json(spots)),
        Ok(None) => Err(status::Custom(Status::NotFound, "No spots found".into())),
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Failed to fetch spots: {:?}", e),
        )),
    }
}

#[launch]
#[rocket::main]
async fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![options, get_wspr_spots])
        .register("/", catchers![internal_error, not_found])
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
