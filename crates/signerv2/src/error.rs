use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignerV2Error {
    /// Build ethhers wallet from private key
    #[error("Failed to build wallet from private key")]
    BuildWallet,

    /// Failed to sign transaction
    #[error("Failed to sign transaction")]
    SignTransaction,

    /// Failed to decrypt key
    #[error("Failed to decrypt key in keystore ")]
    Decryptkey,

    /// Failed to get any config in  signerv2
    #[error("Did not found any signer type in the config")]
    ConfigNotFound,
}
