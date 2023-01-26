// import dependencies
use cosmwasm_std::StdError;
use thiserror::Error;

// derive traits
#[derive(Error, Debug)]

// define ContractError enumerator
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    // derive unauthorized error
    #[error("Unauthorized")]
    Unauthorized {},

    // derive too many options error for a poll with too many options
    #[error("Too many poll options")]
    TooManyOptions {},
}
