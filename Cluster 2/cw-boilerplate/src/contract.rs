// imports data structures and macros from cosmwasm_std library
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Responde, StdResult, to_binary};
// imports entry_point macro from cosmwasm_std library
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
// imports set_contract_version macro from cw_2 library
use cw_2::set_contract_version;

// imports data structures from our other contract files
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:template";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


// module for contract execution
pub mod execute {
    use super::*;

// unimplemented execute functions
}