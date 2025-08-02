#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, ReplyOn, SubMsg, WasmMsg,
    Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply, Response, StdResult,
};
use cw_utils::parse_instantiate_response_data;

use crate::error::ContractError;
use crate::msg::{EscrowInstantiateMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, COMPLETED_ORDERS, STATE};

pub const ESCROW_DEPLOY_REPLY: u64 = 1;
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:escrow-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        escrow_code_id: msg.escrow_code_id,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DeployEscrow(msg) => execute::deploy_dest_escrow(deps, env, info, msg),
    }
}

pub mod execute {
    use super::*;

    pub fn deploy_dest_escrow(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: EscrowInstantiateMsg,
    ) -> Result<Response, ContractError> {
        //check if order already proccessed
        let state = STATE.load(deps.storage)?;
        let is_order_processed = COMPLETED_ORDERS
            .may_load(deps.storage, msg.order_hash.clone())
            .unwrap()
            .unwrap();
        if is_order_processed == true {
            return Err(ContractError::OrderAlreadyProcessed);
        }

        //check if send funds match the order details
        let mut ok = false;
        for asset in info.funds {
            if asset.denom == msg.token.denom && asset.amount == msg.token.amount {
                ok = true;
            }
        }
        if !ok {
            return Err(ContractError::UnmatchedDenomOrAmount);
        }

        //deploy the contract with funds
        let escrow_init_playload_msg = EscrowInstantiateMsg {
            hashlock: msg.hashlock,
            maker: msg.maker.clone(),
            taker: msg.taker,
            order_hash: msg.order_hash.clone(),
            rescue_delay: msg.rescue_delay,
            timelocks: msg.timelocks,
            token: msg.token,
        };

        let instantiate_child_msg = WasmMsg::Instantiate {
            admin: None,
            code_id: state.escrow_code_id,
            msg: to_json_binary(&escrow_init_playload_msg)?,
            funds: vec![escrow_init_playload_msg.token],
            label: format!(
                "Escrow Contract for {}",
                escrow_init_playload_msg.order_hash.as_str()
            ),
        };
        let submsg = SubMsg {
            payload: Binary::new(vec![]),
            msg: instantiate_child_msg.into(),
            gas_limit: None,
            id: ESCROW_DEPLOY_REPLY, // assign an ID to catch the reply
            reply_on: ReplyOn::Always,
        };

        COMPLETED_ORDERS.save(deps.storage, msg.order_hash.clone(), &true)?;

        Ok(Response::new().add_submessage(submsg))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        ESCROW_DEPLOY_REPLY => {
            if msg.result.is_err() {
                return Err(ContractError::EscrowContractError);
            }

            let init_res = parse_instantiate_response_data(msg.payload.as_slice()).unwrap();

            let event = Event::new("escrow_contract")
                .add_attribute("contract_address", init_res.contract_address);
            Ok(Response::new().add_event(event))
        }

        _ => Ok(Response::new()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
