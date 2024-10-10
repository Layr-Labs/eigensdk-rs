use alloy::contract::Error as AlloyError;
use jsonwebtoken::errors::Error;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeErr;
use std::fmt;
use thiserror::Error;

/// Error returned by AvsRegistry
#[derive(Debug, Error)]
pub enum FireBlockError {
    /// Could not sign the jwt payload
    #[error("Failed to sign the jwt payload")]
    JsonWebTokenError(Error),

    /// Alloy Contract error
    #[error("Alloy error{0}")]
    AlloyContractError(#[from] AlloyError),

    #[error("Other Error {0}")]
    OtherError(String),

    /// Reqwest error
    #[error("Reqwest error {0}")]
    ReqwestError(#[from] ReqwestError),

    /// Invalid header value error
    #[error("Invalid header value error {0}")]
    InvalidHeaderValueError(#[from] InvalidHeaderValue),

    /// Serde error
    #[error("Serde json error {0}")]
    SerdeError(#[from] SerdeErr),

    /// Serde error
    #[error("Chain Id not supported{0} . Create an issue at  https://github.com/Layr-Labs/eigensdk-rs/issues/new")]
    AssetIDError(String),

    /// Account not found in whitelisted accounts
    #[error("Account not found in whitelisted accounts {0}")]
    AccountNotFoundError(String),

    /// Contract not found in whitelisted contract
    #[error("Contract {0} not found in whitelisted contract")]
    ContractNotFound(String),

    /// Transaction failed or rejected or Cancelled or Blocked
    #[error("Transaction Failed with  Status {0} , tx_id {1}")]
    TransactionFailed(String, String),

    /// To be broadcasted transactions
    #[error("Transaction to be broadcasted with Status {0} , tx_id {1}")]
    NotBroadcasted(String, String),

    /// Receipt not available yet
    #[error("Transaction not available yet with Status {0} , tx_id {1} ")]
    ReceiptNotYetAvailable(String, String),

    /// Transaction receipt not found
    #[error("Transactin receipt not found with tx_id {0}")]
    TransactionReceiptNotFound(String),
}

impl FireBlockError {
    pub fn from<E: fmt::Display>(error: E) -> FireBlockError {
        FireBlockError::OtherError(error.to_string())
    }
}

impl From<&str> for FireBlockError {
    fn from(err: &str) -> FireBlockError {
        FireBlockError::OtherError(err.to_string())
    }
}
