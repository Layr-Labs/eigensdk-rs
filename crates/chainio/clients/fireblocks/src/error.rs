use jsonwebtoken::errors::Error;
use thiserror::Error;

/// Error returned by AvsRegistry
#[derive(Debug, Error)]
pub enum FireBlockError {
    /// Could not sign the jwt payload
    #[error("Failed to sign the jwt payload")]
    JsonWebTokenError(Error),
}
