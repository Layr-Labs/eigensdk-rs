use alloy::primitives::B256;
use eigen_types::avs::TaskIndex;
use serde::{Deserialize, Serialize};

/// Task response trait
pub trait TaskResponse: for<'de> Deserialize<'de> + Serialize {
    /// Returns the index of the task
    fn task_index(&self) -> TaskIndex;

    /// Returns a 32-byte digest of the response
    fn digest(&self) -> B256;
}
