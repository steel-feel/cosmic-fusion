use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Only Taker can call")]
    OnlyTaker,

    #[error("Invalid Secret")]
    InvalidSecret,

    #[error("Source withdraw time has not passed")]
    SrcWithrawTimeLimit,

    #[error("Source cancellation time has passed")]
    SrcCancelTimeLimit,

    #[error("Rescue time has passed")]
    RescueTimeLimit
}
