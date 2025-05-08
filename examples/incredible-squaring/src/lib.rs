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

    async fn create_new_task(
        &self,
        input: U256,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Result<(), TaskManagerError> {
        self.createNewTask(input, quorum_threshold.into(), quorums.into())
            .send()
            .await
            .map_err(box_error)?
            .get_receipt()
            .await
            .map_err(box_error)
            .map(|_| ())
    }

    async fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        task_response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), TaskManagerError> {
        let contract_task = (
            task.input,
            task.task_created_block,
            task.quorum_numbers,
            task.quorum_threshold_percentage,
        )
            .into();

        let contract_response = (task_response.task_index, task_response.response).into();

        let apk_g2 = (
            non_signer_stakes_and_signature.apkG2.X,
            non_signer_stakes_and_signature.apkG2.Y,
        )
            .into();

        let sigma = (
            non_signer_stakes_and_signature.sigma.X,
            non_signer_stakes_and_signature.sigma.Y,
        )
            .into();

        let quorum_apks = non_signer_stakes_and_signature
            .quorumApks
            .iter()
            .map(|apk| (apk.X, apk.Y).into())
            .collect();

        let non_signer_pubkeys = non_signer_stakes_and_signature
            .nonSignerPubkeys
            .iter()
            .map(|pubkey| (pubkey.X, pubkey.Y).into())
            .collect();

        let non_signer_stakes_and_signature = (
            non_signer_stakes_and_signature.nonSignerQuorumBitmapIndices,
            non_signer_pubkeys,
            quorum_apks,
            apk_g2,
            sigma,
            non_signer_stakes_and_signature.quorumApkIndices,
            non_signer_stakes_and_signature.totalStakeIndices,
            non_signer_stakes_and_signature.nonSignerStakeIndices,
        )
            .into();

        self.respondToTask(
            contract_task,
            contract_response,
            non_signer_stakes_and_signature,
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

        Ok(())
    }

    async fn raise_challenge(
        &self,
        task: Task<Self::Input>,
        task_response: TaskResponse<Self::Output>,
        task_response_metadata: TaskResponseMetadataSol,
        pubkeys_of_non_signing_operators: Vec<G1Point>,
    ) -> Result<(), TaskManagerError> {
        let contract_task = (
            task.input,
            task.task_created_block,
            task.quorum_numbers,
            task.quorum_threshold_percentage,
        )
            .into();

        let contract_response = (task_response.task_index, task_response.response).into();

        let task_response_metadata = (
            task_response_metadata.taskResponsedBlock,
            task_response_metadata.hashOfNonSigners,
        )
            .into();

        let pubkey_non_signer = pubkeys_of_non_signing_operators
            .iter()
            .map(|p| (p.X, p.Y).into())
            .collect();

        self.raiseAndResolveChallenge(
            contract_task,
            contract_response,
            task_response_metadata,
            pubkey_non_signer,
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

        Ok(())
    }
}
