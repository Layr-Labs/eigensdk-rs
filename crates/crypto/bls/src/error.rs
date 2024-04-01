use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlsError {
    /// Multiply private key to g2 projective
    #[error("Failed to multiply by G2 Projective")]
    MulByG2Projective,

    /// Multiply private key to g1 projective
    #[error("Failed to multiply by G1 Projective")]
    MulByG1Projective,
}
