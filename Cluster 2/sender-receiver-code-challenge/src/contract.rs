use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive { sender, amount } => receive(deps, env, info, sender, amount),
    }
    Ok(Response::new().add_attribute("action", "execute"))
}

// write a function to forward the tokens from the sender to the receiver
fn forward_tokens(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    sender: String,
    amount: Uint128,
    forward_to_address: String,
) -> Result<Response, ContractError> {
    let forward_to_address = deps.api.addr_validate(&forward_to_address)?;
    let res = Response::new()
        .add_attribute("action", "forward_tokens")
        .add_message(SubMsg::new(CosmosMsg::Bank(BankMsg::Send {
            to_address: forward_to_address.to_string(),
            amount: info.funds,
        }],
})));
Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
