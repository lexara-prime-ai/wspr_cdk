use crate::state::prelude::*;

pub struct ClickHouseClient;

impl ClickHouseClient {
    pub fn init() -> ClickHouseState {
        ClickHouseState::new()
    }

    pub fn dispatch(state: &mut ClickHouseState, action: ClickHouseAction) {
        state.reduce(&action);

        match action {
            ClickHouseAction::Get => {
                println!("Fetching all records...");
                state.DATA = vec!["Entry1".to_string(), "Entry2".to_string()];
            }
            ClickHouseAction::GetById(id) => {
                println!("Fetching record with Id: {}", id.clone());
                state.DATA = vec![format!("Entry with Id: {}", id.clone())];
            }
        }
    }
}
