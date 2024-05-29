use crate::services::prelude::*;
use crate::state::prelude::*;

pub struct ClickHouseClient;

impl ClickHouseClient {
    pub fn init() -> ClickHouseState {
        ClickHouseState::new()
    }

    pub fn dispatch(state: &mut ClickHouseState, action: ClickHouseAction, limit: String) {
        state.reduce(&action);

        match action {
            ClickHouseAction::Get => {
                println!("Fetching all records...");

                let query = "select * from wspr.rx where time > subtractHours(now(), 2) limit";

                // Create [SERVICE] request.
                data::DataService::GET_SPOT_DATA(&query.to_string(), limit);

                state.DATA = vec!["Entry1".to_string(), "Entry2".to_string()];
            }
            ClickHouseAction::GetById(id) => {
                println!("Fetching record with Id: {}", id.clone());
                state.DATA = vec![format!("Entry with Id: {}", id.clone())];
            }
        }
    }
}
