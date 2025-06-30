//! AVS Example - Incredible Dot Product
//!
//! This example proposes a more complex AVS than the incredible squaring.
//! The input type handled is a struct that contains two vectors, representing two points,
//! that should be processed making the dot product, being the output type a U256,
//! representing the result of the product. In this sense, the task for the operators
//! to complete is executing the dot product between the two points, and returning the
//! result as the response value submitted to the Task Manager on-chain contract.
//!
//! To learn how to implement each module, refer to the corresponding binary files.
//! Each file contains step-by-step instructions for setting up the module.
//!
//! For more details about the example logic and how to run it, see the
//! [README](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/examples/incredible-dot-product/README.md).

/// Bindings for the Incredible Dot Product Operator
pub mod bindings;
/// Task manager for the Incredible Dot Product Operator
pub mod task_manager;
/// Utils for the Incredible Dot Product Operator
pub mod utils;

pub use bindings::incredible_dot_product_task_manager::*;
