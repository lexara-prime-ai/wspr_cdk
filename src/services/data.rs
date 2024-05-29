#![allow(non_snake_case)]
use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

use crate::services::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataService {
    query: Option<String>,
}

impl DataService {
    // [GET] all "general" records from the [rx] table.
    pub async fn GET_SPOT_DATA(query: &String, limit: String) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let session = session_manager::SessionManager::new();
        let BASE_URL = session.BASE_URL.trim();

        let query = query_manager::QueryManager::new(query);
        let QUERY_STRING = format!("{}", query.QUERY);

        let REQUEST = format!("{}?query={} {}", BASE_URL, QUERY_STRING, limit);

        let response = client
            .get(REQUEST)
            .send()
            .await
            .context("Error sending request!")?;

        // println!("{}", REQUEST);
        if response.status().is_success() {
            let wspr_data = response.text().await.context("Error parsing response!")?;
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
