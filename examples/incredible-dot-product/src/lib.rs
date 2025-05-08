//! Incredible Dot Product Example
//!
//! This example demonstrates how to use the SDK to build an AVS.

/// Task manager for the Incredible Dot Product Operator
pub mod task_manager;

/// Bindings for the Incredible Dot Product Operator
pub mod bindings;

/// Utils for operator registry
pub mod utils;

pub use bindings::incredibledotproducttaskmanager::*;
/// Re-export the task manager and utils
pub use utils::*;
