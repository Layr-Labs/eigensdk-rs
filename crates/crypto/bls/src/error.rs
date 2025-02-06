use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlsError {
    /// Invalid Bls Public Key
    #[error("Invalid bls public key")]
    InvalidPublicKey,

    /// Invalid Bls Private Key
    #[error("Invalid bls private key   ")]
    InvalidBlsPrivateKey,

    /// Invalid G1Affine
    #[error("Points missing in G1Affine")]
    InvalidG1Affine,

    /// Invalid G2Affine
    #[error("Points missing in G2Affine")]
    InvalidG2Affine,
}
