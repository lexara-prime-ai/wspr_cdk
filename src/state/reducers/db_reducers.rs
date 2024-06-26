use crate::state::prelude::*;

pub trait Reducer {
    fn reduce(&mut self, action: &ClickHouseAction);
}

impl Reducer for ClickHouseState {
    fn reduce(&mut self, action: &ClickHouseAction) {
        match action {
            ClickHouseAction::Get => {
                self.STATUS = "Fetching all records.".to_string();
            }
            ClickHouseAction::GetById(id) => {
                self.STATUS = format!("Fetching records with Id: {}", id);
            }
        }
    }
}
