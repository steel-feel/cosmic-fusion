use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};
 
pub const COMPLETED_ORDERS: Map<String, bool> = Map::new("completed_orders");

#[cw_serde]
pub struct State {
    pub escrow_code_id: u64,
}

pub const STATE: Item<State> = Item::new("state");