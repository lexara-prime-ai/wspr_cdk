#![allow(non_snake_case)]
use crate::services::prelude::*;
use crate::state::prelude::*;

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
    ) {
        state.reduce(&action);

        match action {
            ClickHouseAction::Get => {
                println!("Fetching all records...");

                let query = "select * from wspr.rx where time > subtractHours(now(), 2) limit";

                // Create [SERVICE] request.
                let spot_data = data::DataService::GET_SPOT_DATA(
                    &query.to_string(),
                    limit.to_string(),
                    Some(result_format.to_string()),
                )
                .await
                .unwrap();

                ///////////////////////////////////////
                ////////// [DEBUG] logs. //////////////
                //// dbg!("{}", spot_data.clone());////
                ///////////////////////////////////////
                ///////////////////////////////////////
                state.DATA = vec![spot_data];
            }
            ClickHouseAction::GetById(id) => {
                println!("Fetching record with Id: {}", id.clone());
                state.DATA = vec![format!("Entry with Id: {}", id.clone())];
            }
        }
    }
}
