#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::helpers::{only_after, only_before, only_valid_secret};
use crate::msg::{ExecuteMsg, InstantiateMsgData, QueryMsg, WithdrawMsg};
use crate::state::{Immutables, State, IMMUTABLES, STATE};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:escrow_dst";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsgData,
) -> Result<Response, ContractError> {
    let mut ok = false;
    for asset in info.funds {
        if asset.denom == msg.token.denom && asset.amount == msg.token.amount {
            ok = true;
        }
    }
    if !ok {
        return Err(ContractError::UnmatchedDenomOrAmount);
    }

    let hashlock = hex::decode(&msg.hashlock).map_err(|e| StdError::generic_err(e.to_string()))?;

    let order_hash =
        hex::decode(&msg.order_hash).map_err(|e| StdError::generic_err(e.to_string()))?;

    STATE.save(
        deps.storage,
        &State {
            deployed_at: env.block.time.seconds(),
            rescue_delay: msg.rescue_delay,
        },
    )?;
    IMMUTABLES.save(
        deps.storage,
        &Immutables {
            hashlock,
            order_hash,
            maker: msg.maker,
            taker: msg.taker,
            timelocks: msg.timelocks,
            token: msg.token,
        },
    )?;

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
        ExecuteMsg::Withdraw(msg) => execute::withdraw(deps, env, info, msg),
        ExecuteMsg::PublicWithdraw(msg) => execute::public_withdraw(deps, env, info, msg),
        ExecuteMsg::Cancel => execute::cancel(deps, env, info),
        ExecuteMsg::RescueFunds => execute::rescue_funds(deps, env, info)
    }
}

pub mod execute {

    use cosmwasm_std::{Addr, BankMsg, Coin, SubMsg};

    use super::*;

    pub fn withdraw(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: WithdrawMsg,
    ) -> Result<Response, ContractError> {
        let state: State = STATE.load(deps.storage)?;
        let immutables: Immutables = IMMUTABLES.load(deps.storage)?;

        if info.sender != immutables.taker {
            return Err(ContractError::OnlyTaker);
        }

        let current_time_in_secs = env.block.time.seconds();
        if only_after(
            current_time_in_secs,
            state.deployed_at + immutables.timelocks.dest_withdrawal,
        ) {
            return Err(ContractError::DestWithrawTimeLimit);
        }

        if only_before(
            current_time_in_secs,
            state.deployed_at + immutables.timelocks.dest_cancellation,
        ) {
            return Err(ContractError::DestCancelTimeLimit);
        }

        only_valid_secret(msg.secret, immutables.hashlock)?;

        Ok(Response::new().add_submessage(send_bank_msg(immutables.maker, immutables.token)))
    }

    pub fn public_withdraw(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: WithdrawMsg,
    ) -> Result<Response, ContractError> {
        let state: State = STATE.load(deps.storage)?;
        let immutables: Immutables = IMMUTABLES.load(deps.storage)?;
        let current_time_in_secs = env.block.time.seconds();

        if only_after(
            current_time_in_secs,
            state.deployed_at + immutables.timelocks.dest_public_withdrawal,
        ) {
            return Err(ContractError::DestWithrawTimeLimit);
        }

        if only_before(
            current_time_in_secs,
            state.deployed_at + immutables.timelocks.dest_cancellation,
        ) {
            return Err(ContractError::DestCancelTimeLimit);
        }

        //Check secret hash
        only_valid_secret(msg.secret, immutables.hashlock)?;

        Ok(Response::new().add_submessage(send_bank_msg(immutables.maker, immutables.token)))
    }
   pub fn cancel(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let state: State = STATE.load(deps.storage)?;
        let immutables: Immutables = IMMUTABLES.load(deps.storage)?;
        let current_time_in_secs = env.block.time.seconds();

        if info.sender != immutables.taker {
            return Err(ContractError::OnlyTaker);
        }

        if only_after(
            current_time_in_secs,
            state.deployed_at + immutables.timelocks.dest_cancellation,
        ) {
            return Err(ContractError::DestCancelTimeLimit);
        }

        Ok(Response::new().add_submessage(send_bank_msg(immutables.taker, immutables.token)))
    }

    pub fn rescue_funds(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> { 
        let state: State = STATE.load(deps.storage)?;
        let immutables: Immutables = IMMUTABLES.load(deps.storage)?;
        let current_time_in_secs = env.block.time.seconds();

        if info.sender != immutables.taker {
            return Err(ContractError::OnlyTaker);
        }

        if only_after(current_time_in_secs, state.deployed_at+  state.rescue_delay) {
            return Err(ContractError::RescueTimeLimit);
        }



        Ok(Response::new().add_submessage(send_bank_msg(immutables.taker, immutables.token)))

    }


    fn send_bank_msg(to: Addr, amount: Coin) -> SubMsg {
        let msg = BankMsg::Send {
            to_address: to.into(),
            amount: vec![amount],
        };

        SubMsg::reply_never(msg)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
