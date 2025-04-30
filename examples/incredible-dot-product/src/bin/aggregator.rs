use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
};
use eigensdk::aggregator::{Aggregator, AggregatorConfig};
use eigensdk::task_processor::task_manager::TaskManagerError;
use eigensdk::task_processor::{
    task::Task, task_manager::TaskManagerContract, task_response::TaskResponse,
    IndexingTaskProcessor,
};
use eigensdk::utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;

use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use incredible_bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use incredible_bindings::incrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use std::str::FromStr;

pub struct TaskManagerWrapper<T, P, N>(pub IncredibleSquaringTaskManagerInstance<T, P, N>);

impl<T, P, N> TaskManagerContract<T, P, N> for TaskManagerWrapper<T, P, N>
where
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    type Input = U256;
    type Output = U256;
    type NewTaskEvent = NewTaskCreated;

    async fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), TaskManagerError> {
        // let contract_task = ContractTask {
        //     numberToBeSquared: task.input,
        //     taskCreatedBlock: task.task_created_block,
        //     quorumNumbers: task.quorum_numbers,
        //     quorumThresholdPercentage: task.quorum_threshold_percentage,
        // };

        // let contract_response = ContractTaskResponse {
        //     numberSquared: response.response,
        //     referenceTaskIndex: response.task_index,
        // };

        // let apk_g2 = G2Point {
        //     X: non_signer_stakes_and_signature.apkG2.X,
        //     Y: non_signer_stakes_and_signature.apkG2.Y,
        // };

        // let sigma = G1Point {
        //     X: non_signer_stakes_and_signature.sigma.X,
        //     Y: non_signer_stakes_and_signature.sigma.Y,
        // };

        // let quorum_apks = non_signer_stakes_and_signature
        //     .quorumApks
        //     .iter()
        //     .map(|apk| G1Point { X: apk.X, Y: apk.Y })
        //     .collect();

        // let non_signer_pubkeys = non_signer_stakes_and_signature
        //     .nonSignerPubkeys
        //     .iter()
        //     .map(|pubkey| G1Point {
        //         X: pubkey.X,
        //         Y: pubkey.Y,
        //     })
        //     .collect();

        // let non_signer_stakes_and_signature = ContractNonSignerStakesAndSignature {
        //     nonSignerStakeIndices: non_signer_stakes_and_signature.nonSignerStakeIndices,
        //     nonSignerQuorumBitmapIndices: non_signer_stakes_and_signature
        //         .nonSignerQuorumBitmapIndices,
        //     quorumApkIndices: non_signer_stakes_and_signature.quorumApkIndices,
        //     totalStakeIndices: non_signer_stakes_and_signature.totalStakeIndices,
        //     apkG2: apk_g2,
        //     quorumApks: quorum_apks,
        //     nonSignerPubkeys: non_signer_pubkeys,
        //     sigma,
        // };

        // self.respondToTask(
        //     contract_task,
        //     contract_response,
        //     non_signer_stakes_and_signature,
        // )
        // .send()
        // .await
        // .unwrap()
        // .get_receipt()
        // .await
        // .unwrap();

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);
    let task_processor = IndexingTaskProcessor::new(contract);

    let config = AggregatorConfig {
        server_address: "http://localhost:8080".to_string(),
        http_rpc_url: "http://localhost:8545".to_string(),
        ws_rpc_url: "ws://localhost:8545".to_string(),
        registry_coordinator,
        operator_state_retriever,
    };

    let aggregator = Aggregator::new(config, task_processor).await.unwrap();
    aggregator.start().await.unwrap();
}
