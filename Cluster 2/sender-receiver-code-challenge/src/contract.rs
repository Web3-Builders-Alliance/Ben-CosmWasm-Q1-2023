use cosmwasm_std::{BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// use cw2::set_contract_version;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sender-receiver-code-challenge";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ForwardTokens { forward_to_addr } => {
            forward_tokens(deps, env, info, forward_to_addr)
        }
    }
}

fn forward_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    forward_to_addr: String,
) -> Result<Response, ContractError> {
    let validated_addr = deps.api.addr_validate(&forward_to_addr)?.to_string();

    let msg = BankMsg::Send {
        to_address: validated_addr,
        amount: info.funds,
    };

    Ok(Response::new()
        .add_attribute("action", "forward_tokens")
        .add_message(CosmosMsg::Bank(msg)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
