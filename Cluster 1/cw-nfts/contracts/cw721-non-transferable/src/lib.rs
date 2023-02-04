use cosmwasm_std::Empty;
pub use cw721_base::{
    ContractError,
    Cw721Contract, entry::{execute as _execute, query as _query}, ExecuteMsg, Extension, InstantiateMsg as Cw721BaseInstantiateMsg,
    MinterResponse, MintMsg,
};

pub use crate::msg::{InstantiateMsg, QueryMsg};

pub mod msg;
pub mod query;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-non-transferable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw721NonTransferableContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::{
        Addr, Binary, Deps, DepsMut, entry_point, Env, MessageInfo, Response, StdResult, to_binary,
    };

    use crate::query::admin;
    use crate::state::{Config, CONFIG};

    use super::*;

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let admin_addr: Option<Addr> = msg
            .admin
            .as_deref()
            .map(|s| deps.api.addr_validate(s))
            .transpose()?;

        let config = Config { admin: admin_addr };

        CONFIG.save(deps.storage, &config)?;

        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        };

        Cw721NonTransferableContract::default().instantiate(
            deps.branch(),
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, cw721_base::ContractError> {
        let config = CONFIG.load(deps.storage)?;
        match config.admin {
            Some(admin) => {
                if admin == info.sender {
                    _execute(deps, env, info, msg)
                } else {
                    Err(ContractError::Unauthorized {})
                }
            }
            None => match msg {
                ExecuteMsg::Mint(msg) => {
                    Cw721NonTransferableContract::default().mint(deps, env, info, msg)
                }
                _ => Err(ContractError::Unauthorized {}),
            },
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Admin {} => to_binary(&admin(deps)?),
            _ => _query(deps, env, msg.into()),
        }
    }
}
