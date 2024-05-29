use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DataService {
    query: Option<String>,
}

impl DataService {
    pub fn get_spot_data() {}
}
