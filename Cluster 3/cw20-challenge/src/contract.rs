use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg, Denom};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-challenge";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // instantiate with the CW20_ADDRESS
    let cw20_address = Cw20Contract {
        address: deps.api.addr_validate(&msg.cw20_address)?,
    };
    Ok(Response::new().add_attribute("action", "instantiate").add_attribute("cw20_address", cw20_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AcceptMint {} => accept_mint(deps, env, info),
    }
}

pub mod execute {
    use super::*;

    pub fn accept_mint(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        // accept_mint will allow the user to transfer uluna coins to the contract address, will validate the coins sent are uluna, and will send a submessage to the cw20 contract address to mint the corresponding amount of cw20 tokens.
        // transfer uluna to the contract address
        let cw20_address = Cw20Contract {
            address: deps.api.addr_validate(&msg.cw20_address)?,
        };
        let uluna = Denom::Native("uluna".to_string());
        let uluna_coin = Cw20Coin {
            denom: uluna,
            amount: msg.amount,
        };
        let transfer_msg = Cw20ExecuteMsg::Transfer {
            recipient: env.contract.address,
            amount: uluna_coin,
        };
        Ok(Response::new()
            .add_message(transfer_msg)
            .add_attribute("action", "accept_mint")
            .add_attribute("cw20_address", cw20_address))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // query the amount of uluna currently held by the contract address
    let uluna = Denom::Native("uluna".to_string());
    let uluna_coin = Cw20Coin {
        denom: uluna,
        amount: msg.amount,
    };
    Ok(to_binary(&uluna_coin)?)
}

#[cfg(test)]
mod tests {}
