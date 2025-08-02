use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Auction has not started yet")]
    AuctionNotStarted,

    #[error("Auction has already ended")]
    AuctionEndedAlready,

    #[error("Price is above takers threshold")]
    PriceIsAboveThreshold,

    #[error("Unable to pull funds from maker")]
    PullFundsError,

    #[error("Unable to create escrow contract")]
    EscrowContractError,

    #[error("Order already processed")]
    OrderAlreadyProcessed



    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
