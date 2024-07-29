use alloy_contract::Error as AlloyError;
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
