use wspr::state::prelude::*;

fn main() {
    let mut state = ClickHouseClient::init();

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get);
    println!("[OUTPUT]: {:?}", state);

    ClickHouseClient::dispatch(&mut state, ClickHouseAction::GetById(1));
    println!("[OUTPUT]: {:?}", state);
}
