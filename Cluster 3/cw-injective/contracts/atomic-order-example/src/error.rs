use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}

/*
Only 3 errors defined by the contract: standard error, unauthorized, and a custom error which takes a val argument of type string, which will be defined later when the developer
chooses to implement the custom error response type.
 */
