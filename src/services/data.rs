#![allow(non_snake_case)]
use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

#[allow(unused)]
use crate::{services::prelude::*, state::prelude::WsprSpot};

#[derive(Debug, Deserialize, Serialize)]
pub struct DataService {
    query: Option<String>,
}

impl DataService {
    // [GET] all "general" records from the [rx] table.
    pub async fn GET_SPOT_DATA(
        query: &String,
        limit: String,
        result_format: Option<String>,
    ) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let session = session_manager::SessionManager::new();
        let BASE_URL = session.BASE_URL.trim();

        let query = query_manager::QueryManager::new(query);
        let QUERY_STRING = query.QUERY;

        let FORMAT_OPTIONS = match result_format {
            Some(format) => format,
            None => String::from(""),
        };

        let REQUEST = format!(
            "{}?query={} {} FORMAT {}",
            BASE_URL, QUERY_STRING, limit, FORMAT_OPTIONS
        );

        let response = client
            .get(REQUEST)
            .send()
            .await
            .context("Error sending request!")?;

        //////////////////////////////////////
        ///// Verify [RESPONSE] status. //////
        //////////////////////////////////////
        if response.status().is_success() {
            let wspr_data = response.text().await.context("Error parsing response!")?;

            let parsed_data: serde_json::Value = match serde_json::from_str(&wspr_data) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error parsing JSON: {:?}", e);
                    return Err(anyhow::anyhow!("Error parsing JSON: {:?}", e));
                }
            };

            //////////////////////////////////////////////////////
            ///// Retrieve <data> block from <parsed_data>  /////
            ////////////////////////////////////////////////////
            let data = &parsed_data["data"];

            ///////////////////////////////
            ///// Parse <spot> data. //////
            ///////////////////////////////
            let spots: Vec<WsprSpot> = match serde_json::from_value(data.clone()) {
                Ok(spots) => spots,
                Err(e) => {
                    eprintln!("Error deserializing WsprSpot data: {:?}", e);
                    return Err(anyhow::anyhow!(
                        "Error deserializing WsprSpot data: {:?}",
                        e
                    ));
                }
            };

            println!("\nparsed_data_block{:?} \n\n", spots);

            /*****************************
               [DEBUG] logs.

               dbg!("{}", parsed_data);
               dbg!("{}", data);
               dbg!("{}", wspr_data);

            *********************************/

            Ok(wspr_data)
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    // [GET] record by [Id] from the [rx] table.
    pub fn GET_SPOT_DATA_BY_ID() {
        todo!()
    }
}
