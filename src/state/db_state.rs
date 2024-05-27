#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ClickHouseState {
    pub DATA: Vec<String>,
    pub STATUS: String,
}

impl ClickHouseState {
    pub fn new() -> Self {
        Self {
            DATA: Vec::new(),
            STATUS: "Initialized".to_string(),
        }
    }
}
