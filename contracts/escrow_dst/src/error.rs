use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Denom/Amount does not match")]
    UnmatchedDenomOrAmount,

    #[error("Only Taker can call")]
    OnlyTaker,

    #[error("Invalid Secret")]
    InvalidSecret,

    #[error("Destinational withdraw time has not passed")]
    DestWithrawTimeLimit,

    #[error("Destinational cancellation time has passed")]
    DestCancelTimeLimit,

    #[error("Rescue time has passed")]
    RescueTimeLimit
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
