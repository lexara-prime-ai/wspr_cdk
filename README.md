# WSPR CDK

`wspr_cdk` provides an abstraction for accessing and analyzing **WSPR** (*Weak Signal Propagation Reporter*) real-time spot data. This crate allows you to perform queries and fetch data from the WSPR database with ease.

## Features

-   Fetch **WSPR** spot data in various formats (**JSON**, **JSONCompact**, **JSONEachRow**)
-   Easy integration with **Tokio** for asynchronous operations
-   Abstractions to manage session state and dispatch actions to the **ClickHouse** client

## Installation

To use this crate, add `wspr_cdk` to your `Cargo.toml`:

```toml
[dependencies]
wspr-client-rust = "0.1.0"
``` 

## Environment Variable

Before using the crate, ensure you set the following environment variable:

```sh
export BASE_URL=http://db1.wspr.live/
``` 

## Usage

Here's an example of how to use the `wspr_cdk` crate:

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

    // Example of fetching data by ID (commented out)
    // ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    // println!("\n[OUTPUT]: {:?}", state);

    // Example of serializing response to JSON (commented out)
    // let json_response = serde_json::to_string_pretty(&response).unwrap();
    // println!("{}", json_response);
}
``` 

### Example Query

```sh
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

## WSPR Guidelines

**Disclaimer**: The data shown here is the raw data as reported, saved, and published by *wsprnet.org*. Therefore, there might be **duplicates**, **false** spots, and other **errors** in the data. Keep this in mind when you see something strange. You are allowed to use the services provided on **wspr.live** for your own research and projects, as long as the results are accessible **free of charge for everyone**. You are **NOT** allowed to use this service for any ***commercial*** or ***profit-oriented*** use cases. The complete WSPR infrastructure is maintained by volunteers in their spare time, so there are no guarantees on the correctness, availability, or stability of these services.

## License

This project is licensed under the BSD License. See the LICENSE file for details.

## Contribution

Contributions are **welcome**! Please submit issues or pull requests as needed. Ensure that your contributions comply with the licensing and guidelines set forth.

## Acknowledgments

Special thanks to the WSPR community for providing access to the data and maintaining the infrastructure.

----------

This documentation is also available as a crate on **crates.io**.