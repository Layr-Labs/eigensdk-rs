//! This is an example of how to use the aggregator.
use alloy::primitives::aliases::U96;
use alloy::primitives::{Address, B256};
use alloy::providers::WalletProvider;
use alloy::sol;
use alloy::sol_types::SolCall;
use alloy::transports::http::reqwest;
use eigen_aggregator::{
    config::AggregatorConfig,
    traits::{
        task_processor::{box_error, TaskProcessor, TaskProcessorError},
        task_response::TaskResponse,
    },
    Aggregator,
};

use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_client_elcontracts::writer::ELChainWriter;
use eigen_common::{get_provider, get_signer};
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_test_logger;
use eigen_logging::{init_logger, log_level::LogLevel};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_testing_utils::anvil_constants::{
    get_allocation_manager_address, get_avs_directory_address, get_delegation_manager_address,
    get_erc20_mock_strategy, get_rewards_coordinator_address, get_service_manager_address,
    get_strategy_manager_address, FIRST_ADDRESS, OPERATOR_BLS_KEY,
};
use eigen_testing_utils::{
    anvil::start_anvil_container,
    anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address, FIRST_PRIVATE_KEY,
    },
};
pub use eigen_types::operator::Operator;

use eigen_utils::slashing::core::allocationmanager::AllocationManager;
use eigen_utils::slashing::core::delegationmanager::DelegationManager;
use eigen_utils::slashing::middleware::slashingregistrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigen_utils::slashing::middleware::slashingregistrycoordinator::IStakeRegistryTypes::StrategyParams;
use eigen_utils::slashing::middleware::slashingregistrycoordinator::SlashingRegistryCoordinator;
use eigen_utils::slashing::sdk::mockavsservicemanager::MockAvsServiceManager;
use serde::{Deserialize, Serialize};
use tracing::info;

// Fake contract to emit event
sol! {
    #[allow(missing_docs)]
    #[derive(Debug)]
    // IF WE MODIFY THE CONTRACT, WE NEED TO UPDATE THE BYTECODE - `solc --bin <CONTRACT_NAME>.sol`
    #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506101868061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c80635867173014610038578063a747649314610056575b5f5ffd5b610040610060565b60405161004d91906100c3565b60405180910390f35b61005e610065565b005b5f5481565b5f5f81548092919061007690610109565b91905055505f547f0308e35d068a731dcf227a01af50313408b863f815670a72362dfa93fdd0686d60405160405180910390a2565b5f819050919050565b6100bd816100ab565b82525050565b5f6020820190506100d65f8301846100b4565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610113826100ab565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610145576101446100dc565b5b60018201905091905056fea264697066735822122066967740649d975b00c00d92c8ad08ecca35b10d89341e538ea5d4e4c3d0852b64736f6c634300081d0033")]
    contract TaskContract {
        uint256 public taskCounter;

        event NewTask(uint256 indexed taskIndex);

        function createTask() external {
            taskCounter++;
            emit NewTask(taskCounter);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FakeResponse {
    pub response: String,
}

impl TaskResponse for FakeResponse {
    fn digest(&self) -> B256 {
        B256::new([0x99; 32])
    }

    fn task_index(&self) -> u32 {
        1
    }
}

#[derive(Debug, Clone)]
struct MockTaskProcessor;

impl TaskProcessor for MockTaskProcessor {
    type NewTaskEvent = TaskContract::NewTask;
    type TaskResponse = FakeResponse;

    async fn process_new_task(
        &self,
        event: Self::NewTaskEvent,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        Ok(TaskMetadata::new(
            event.taskIndex.to::<u32>(),
            12345,
            vec![0],
            vec![50],
            std::time::Duration::from_secs(60),
        ))
    }

    async fn process_task_response(
        &self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        if response.response.is_empty() {
            return Err(box_error(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Empty result",
            )));
        }
        Ok(response.digest())
    }

    async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        println!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let (_container, http_rpc, ws_rpc) = start_anvil_container().await;

    let el_chain_writer = new_test_writer(http_rpc.clone(), FIRST_PRIVATE_KEY.to_string()).await;
    let avs_registry = AvsRegistryChainReader::new(
        get_test_logger(),
        get_registry_coordinator_address(http_rpc.clone()).await,
        get_operator_state_retriever_address(http_rpc.clone()).await,
        http_rpc.clone(),
    )
    .await
    .unwrap();

    // Deploy the task contract
    let provider = get_signer(FIRST_PRIVATE_KEY, &http_rpc);
    let task_contract = TaskContract::deploy(&provider).await.unwrap();
    let avs_address = get_service_manager_address(http_rpc.clone()).await;

    // Create quorums and operator sets
    create_total_delegated_stake_operator_set(
        &http_rpc,
        get_erc20_mock_strategy(http_rpc.clone()).await,
        avs_address,
    )
    .await;
    info!("Operator set created");

    // Register operator to operator set
    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
    el_chain_writer
        .register_for_operator_sets(FIRST_ADDRESS, avs_address, vec![0], bls_key_pair, "socket")
        .await
        .unwrap();
    info!("Operator registered to operator set");

    let operator_id = avs_registry.get_operator_id(FIRST_ADDRESS).await.unwrap();

    // Set up the aggregator config and initialize the processor
    let registry_coordinator = get_registry_coordinator_address(http_rpc.clone()).await;
    let operator_state_retriever = get_operator_state_retriever_address(http_rpc.clone()).await;
    let config = AggregatorConfig {
        server_address: "127.0.0.1:8081".to_string(),
        registry_coordinator,
        operator_state_retriever,
        http_rpc_url: http_rpc.clone(),
        ws_rpc_url: ws_rpc.clone(),
    };
    let processor = MockTaskProcessor;

    // Initialize the aggregator
    let aggregator = Aggregator::new(config, processor).await.unwrap();

    // Start the aggregator in the background
    let aggregator_handle = tokio::spawn(aggregator.start(ws_rpc.clone()));

    // Wait for the aggregator to initialize
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // EMIT A NEW TASK WITH EVENT
    let result = task_contract
        .createTask()
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    info!("Task created: {:?}", result.transaction_hash);

    // Send fake response from operator
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    tokio::spawn(async move {
        info!("Simulating operator response");

        let fake_response = FakeResponse {
            response: "Hello world".to_string(),
        };

        let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
        let bls_signature = bls_key_pair.sign_message(fake_response.digest().as_ref());

        let client = reqwest::Client::new();
        client
            .post("http://127.0.0.1:8081")
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "process_signed_task_response",
                "params": {
                    "params": {
                        "task_response": fake_response,
                        "signature": bls_signature,
                        "operator_id": operator_id
                    }
                },
                "id": 1
            }))
            .send()
            .await
            .unwrap();

        info!("Response sent");
    });

    // Keep the service running
    let result = aggregator_handle.await.unwrap();

    info!("Aggregator finished: {:?}", result);
}

// After Maxi's PR, we can remove these aux functions
async fn create_total_delegated_stake_operator_set(
    http_endpoint: &str,
    erc20_mock_strategy_addr: Address,
    avs_address: Address,
) {
    let default_signer = get_signer(FIRST_PRIVATE_KEY, http_endpoint);

    let allocation_manager_addr = get_allocation_manager_address(http_endpoint.to_string()).await;
    let allocation_manager =
        AllocationManager::new(allocation_manager_addr, default_signer.clone());

    let service_manager_address = get_service_manager_address(http_endpoint.to_string()).await;
    let service_manager =
        MockAvsServiceManager::new(service_manager_address, default_signer.clone());

    service_manager
        .setAppointee(
            default_signer.default_signer_address(),
            allocation_manager_addr,
            alloy::primitives::FixedBytes(AllocationManager::setAVSRegistrarCall::SELECTOR),
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    let registry_coordinator_addr =
        get_registry_coordinator_address(http_endpoint.to_string()).await;

    allocation_manager
        .setAVSRegistrar(avs_address, registry_coordinator_addr)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    service_manager
        .setAppointee(
            registry_coordinator_addr,
            allocation_manager_addr,
            alloy::primitives::FixedBytes(AllocationManager::createOperatorSetsCall::SELECTOR),
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    service_manager
        .setAppointee(
            registry_coordinator_addr,
            allocation_manager_addr,
            alloy::primitives::FixedBytes(
                AllocationManager::deregisterFromOperatorSetsCall::SELECTOR,
            ),
        )
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    let operator_set_param = OperatorSetParam {
        maxOperatorCount: 10,
        kickBIPsOfOperatorStake: 100,
        kickBIPsOfTotalStake: 1000,
    };

    let minimum_stake = U96::from(1);

    let strategy_params = StrategyParams {
        strategy: erc20_mock_strategy_addr,
        multiplier: U96::from(1),
    };

    let slashing_registry_coordinator = SlashingRegistryCoordinator::new(
        get_registry_coordinator_address(http_endpoint.to_string()).await,
        default_signer.clone(),
    );

    let tx_hash = slashing_registry_coordinator
        .createTotalDelegatedStakeQuorum(operator_set_param, minimum_stake, vec![strategy_params])
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    assert!(tx_hash.status());
}

pub async fn new_test_writer(http_endpoint: String, private_key: String) -> ELChainWriter {
    let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
    let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
    let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;
    let delegation_manager = get_delegation_manager_address(http_endpoint.clone()).await;
    let allocation_manager = get_allocation_manager_address(http_endpoint.clone()).await;
    let contract_delegation_manager =
        DelegationManager::new(delegation_manager, get_provider(&http_endpoint));
    let permission_controller = contract_delegation_manager
        .permissionController()
        .call()
        .await
        .unwrap()
        ._0;
    let registry_coordinator = get_registry_coordinator_address(http_endpoint.clone()).await;

    ELChainWriter::new(
        strategy_manager,
        rewards_coordinator,
        Some(permission_controller),
        Some(allocation_manager),
        registry_coordinator,
        el_chain_reader,
        http_endpoint.clone(),
        private_key,
    )
}

pub async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
    let delegation_manager_address = get_delegation_manager_address(http_endpoint.clone()).await;
    let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
    let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;

    ELChainReader::build(
        get_test_logger().clone(),
        delegation_manager_address,
        avs_directory_address,
        rewards_coordinator,
        &http_endpoint,
    )
    .await
    .unwrap()
}
