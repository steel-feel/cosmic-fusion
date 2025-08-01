use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};
use sha3::{ Digest,Keccak256};

use crate::{msg::{ExecuteMsg, GetOrderDetailsResponse, QueryMsg}, ContractError};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    /// Get Count
    pub fn get_order_details<Q, T, CQ>(&self, querier: &Q) -> StdResult<GetOrderDetailsResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::OrderDetails {  };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&msg)?,
        }
        .into();
        let res: GetOrderDetailsResponse= QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}

pub fn only_valid_secret(
    secret: String,
    hashlock: Vec<u8>
) -> Result<(), ContractError> {

    let mut hasher = Keccak256::new();
    hasher.update(secret.as_bytes());
    let computed_hash = hasher.finalize();

    if computed_hash.to_vec() != hashlock {
        return Err(ContractError::InvalidSecret);
    }

    Ok(())
}

pub fn only_after(  current_time: u64, value: u64) -> bool  {
    value > current_time      
}

pub fn only_before(current_time: u64, value: u64) -> bool {
  value < current_time 
}


