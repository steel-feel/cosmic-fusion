#[cfg(not(feature = "library"))]
use prost::Message;
use injective_std:: { shim::Any, types::{  cosmos::authz::v1beta1::MsgExec,  cosmos::bank::v1beta1::MsgSend, cosmos::base::v1beta1::Coin as ProtoCoin },};       
use cosmwasm_std::{entry_point,SubMsg,to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;
use crate::{encode_helpers::encode_bytes_message};
use crate::error::ContractError;
use crate::msg::{create_stargate_msg, ExecuteMsg, GetCountResponse, InstantiateMsg, PullFundsMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:escrow_src";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MSG_EXEC: &str = "/cosmos.authz.v1beta1.MsgExec";
pub const MSG_BANK_SEND: &str = "/cosmos.bank.v1beta1.MsgSend";
pub const REPLY_ID:u64 = 1; 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::PullFunds(msg) => execute::pull_funds(deps, _env,info, msg),
    }
}



pub mod execute {
    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }

    pub fn pull_funds(_deps: DepsMut,_env: Env,  _info: MessageInfo, msg: PullFundsMsg ) -> Result<Response, ContractError>  {
        let giver = msg.from;
        let amount =    ProtoCoin  { denom: msg.amount.denom, amount: msg.amount.amount.to_string() };


        let bank_send_msg = MsgSend {
            amount : vec![amount],
            from_address :giver.to_string(),
            to_address : _env.contract.address.to_string(),
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
    
        Ok(Response::new().add_submessage(submessage))  
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        REPLY_ID =>  {
            Ok(Response::new().add_attribute("jeete", "raaho!"))  
        },
        _ => Ok(Response::new())  
        
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_json(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
