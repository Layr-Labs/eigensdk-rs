//! This module exports generated bindings.
pub mod rewardsv2;
pub mod slashing;

use crate::slashing::core::allocationmanager::AllocationManager::OperatorSet;
use crate::slashing::core::irewardscoordinator::IRewardsCoordinator::OperatorSet as RewardsOperatorSet;

/// Converts [`OperatorSet`] to [`RewardsOperatorSet`]
pub fn convert_allocation_operator_set_to_rewards_operator_set(
    operator_set: OperatorSet,
) -> RewardsOperatorSet {
    RewardsOperatorSet {
        avs: operator_set.avs,
        id: operator_set.id,
    }
}
