use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError), // standard error response

    #[error("Unauthorized")]
    Unauthorized {}, // unauthorized error response if the user is not the owner of the contract and attempts to reset the count
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
