# WSPR CDK

`wspr_cdk` provides an abstraction for accessing and analyzing **WSPR** (_Weak Signal Propagation Reporter_) real-time spot data. This crate allows you to perform queries and fetch data from the WSPR database with ease.

## Features

-   Fetch **WSPR** spot data in various formats (**JSON**, **JSONCompact**, **JSONEachRow**)
-   Easy integration with **Tokio** for asynchronous operations
-   Abstractions to manage session state and dispatch actions to the **ClickHouse** client
-   **Server component** for accessing and sharing real-time data via HTTP

### Upcoming Features

-   **Mutual TLS** for secure client-server communications
-   **SSL (OpenSSL)** support for encrypted data transfer

## Installation

To use this crate, add `wspr_cdk` to your `Cargo.toml`:

```toml
[dependencies]
wspr_cdk = "0.0.5"
``` 

## Environment Variable

Before using the crate, ensure you set the following environment variable:

```sh
export BASE_URL=http://db1.wspr.live/
``` 

## Usage

Here are some examples of how to use the `wspr_cdk` crate:

```rust
#![allow(unused)]

use chrono::NaiveDateTime;
use wspr::{services::prelude::*, state::prelude::*};

#[tokio::main]
async fn main() {
    // Initialize the ClickHouse client state
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    // Dispatch a GET request to fetch data in JSON format
    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, "10", "JSON").await;

    // Print the state after fetching data
    println!("\n{:#?}\n", state);

    // Example of fetching data by ID
    ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    println!("\n[OUTPUT]: {:?}", state);

    // Example of serializing response to JSON
    let json_response = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", json_response);
}
``` 

### Example Query

```sh

Copy code
wget -q -O - "http://db1.wspr.live/?query=SELECT * FROM wspr.rx LIMIT 5 FORMAT JSON;"
``` 

### Sample Output

```sh
Fetching all records...

ClickHouseState {
    DATA: [
        WsprSpot {
            id: 7766261671,
            time: 2024-05-29T17:30:00,
            band: -1,
            rx_sign: "F6CWA",
            rx_lat: 48.021,
            rx_lon: -4.125,
            rx_loc: "IN78wa",
            tx_sign: "DL7NN",
            tx_lat: 50.771,
            tx_lon: 12.708,
            tx_loc: "JO60is",
            distance: 1253,
            azimuth: 262,
            rx_azimuth: 69,
            frequency: 137433,
            power: 30,
            snr: -18,
            drift: 0,
            version: "2.6.1",
            code: 2,
        },
    ],
    STATUS: "Fetching all records.",
}
``` 

## Server Component

The server component allows you to access and share real-time WSPR data via HTTP. Below is a snippet of the server component source code:

```rust
#[macro_use]
extern crate rocket;

use anyhow::Error;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::result::Result::{Err, Ok};

// Required [modules].
use wspr_cdk::{services::prelude::*, state::prelude::*};

/// Get all <wspr> spots.
#[get("/api/spots")]
async fn get_wspr_spots() -> Result<Json<Vec<WsprSpot>>, status::Custom<String>> {
    let mut state = ClickHouseClient::init();
    let _session = session_manager::SessionManager::new();

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
    rocket::build().mount("/", routes![get_wspr_spots])
}
``` 

### Sample cURL Request

To fetch WSPR spots using the server component, you can use the following cURL command:

```sh
curl -X GET http://localhost:8000/api/spots
``` 

## Client-Side Usage Example

You can also fetch WSPR data using client-side JavaScript. Here is a sample implementation:

```html
<!DOCTYPE html>
<html>
<head>
    <title>WSPR Spots</title>
</head>
<body>
    <div id="demo"></div>

    <script> const content = document.getElementById("demo");

        async function getData() {
            let response = await fetch("http://localhost:8000/api/spots");
            let raw = await response.json();

            for (let i of raw) {
                console.log(i);
                content.innerHTML += `
                    <h2>Spot id: ${i.id}</h2>
                    <p>Time: ${i.time}</p>
                    <p>Band: ${i.band}</p>
                `;
            }
        }

        getData(); </script>
</body>
</html>
``` 

## WSPR Guidelines

**Disclaimer**: The dataset contains the raw **spot** data as reported, saved, and published by _wsprnet.org_. Therefore, there might be **duplicates**, **false** spots, and other **errors** in the data. Keep this in mind when you see something strange. You are allowed to use the services provided on **wspr.live** for your own research and projects, as long as the results are accessible **free of charge for everyone**. You are **NOT** allowed to use this service for any _**commercial**_ or _**profit-oriented**_ use cases. The complete WSPR infrastructure is maintained by volunteers in their spare time, so there are no guarantees on the correctness, availability, or stability of these services.

## License

This project is licensed under the BSD License. See the LICENSE file for details.

## Contribution

Contributions are **welcome**! Please submit issues or pull requests as needed. Ensure that your contributions comply with the licensing and guidelines set forth.

## Acknowledgments

Special thanks to the WSPR community for providing access to the data and maintaining the infrastructure.

----------

This documentation is also available as a crate on [crates.io](https://crates.io/)