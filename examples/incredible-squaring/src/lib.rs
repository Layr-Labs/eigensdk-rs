//! Example AVS which squares a number

use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
    primitives::U256,
};
use bindings::incrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task as ContractTask, TaskResponse as ContractTaskResponse, TaskResponseMetadata,
};
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use bindings::incrediblesquaringtaskmanager::BN254::{G1Point as G1Binding, G2Point};
use bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as ContractNonSignerStakesAndSignature,
    IncredibleSquaringTaskManager::TaskResponded,
};
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
    type NewTaskEvent = NewTaskCreated;
    type TaskRespondedEvent = TaskResponded;

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
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), TaskManagerError> {
        let contract_task = ContractTask {
            numberToBeSquared: task.input,
            taskCreatedBlock: task.task_created_block,
            quorumNumbers: task.quorum_numbers,
            quorumThresholdPercentage: task.quorum_threshold_percentage,
        };

        let contract_response = ContractTaskResponse {
            numberSquared: response.response,
            referenceTaskIndex: response.task_index,
        };

        let apk_g2 = G2Point {
            X: non_signer_stakes_and_signature.apkG2.X,
            Y: non_signer_stakes_and_signature.apkG2.Y,
        };

        let sigma = G1Binding {
            X: non_signer_stakes_and_signature.sigma.X,
            Y: non_signer_stakes_and_signature.sigma.Y,
        };

        let quorum_apks = non_signer_stakes_and_signature
            .quorumApks
            .iter()
            .map(|apk| G1Binding { X: apk.X, Y: apk.Y })
            .collect();

        let non_signer_pubkeys = non_signer_stakes_and_signature
            .nonSignerPubkeys
            .iter()
            .map(|pubkey| G1Binding {
                X: pubkey.X,
                Y: pubkey.Y,
            })
            .collect();

        let non_signer_stakes_and_signature = ContractNonSignerStakesAndSignature {
            nonSignerStakeIndices: non_signer_stakes_and_signature.nonSignerStakeIndices,
            nonSignerQuorumBitmapIndices: non_signer_stakes_and_signature
                .nonSignerQuorumBitmapIndices,
            quorumApkIndices: non_signer_stakes_and_signature.quorumApkIndices,
            totalStakeIndices: non_signer_stakes_and_signature.totalStakeIndices,
            apkG2: apk_g2,
            quorumApks: quorum_apks,
            nonSignerPubkeys: non_signer_pubkeys,
            sigma,
        };

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
        let contract_task = ContractTask {
            numberToBeSquared: task.input,
            taskCreatedBlock: task.task_created_block,
            quorumNumbers: task.quorum_numbers,
            quorumThresholdPercentage: task.quorum_threshold_percentage,
        };

        let contract_response = ContractTaskResponse {
            numberSquared: task_response.response,
            referenceTaskIndex: task_response.task_index,
        };

        let task_response_metadata = TaskResponseMetadata {
            taskResponsedBlock: task_response_metadata.taskResponsedBlock,
            hashOfNonSigners: task_response_metadata.hashOfNonSigners,
        };

        let pubkey_non_signer = pubkeys_of_non_signing_operators
            .iter()
            .map(|p| G1Binding { X: p.X, Y: p.Y })
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
