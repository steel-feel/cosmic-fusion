use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct InstantiateMsg {
    pub escrow_code_id : u64
}

#[cw_serde]
pub enum ExecuteMsg {
    DeployEscrow(EscrowInstantiateMsg)
}

#[cw_serde]
pub struct EscrowInstantiateMsg {
    pub rescue_delay: u64,
    pub order_hash: String,
    pub hashlock: String,
    pub maker: Addr,
    pub taker: Addr,
    pub token: Coin,
    pub timelocks: Timelocks,
}

#[cw_serde]
pub struct Timelocks {
    pub dest_withdrawal: u64,
    pub dest_public_withdrawal: u64,
    pub dest_cancellation: u64,
    pub src_cancellation: u64,
    pub src_withdrawal:u64,
    pub src_public_withdrawal:u64,
    pub src_public_cancellation : u64
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
