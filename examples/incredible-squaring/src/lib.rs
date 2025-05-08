//! Example AVS which squares a number

use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
    primitives::{B256, U256},
    sol_types::SolEvent,
};
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::TaskResponded;
use eigen_task_processor::{
    default_contract_impl,
    task::Task,
    task_manager::{box_error, TaskManagerContract, TaskManagerError},
    task_response::TaskResponse,
    task_response_metadata_sol::TaskResponseMetadataSol,
};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;

// Implement the TaskManagerContract trait for the task manager contract.
// You need to specify the input type of the task. In this case, U256.
// You also need to specify the call type of the task manager contract. `createNewTask` uses `createNewTaskCall`.
// You also need to specify the provider and network types.
//
// NOTE: When you are implementing this, you will have an exteranl trait `TaskManagerContract` and and external struct
// `CONTRACT_NAME_INSTANCE`, so it will throw an error. You can wrap the external struct in a newtype to avoid this.
// Example:
// struct TaskManagerWrapper<T, P, N>(IncredibleSquaringTaskManagerInstance<T, P, N>);
//
// impl<T, P, N> TaskManagerContract<U256, T, P, N> for TaskManagerWrapper<T, P, N> { ... }
impl<T, P, N> TaskManagerContract for IncredibleSquaringTaskManagerInstance<T, P, N>
where
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    type Input = U256;
    type Output = U256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;

    default_contract_impl! {}
}
