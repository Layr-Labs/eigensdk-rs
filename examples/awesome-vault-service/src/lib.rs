//! AVS Example - Awesome Vault Service
//!
//! This example shows how to create a more complex AVS based on the SDK structure.
//! The AVS shown here is a local Redis version that allows storing key-value pairs
//! in the Task Manager. In this sense, the task for the operators to complete is
//! storing the pair into a local Merkle tree of pairs and recomputing the root hash
//! of the tree, submitting it as the response value to the Task Manager on-chain contract.
//!
//! To learn how to implement each module, refer to the corresponding binary files.
//! Each file contains step-by-step instructions for setting up the module.
//!
//! For more details about the example logic and how to run it, see the
//! [README](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/examples/awesome-vault-service/README.md).

pub mod bindings;
pub mod response_calculator;
pub mod task_manager;
pub mod utils;
