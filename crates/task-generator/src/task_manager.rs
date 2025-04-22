use alloy::{contract::SolCallBuilder, sol_types::SolCall};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};

/// Task manager contract trait
pub trait TaskManagerContract<Input, T, P, N: alloy::network::Network> {
    /// Call type that will be used to create a new task
    type Call: SolCall;

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
    /// * `SolCallBuilder<T, &P, Self::Call, N>` - The call builder for the task
    fn create_new_task(
        &self,
        input: Input,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> SolCallBuilder<T, &P, Self::Call, N>;
}
