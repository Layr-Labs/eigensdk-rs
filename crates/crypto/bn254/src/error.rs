use thiserror::Error;

#[derive(Debug, Error)]
pub enum Bn254Err {
    /// Build Fq instance from string
    #[error("failed to build a Fq instance from string ")]
    Fq,
}
