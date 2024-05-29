#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::services::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataService {
    query: Option<String>,
}

impl DataService {
    // [GET] all "general" records from the [rx] table.
    pub fn GET_SPOT_DATA(query: &String, limit: String) {
        let session = session_manager::SessionManager::new();
        let BASE_URL = session.BASE_URL.trim();

        let query = query_manager::QueryManager::new(query);
        let QUERY_STRING = format!("{}", query.QUERY);

        let REQUEST = format!("{}?query={} {}", BASE_URL, QUERY_STRING, limit);

        println!("{}", REQUEST);
    }

    // [GET] record by [Id] from the [rx] table.
    pub fn GET_SPOT_DATA_BY_ID() {
        todo!()
    }
}
