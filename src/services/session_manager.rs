#![allow(non_snake_case)]
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionManager {
    pub BASE_URL: String,
    pub SESSION_ID: i64,
}

/// Session initialization.
impl SessionManager {
    pub fn new() -> Self {
        dotenv().ok();

        let timestamp = chrono::Utc::now().timestamp();

        let BASE_URL = std::env::var_os("BASE_URL")
            .expect("[BASE_URL] not found!")
            .into_string()
            .unwrap();

        Self {
            BASE_URL,
            SESSION_ID: timestamp,
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
