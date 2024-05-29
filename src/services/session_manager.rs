use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionManager {
    pub BASE_URL: String,
    pub SESSION_ID: i64,
}

impl SessionManager {
    pub fn new() -> Self {
        dotenv().ok();

        let timestamp = chrono::Utc::now().timestamp();
        #[allow(non_snake_case)]
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
