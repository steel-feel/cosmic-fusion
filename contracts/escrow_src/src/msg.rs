use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, AnyMsg, Coin, CosmosMsg, StdResult};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::state::Timelocks;

#[cw_serde]
pub struct InstantiateMsg {
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
    PullFunds(PullFundsMsg),
    Withdraw(WithdrawMsg),
    WithdrawTo(WithdrawToMsg),
    PublicWithdraw(WithdrawMsg),
    Cancel(),
    PublicCancel(),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct WithdrawMsg {
    pub secret: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct WithdrawToMsg {
    pub secret: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetOrderDetailsResponse)]
    OrderDetails {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetOrderDetailsResponse {
    pub deployed_at: u64 ,
    pub rescue_delay: u64,
    pub order_hash: String,
    pub hashlock: String,
    pub maker: Addr,
    pub taker: Addr,
    pub token: Coin,
    pub timelocks: Timelocks,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct PullFundsMsg {
    pub from: Addr,
    /// the tokens to be sell
    pub amount: Coin,
   
}

pub fn create_stargate_msg(type_url: &str, value: Vec<u8>) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Any(AnyMsg {
        type_url: type_url.to_string(),
        value: value.into(),
    }))
}
