use alloy::network::Network;
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use std::future::Future;

use crate::error::TaskGeneratorError;

/// Task manager contract trait
pub trait TaskManagerContract<Input, T, P, N: Network> {
    /// Create a new task
    ///
    /// # Arguments
    ///
    /// * `input` - Generic input of the task
    /// * `quorum_threshold` - The quorum threshold for the task
    /// * `quorums` - The quorums for the task
    ///
    /// # Returns
    ///
    /// * `Result<N::ReceiptResponse, TaskGeneratorError>` - The result of the task
    fn create_new_task(
        &self,
        input: Input,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> impl Future<Output = Result<N::ReceiptResponse, TaskGeneratorError>> + Send;
}
