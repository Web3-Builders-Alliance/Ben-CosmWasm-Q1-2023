/*
The error.rs file contains all of the errors possible in the contract.
 */

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError), // standard error response

    #[error("Unauthorized")]
    Unauthorized {}, // unauthorized error response

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {}, // cannot set to own account error response used for limits and allowances

    #[error("Invalid zero amount")]
    InvalidZeroAmount {}, // invalid zero amount as input error response

    #[error("Allowance is expired")]
    Expired {}, // allowance is expired error response

    #[error("No allowance for this account")]
    NoAllowance {}, // no allowance for this account error response

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {}, // minting cannot exceed the supply cap error response

    #[error("Logo binary data exceeds 5KB limit")]
    LogoTooBig {},  // logo binary data exceeds 5KB limit error response

    #[error("Invalid xml preamble for SVG")]
    InvalidXmlPreamble {}, // invalid xml preamble for SVG error response

    #[error("Invalid png header")]
    InvalidPngHeader {}, // invalid png header error response

    #[error("Invalid expiration value")]
    InvalidExpiration {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},
}
