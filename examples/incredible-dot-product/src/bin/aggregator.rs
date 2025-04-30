use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
};
use eigensdk::aggregator::Aggregator;
use eigensdk::common::get_signer;
use eigensdk::task_processor::task_manager::TaskManagerError;
use eigensdk::task_processor::{
    task::Task, task_manager::TaskManagerContract, task_response::TaskResponse,
    IndexingTaskProcessor,
};
use eigensdk::utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;

use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::incrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as ContractNonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{
        DotProductInput, DotProductResult, Task as ContractTask,
        TaskResponse as ContractTaskResponse,
    },
};
use incredible_dot_product::config::Config;

// We need to implement this struct to avoid "must be used as the type parameter for some local type" error
// With the new struct, there is no problem when implementing the TaskManagerContract trait to the external binding struct
#[derive(Clone, Debug)]
pub struct TaskManagerWrapper<T, P, N>(pub IncredibleSquaringTaskManagerInstance<T, P, N>);

impl<T, P, N> TaskManagerContract<T, P, N> for TaskManagerWrapper<T, P, N>
where
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    // TODO: Check bounds of Input and Output in the SDK:
    //       I need to derive Debug, Serialize and Deserialize for DotProductInput and DotProductResult in the bindings
    type Input = DotProductInput;
    type Output = DotProductResult;
    type NewTaskEvent = NewTaskCreated;

    async fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), TaskManagerError> {
        let contract_task = ContractTask {
            input: task.input,
            quorumNumbers: task.quorum_numbers,
            quorumThresholdPercentage: task.quorum_threshold_percentage,
            taskCreatedBlock: task.task_created_block,
        };

        let contract_response = ContractTaskResponse {
            referenceTaskIndex: response.task_index,
            output: response.response,
        };

        let apk_g2 = G2Point {
            X: non_signer_stakes_and_signature.apkG2.X,
            Y: non_signer_stakes_and_signature.apkG2.Y,
        };

        let sigma = G1Point {
            X: non_signer_stakes_and_signature.sigma.X,
            Y: non_signer_stakes_and_signature.sigma.Y,
        };

        let quorum_apks = non_signer_stakes_and_signature
            .quorumApks
            .iter()
            .map(|apk| G1Point { X: apk.X, Y: apk.Y })
            .collect();

        let non_signer_pubkeys = non_signer_stakes_and_signature
            .nonSignerPubkeys
            .iter()
            .map(|pubkey| G1Point {
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

        self.0
            .respondToTask(
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
}

#[tokio::main]
async fn main() {
    let config = Config::load_from("config.toml");
    let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

    let contract =
        IncredibleSquaringTaskManagerInstance::new(config.contract_address.task_manager, wallet);
    let wrapper_contract = TaskManagerWrapper(contract);

    let task_processor = IndexingTaskProcessor::new(wrapper_contract);

    let aggregator = Aggregator::new(config.aggregator_config, task_processor)
        .await
        .unwrap();
    aggregator.start().await.unwrap();
}
