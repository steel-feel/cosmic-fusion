use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    #[error("Unable to create escrow contract")]
    EscrowContractError,

    #[error("Order already processed")]
    OrderAlreadyProcessed,

    #[error("Denom/Amount does not match")]
    UnmatchedDenomOrAmount,
}
