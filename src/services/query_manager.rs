#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryManager {
    pub QUERY: String,
}

impl QueryManager {
    pub fn new(query: &String) -> Self {
        Self {
            QUERY: query.into(),
        }
    }
}
