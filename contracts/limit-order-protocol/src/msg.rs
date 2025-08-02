use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {
    pub escrow_code_id : u64
}

#[cw_serde]
pub enum ExecuteMsg {
    FillOrder(FillOrderMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct FillOrderMsg {
    pub making_amount: Coin,
    pub taking_amount:Coin,
    pub auction_params: AuctionParameters,
    pub taker_traits: TakerTraits,
    pub immutables: Immutables,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct AuctionParameters {
    pub duration: u64, // in seconds
    pub start_time: u64, // unix timestamp (in sec)
    pub initial_rate_bump: u32, // as a ratio, e.g., 50_000 means 0.5%
    pub points: Vec<PricePoint>,
    pub gas_cost: GasCost,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct PricePoint {
    pub delay: u64, // relative to previous point (or auction start)
    pub coefficient: u32, // as a ratio, e.g., 40_000 means 0.4%
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct GasCost {
    pub gas_bump_estimate: u128, // as a ratio, e.g., 10_000 means 0.1%
    pub gas_price_estimate: u128, // e.g., 1000 means 1 Gwei
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct TakerTraits {
    pub threshold_taking_price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct Immutables {
    pub rescue_delay: u64,
    pub order_hash: String,
    pub hashlock: String,
    pub maker: Addr,
    pub taker: Addr,
    pub timelocks: Timelocks,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
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
pub struct EscrowInstantiateMsg {
    pub rescue_delay: u64,
    pub order_hash: String,
    pub hashlock: String,
    pub maker: Addr,
    pub taker: Addr,
    pub token: Coin,
    pub timelocks: Timelocks,
}
