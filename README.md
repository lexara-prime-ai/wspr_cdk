
# WSPR CDK

`wspr_cdk` makes it easy to access and analyze **WSPR** (_Weak Signal Propagation Reporter_) real-time spot data from the WSPR database.

## Prerequisites

Ensure the `service_account.json` file is correctly set up for Google Drive API authentication. If missing or misconfigured, you will encounter authentication errors.

### Setup Guide:

1.  **Create a Service Account**:

    -   Go to the Google Cloud Console.
    -   Navigate to IAM & Admin > Service Accounts.
    -   Click "Create Service Account", fill out the details, and click "Create".
2.  **Generate a JSON Key**:

    -   Click on the created service account.
    -   Go to the "Keys" tab.
    -   Click "Add Key" > "Create new key" > **JSON**. This downloads a JSON file with your credentials.
3.  **Provide Permissions**:

    -   Assign the necessary roles to the service account for Google Drive access.
4.  **Configure Environment**:

    -   Place `service_account.json` in an accessible location for your application.
    -   If using Docker, mount `service_account.json` into the container at runtime.

### Running in a Development Container

To run `wspr_cdk` in a **Development Container**, use Docker. Mount the `service_account.json` file securely to provide credentials at runtime:

```sh
`sudo docker run -it -p 8000:8000 -e GOOGLE_APPLICATION_CREDENTIALS=/wspr_cdk/service_account.json -v ./service_account.json:/wspr_cdk/service_account.json wspr_cdk python ./hyper/hyper/server.py --interval 10 --num_rows 10
```

## Features

-   Fetch WSPR spot data in formats: **JSON**, **JSONCompact**, **JSONEachRow**
-   Integrate with **Tokio** for asynchronous operations
-   Manage session state and dispatch actions to the **ClickHouse** client
-   **Server component** for sharing real-time data via HTTP

### Upcoming Features

-   **Mutual TLS** for secure communications
-   **SSL (OpenSSL)** support for encrypted data transfer

## Installation

Add `wspr_cdk` to your `Cargo.toml`:

```toml
[dependencies]
wspr_cdk = "0.0.12"
```

Set the environment variable:

```sh
export BASE_URL=http://db1.wspr.live/
```

## Usage

Run the **Python** server:

```sh
docker run -it wspr_cdk python ./hyper/hyper/server.py --interval 5 --num_rows 5
```

Run the **Rust** server:

```sh
docker run -e ROCKET_ADDRESS=0.0.0.0 -e ROCKET_PORT=8000 -it wspr_cdk rust
```

### Example Rust Code

```rust
use chrono::NaiveDateTime;
use wspr::{services::prelude::*, state::prelude::*};

#[tokio::main]
async fn main() {
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, "10", "JSON").await;
    println!("\n{:#?}\n", state);

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    println!("\n[OUTPUT]: {:?}", state);

    let json_response = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", json_response);
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
            ...
        },
    ],
    STATUS: "Fetching all records.",
}
```

## Server Component

Access and share real-time WSPR data via HTTP. Example cURL request:

```sh
curl -X GET http://localhost:8000/api/spots
```

## Client-Side Usage Example

Fetch WSPR data using client-side JavaScript:

```html
<!doctype html>
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
````

## WSPR Guidelines

**Disclaimer**: The dataset may contain duplicates, false spots, and other errors. Use the services provided on **wspr.live** for non-commercial, free research projects. No guarantees on the correctness, availability, or stability of these services.

## License

This project is licensed under the BSD License. See the LICENSE file for details.

## Contribution

Contributions are welcome! Please submit issues or pull requests. Ensure compliance with the licensing and guidelines.

## Acknowledgments

Special thanks to the WSPR community for data access and infrastructure maintenance.

## Docker Image

The `wspr_cdk` is available on Docker Hub:

```sh
docker pull lexaraprime/wspr_cdk:master
```

Documentation is also available on [crates.io](https://crates.io/).