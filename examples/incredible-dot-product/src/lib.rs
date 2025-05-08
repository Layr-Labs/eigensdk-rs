//! Incredible Dot Product Example
//!
//! This example demonstrates how to use the SDK to build an AVS.

/// Task manager for the Incredible Dot Product Operator
pub mod task_manager;

/// Utils for operator registry
pub mod utils;

/// Re-export the task manager and utils
pub use task_manager::*;
pub use utils::*;
