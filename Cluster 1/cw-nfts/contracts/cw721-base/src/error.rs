use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError), // std error

    #[error("Unauthorized")]
    Unauthorized {}, // unauthorized

    #[error("token_id already claimed")]
    Claimed {}, // token_id already claimed

    #[error("Cannot set approval that is already expired")]
    Expired {}, // approval expired

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String }, // approval not found
}
