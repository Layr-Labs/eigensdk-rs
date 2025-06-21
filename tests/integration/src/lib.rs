//! Integration tests for the Eigensdk
//!
//! This crate contains the necessary code to run an AVS integration test.
//!
//! We are testing our AVS examples
//! * [Incredible Squaring](https://github.com/Layr-Labs/eigensdk-rs/tree/v2-dev-2/examples/incredible-squaring)
//! * [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/tree/v2-dev-2/examples/incredible-dot-product)
//! * [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/tree/v2-dev-2/examples/awesome-vault-service)

/// AVS bindings
#[allow(warnings)]
pub mod bindings;

/// Generic AVS code
#[cfg(test)]
pub mod generic_avs;
/// Incredible Squaring integration test
#[cfg(test)]
pub mod incredible_squaring;
