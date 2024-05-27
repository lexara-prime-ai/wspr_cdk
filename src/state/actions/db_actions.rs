#[derive(Debug)]
pub enum ClickHouseActions {
    Get,
    GetById(String),
}
