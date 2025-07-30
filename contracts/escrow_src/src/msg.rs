use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, AnyMsg, Coin, CosmosMsg, StdResult};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    PullFunds(PullFundsMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
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
