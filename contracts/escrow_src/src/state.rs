use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Immutables {
    pub order_hash: Vec<u8>,
    pub hashlock: Vec<u8>,
    pub maker: Addr,
    pub taker: Addr,
    pub token: Coin,
    pub timelocks: Timelocks,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Timelocks {
    pub dest_withdrawal: u64,
    pub dest_public_withdrawal: u64,
    pub dest_cancellation: u64,
    pub src_cancellation: u64,
    pub src_withdrawal:u64,
    pub src_public_withdrawal:u64,
    pub src_public_cancellation : u64
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub deployed_at: u64,
    pub rescue_delay: u64,
}

pub const STATE: Item<State> = Item::new("state");
pub const IMMUTABLES: Item<Immutables> =  Item::new("Immutables");
