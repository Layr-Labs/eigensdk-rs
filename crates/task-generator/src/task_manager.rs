use alloy::{contract::SolCallBuilder, sol_types::SolCall};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};

/// Task manager contract trait
pub trait TaskManagerContract<Input, T, P, N: alloy::network::Network> {
    /// Input type of the task

    /// Call type of the task manager contract
    type Call: SolCall;

    /// Create a new task
    fn create_new_task(
        &self,
        input: Input,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> SolCallBuilder<T, &P, Self::Call, N>;
}
