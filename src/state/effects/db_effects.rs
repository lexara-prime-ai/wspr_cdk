#![allow(non_snake_case)]
use crate::services::prelude::*;
use crate::state::prelude::*;
use anyhow::{Context, Error};

// use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClickHouseClient;

impl ClickHouseClient {
    pub fn init() -> ClickHouseState {
        ClickHouseState::new()
    }

    pub async fn dispatch(
        state: &mut ClickHouseState,
        action: ClickHouseAction,
        limit: &str,
        result_format: &str,
    ) -> Result<Option<Vec<WsprSpot>>, Error> {
        state.reduce(&action);

        match action {
            ClickHouseAction::Get => {
                println!("Fetching all records...");

                /*
                    A <String> is a growable buffer. Its purpose is to be modifiable.
                    Why would you pass around a <String> if it's meant to be constant?
                    Remember that API should usually take a <&str> as argument, not a <String>.
                    The right constant type is <&str>. – Denys Séguret

                    Hence,

                    const QUERY: &str =
                    "select * from wspr.rx where time > subtractHours(now(), 12) limit";
                */

                const QUERY: &str =
                    "select * from wspr.rx where time > subtractHours(now(), 35040) limit";

                // Create [SERVICE] request.
                let result = data::DataService::GET_SPOT_DATA(
                    &QUERY.to_string(),
                    limit.to_string(),
                    Some(result_format.to_string()),
                )
                .await;

                match result {
                    Ok(spot_data) => {
                        if spot_data.is_empty() {
                            println!("No data found");
                            state.DATA = None;
                            Ok(None)
                        } else {
                            // WsprSpot now implements the [Clone] trait.
                            // Move spot_data into state.DATA and return a reference to it.
                            let result = Some(spot_data);
                            state.DATA = result.clone();
                            Ok(result)
                        }
                    }
                    Err(e) => {
                        println!("Error fetching data: {:?}", e);
                        state.DATA = None;
                        Err(e).context("Failed to fetch spot data")
                    }
                }
            }
            #[allow(unreachable_code)]
            ClickHouseAction::GetById(id) => {
                println!("Fetching record with Id: {}", id.clone());
                state.DATA = todo!();
                Ok(None) // Placeholder until implementation is done
            }
        }
    }
}
