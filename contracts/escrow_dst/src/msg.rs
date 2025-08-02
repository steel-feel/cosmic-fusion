use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::Timelocks;
#[cw_serde]
pub struct InstantiateMsgData {
    pub rescue_delay: u64,
    pub order_hash: String,
    pub hashlock: String,
    pub maker: Addr,
    pub taker: Addr,
    pub token: Coin,
    pub timelocks: Timelocks,
}
#[cw_serde]
pub enum ExecuteMsg {
    Withdraw(WithdrawMsg),
    PublicWithdraw(WithdrawMsg),
    Cancel,
    RescueFunds
}

#[cw_serde]
pub struct WithdrawMsg {
    pub secret: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

