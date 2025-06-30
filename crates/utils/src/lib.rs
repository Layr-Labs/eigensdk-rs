//! This module exports generated bindings.
pub mod common;
pub mod rewardsv2;
pub mod slashing;

use crate::slashing::core::allocation_manager::AllocationManager::OperatorSet;
use crate::slashing::core::i_rewards_coordinator::IRewardsCoordinator::OperatorSet as RewardsOperatorSet;
use crate::slashing::middleware::registry_coordinator::IStakeRegistryTypes::StrategyParams as RegistryCoordiinatorStrategyParams;
use crate::slashing::middleware::stake_registry::IStakeRegistryTypes::StrategyParams;
/// Converts [`OperatorSet`] to [`RewardsOperatorSet`]
pub fn convert_allocation_operator_set_to_rewards_operator_set(
    operator_set: OperatorSet,
) -> RewardsOperatorSet {
    RewardsOperatorSet {
        avs: operator_set.avs,
        id: operator_set.id,
    }
}

pub fn convert_stake_registry_strategy_params_to_registry_coordinator_strategy_params(
    params: StrategyParams,
) -> RegistryCoordiinatorStrategyParams {
    RegistryCoordiinatorStrategyParams {
        strategy: params.strategy,
        multiplier: params.multiplier,
    }
}
