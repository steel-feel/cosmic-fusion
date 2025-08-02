use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_json_binary, Addr, AnyMsg, CosmosMsg, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;
use prost::Message;

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
}

pub(crate) fn encode_bytes_message<T: Message>(msg: &T) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buffer = Vec::new();
    msg.encode(&mut buffer)?; // Encode the message using prost
    Ok(buffer)
}

pub fn create_stargate_msg(type_url: &str, value: Vec<u8>) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Any(AnyMsg {
        type_url: type_url.to_string(),
        value: value.into(),
    }))
}
