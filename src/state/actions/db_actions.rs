/// #[derive(Debug)]
/// pub enum ClickHouseActions {
///    Get,             // Fetch all records.
///    GetById(u64),    // Fetch record by Id.
/// }

#[derive(Debug)]
pub enum ClickHouseActions {
    Get,
    GetById(u64),
}
