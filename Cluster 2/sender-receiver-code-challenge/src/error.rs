use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Incorrect amount of tokens sent")]
    IncorrectAmount {},

    #[error("Incorrect denomination of tokens sent")]
    IncorrectDenomination {
        denom: String
    },

    #[error("Only 1 type of coin being sent is allowed")]
    OnlyOneCoinAllowed {},
}

/*

- Validate that the amount of tokens being sent in the transaction match the ones the execute message intents to deposit (given that you are passing an amount in the execute message)
- Validate that the denom of the funds sent is uluna, considering we are gonna deploy this on terra.
- Validate only 1 type of coin is being sent in the tx.

*/
