use crate::encode_helpers::encode_bytes_message;
use crate::error::ContractError;
use crate::msg::{
    create_stargate_msg, ExecuteMsg, GetOrderDetailsResponse, InstantiateMsg, PullFundsMsg,
    QueryMsg,
};
use crate::state::{Immutables, State, IMMUTABLES, STATE};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult, SubMsg,
};
use cw2::set_contract_version;
use injective_std::{
    shim::Any,
    types::{
        cosmos::authz::v1beta1::MsgExec, cosmos::bank::v1beta1::MsgSend,
        cosmos::base::v1beta1::Coin as ProtoCoin,
    },
};
#[cfg(not(feature = "library"))]
use prost::Message;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:escrow_src";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MSG_EXEC: &str = "/cosmos.authz.v1beta1.MsgExec";
pub const MSG_BANK_SEND: &str = "/cosmos.bank.v1beta1.MsgSend";
pub const REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        deployed_at: _env.block.time.seconds(),
        rescue_delay: msg.rescue_delay,
    };

    let hashlock = hex::decode(&msg.hashlock)
        .map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;

    let order_hash = hex::decode(&msg.order_hash)
        .map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;

    let immutables = Immutables {
        hashlock,
        order_hash,
        maker: msg.maker,
        taker: msg.taker,
        timelocks: msg.timelocks,
        token: msg.token,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    IMMUTABLES.save(deps.storage, &immutables)?;
        /*
        *
        *Note: Can not do pulling of funds here since 
        * address of escrow_src is not known at order, 
        * instantiate2 (create2) requires relayer address to calculate the address of escrow_src
        *
        */

    Ok(Response::new())

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PullFunds(msg) => execute::pull_funds(deps, _env, info, msg),
        ExecuteMsg::Withdraw(msg) => execute::withdraw(deps, _env, info, msg),
    
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, BankMsg, Coin};

    use crate::{helpers::{only_after, only_before, only_valid_secret}, msg::WithdrawMsg};

    use super::*;

    pub fn pull_funds(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: PullFundsMsg,
    ) -> Result<Response, ContractError> {
        let mut state = STATE.load(deps.storage)?;
        state.deployed_at = _env.block.time.seconds();
       

        let giver = msg.from;
        let amount = ProtoCoin {
            denom: msg.amount.denom,
            amount: msg.amount.amount.to_string(),
        };

        let bank_send_msg = MsgSend {
            amount: vec![amount],
            from_address: giver.to_string(),
            to_address: _env.contract.address.to_string(),
        };

        // let exec_msg
        let order_bytes = encode_bytes_message(&bank_send_msg).unwrap();
        let msg_exec = MsgExec {
            grantee: _env.contract.address.to_string(),
            msgs: vec![Any {
                type_url: MSG_BANK_SEND.to_string(), //"/cosmos.bank.v1beta1.MsgSend" ,
                value: order_bytes,
            }],
        };

        let submessage = SubMsg::reply_on_success(
            create_stargate_msg(MSG_EXEC, msg_exec.encode_to_vec()).unwrap(),
            REPLY_ID,
        );

        STATE.save(deps.storage, &state)?;

        Ok(Response::new().add_submessage(submessage))
    }

    pub fn withdraw( deps: DepsMut, env: Env,  info: MessageInfo, msg: WithdrawMsg) -> Result<Response, ContractError> {
        let immutables = IMMUTABLES.load(deps.storage)?;
        let current_time_in_secs = env.block.time.seconds();

        if immutables.taker != info.sender {
            return Err(ContractError::OnlyTaker);
        }

        if only_after(current_time_in_secs, immutables.timelocks.src_withdrawal) {
            return Err(ContractError::SrcWithrawTimeLimit);
        }

        if only_before(current_time_in_secs, immutables.timelocks.src_cancellation) {
            return Err(ContractError::SrcCancelTimeLimit);
        }

        only_valid_secret( msg.secret, immutables.hashlock )? ;

       let sub_msg = _withdraw_to( immutables.taker, immutables.token );
        
        Ok(Response::new().add_submessage(sub_msg))
    }

    



    fn _withdraw_to(target: Addr, amount: Coin ) -> SubMsg {
        let msg = BankMsg::Send {
            to_address: target.into(),
            amount: vec![amount],
        };
        SubMsg::reply_never(msg)
    }


}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::OrderDetails {} => to_json_binary(&query::get_order_details(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_order_details(deps: Deps) -> StdResult<GetOrderDetailsResponse> {
        let state = STATE.load(deps.storage)?;
        let immutables = IMMUTABLES.load(deps.storage)?;
        Ok(GetOrderDetailsResponse {
            deployed_at: state.deployed_at,
            hashlock: hex::encode(immutables.hashlock),
            maker: immutables.maker,
            order_hash: hex::encode(immutables.order_hash),
            rescue_delay: state.rescue_delay,
            taker: immutables.taker,
            timelocks: immutables.timelocks,
            token: immutables.token,
        })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID => Ok(Response::new()),
        _ => Ok(Response::new()),
    }
}