use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
};
use bindings::iincrediblesquaringtaskmanager::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as ContractNonSignerStakesAndSignature;
use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{
    Task as ContractTask, TaskResponse as ContractTaskResponse,
};
use bindings::iincrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use bindings::iincrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use bindings::iincrediblesquaringtaskmanager::BN254::{G1Point, G2Point};
use eigen_aggregator::{Aggregator, AggregatorConfig};
use eigen_task_processor::task::Task;
use eigen_task_processor::task_manager::{TaskManagerContract, TaskManagerError};
use eigen_task_processor::task_response::TaskResponse;
use eigen_task_processor::IndexingTaskProcessor;
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use std::str::FromStr;

pub mod bindings;

impl<T, P, N> TaskManagerContract<T, P, N> for IncredibleSquaringTaskManagerInstance<T, P, N>
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
}

#[tokio::main]
async fn main() {
    let registry_coordinator =
        Address::from_str("0x7969c5ed335650692bc04293b07f5bf2e7a673c0").unwrap();
    let operator_state_retriever =
        Address::from_str("0x1429859428c0abc9c2c47c8ee9fbaf82cfa0f20f").unwrap();
    let http_rpc_url = "http://localhost:8545".to_string();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();
    let url = Url::parse(&http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

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
