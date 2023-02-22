use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, to_binary,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:injective:dummy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const COUNTER: Item<u32> = Item::new("counter");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    COUNTER.save(deps.storage, &0u32)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _: DepsMut,
    _: Env,
    _: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Ping { .. } => {
            let mut response = Response::new();
            response.data = Some(to_binary("pong")?);
            Ok(response)
        }
        ExecuteMsg::Error { .. } => Err(ContractError::Std(StdError::generic_err("oh no!"))),
    }
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BeginBlocker {} => {
            for i in 1..10000 {
                let _r = i * 2 / i * i / 3 * 2 * 7 / 7;
            }
            let runs = COUNTER.load(deps.storage)? + 1;
            COUNTER.save(deps.storage, &runs)?;
            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ping { .. } => to_binary("pong"),
        QueryMsg::Error { .. } => Err(StdError::generic_err("oh no!")),
        QueryMsg::Runs {} => {
            let runs_count = COUNTER.load(deps.storage)?;
            to_binary(&format!("{runs_count}"))
        }
    }
}
