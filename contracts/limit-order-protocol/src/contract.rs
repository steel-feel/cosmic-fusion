#[cfg(not(feature = "library"))]
use cosmwasm_std::{from_json, to_json_binary, Event, Reply, ReplyOn, SubMsg, WasmMsg,entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response,  StdResult, };
use cw_utils::{parse_instantiate_response_data};

use crate::error::ContractError;
use crate::msg::{EscrowInstantiateMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, COMPLETED_ORDERS, STATE};

pub const PULL_REPLY: u64 = 1;
pub const ESCROW_DEPLOY_REPLY: u64 = 2;
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:limit-order-protocol";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // COMPLETED_ORDERS.save(_deps.storage,  )
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
        ExecuteMsg::FillOrder(msg) => execute::fill_order(deps, env, info, msg),
    }
}

pub mod execute {
    use super::*;
    use crate::msg::{AuctionParameters, EscrowInstantiateMsg, FillOrderMsg};
    use crate::{
        error::ContractError,
        helpers::{create_stargate_msg, encode_bytes_message},
    };
    use cosmwasm_std::{
        to_json_binary, Addr, Coin, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
        Uint128,
    };
    use injective_std::{
        shim::Any,
        types::{
            cosmos::authz::v1beta1::MsgExec, cosmos::bank::v1beta1::MsgSend,
            cosmos::base::v1beta1::Coin as ProtoCoin,
        },
    };
    use prost::Message;

    pub const MSG_EXEC: &str = "/cosmos.authz.v1beta1.MsgExec";
    pub const MSG_BANK_SEND: &str = "/cosmos.bank.v1beta1.MsgSend";

    pub fn fill_order(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        msg: FillOrderMsg,
    ) -> Result<Response, ContractError> {
        let is_order_processed = COMPLETED_ORDERS.may_load(deps.storage, msg.immutables.order_hash.clone()).unwrap().unwrap();
        if is_order_processed == true {
            return Err(ContractError::OrderAlreadyProcessed);
        }
        let block_time = env.block.time.seconds();

        if block_time < msg.auction_params.start_time {
            return Err(ContractError::AuctionNotStarted);
        }

        // Check if the auction has ended.
        if block_time > msg.auction_params.start_time + msg.auction_params.duration {
            return Err(ContractError::AuctionEndedAlready);
        }

        let current_price = calculate_price(
            msg.making_amount.amount,
            msg.taking_amount.amount,
            &msg.auction_params,
            block_time,
        )?;

        // Check if current price agreed by taker
        if current_price > msg.taker_traits.threshold_taking_price {
            return Err(ContractError::PriceIsAboveThreshold);
        }
         //Pull funds from maker to LOP
        let proto_amount = ProtoCoin {
            amount: current_price.to_string(),
            denom: msg.making_amount.denom.clone(),
        };

        let escrow_playload_msg = EscrowInstantiateMsg {
            hashlock: msg.immutables.hashlock,
            maker: msg.immutables.maker.clone(),
            taker: msg.immutables.taker,
            order_hash: msg.immutables.order_hash.clone(),
            rescue_delay: msg.immutables.rescue_delay,
            timelocks: msg.immutables.timelocks,
            token: Coin {
                denom: msg.taking_amount.denom.clone(),
                amount: current_price,
            },
        };

        let pull_sub_msg = pull_funds(proto_amount, msg.immutables.maker, env.contract.address);

        COMPLETED_ORDERS.save(deps.storage, msg.immutables.order_hash.clone(), &true)?;

        Ok(Response::new()
            .add_submessage(pull_sub_msg.with_payload(to_json_binary(&escrow_playload_msg)?)))
    }

    fn pull_funds(token: ProtoCoin, from_address: Addr, to_address: Addr) -> SubMsg {
        let bank_send_msg = MsgSend {
            amount: vec![token],
            from_address: from_address.to_string(),
            to_address: to_address.to_string(),
        };

        // let exec_msg
        let order_bytes = encode_bytes_message(&bank_send_msg).unwrap();
        let msg_exec = MsgExec {
            grantee: to_address.to_string(),
            msgs: vec![Any {
                type_url: MSG_BANK_SEND.to_string(), //"/cosmos.bank.v1beta1.MsgSend" ,
                value: order_bytes,
            }],
        };

        SubMsg::reply_always(
            create_stargate_msg(MSG_EXEC, msg_exec.encode_to_vec()).unwrap(),
            PULL_REPLY,
        )
    }

    fn calculate_price(
        making_amount: Uint128,
        taking_amount: Uint128,
        params: &AuctionParameters,
        current_time: u64,
    ) -> StdResult<Uint128> {
        let elapsed_time = current_time.saturating_sub(params.start_time);
        let mut total_delay = 0;
        let mut current_coefficient = params.initial_rate_bump as u64;

        for point in &params.points {
            let point_time = total_delay + point.delay;
            if elapsed_time >= point_time {
                // We have passed this point, so use its coefficient.
                total_delay = point_time;
                current_coefficient = point.coefficient as u64;
            } else {
                // We are in the current price segment, so interpolate the coefficient.
                let segment_start_time = total_delay;
                let segment_duration = point.delay;
                let time_in_segment = elapsed_time.saturating_sub(segment_start_time);

                let prev_coefficient = current_coefficient;
                let next_coefficient = point.coefficient as u64;

                let coeff_diff = if next_coefficient > prev_coefficient {
                    next_coefficient - prev_coefficient
                } else {
                    prev_coefficient - next_coefficient
                };

                let interpolated_coefficient = if segment_duration > 0 {
                    prev_coefficient
                        .checked_add(
                            coeff_diff
                                .checked_mul(time_in_segment)
                                .unwrap()
                                .checked_div(segment_duration)
                                .unwrap(),
                        )
                        .unwrap()
                } else {
                    prev_coefficient
                };

                current_coefficient = interpolated_coefficient;
                break;
            }
        }

        let price_range = making_amount.checked_sub(taking_amount)?;

        // Calculate the decayed price based on the current coefficient and the total price range.
        let decayed_price = price_range
            .checked_mul(Uint128::from(current_coefficient))?
            .checked_div(Uint128::new(100_000_000))?; // Assuming a 100% base rate

        let price = making_amount.checked_sub(decayed_price)?;

        // // Adjust for gas costs (simplified model)
        // let gas_bump = params.gas_cost.gas_bump_estimate as u128;
        // let price_with_gas = price.checked_add(
        //     price.checked_mul(Uint128::from(gas_bump))?.checked_div(Uint128::new(100_000))?
        // )?;

        Ok(price)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    match msg.id {
        PULL_REPLY => {
            if msg.result.is_err() {
                return Err(ContractError::PullFundsError);
            }

            let escrow_init_msg: EscrowInstantiateMsg = from_json(&msg.payload)?;
            let instantiate_child_msg = WasmMsg::Instantiate {
                admin: None,
                code_id: state.escrow_code_id,
                msg: to_json_binary(&escrow_init_msg)?,
                funds: vec![escrow_init_msg.token],
                label: format!(
                    "Escrow Contract for {}",
                    escrow_init_msg.order_hash.as_str()
                ),
            };
            let submsg = SubMsg {
                payload: Binary::new(vec![]),
                msg: instantiate_child_msg.into(),
                gas_limit: None,
                id: ESCROW_DEPLOY_REPLY, // assign an ID to catch the reply
                reply_on: ReplyOn::Always,
            };

            Ok(Response::new().add_submessage(submsg))
        }

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

#[cfg(test)]
mod tests {}
