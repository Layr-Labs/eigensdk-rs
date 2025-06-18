use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::{Address, FixedBytes, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol_types::SolEvent,
    transports::http::reqwest::Url,
};
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig, IndexingAggregatorProcessor},
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        Challenger,
    },
    crypto_bls::BlsPrivateKeyConfig,
    logging::{get_test_logger, init_logger, log_level::LogLevel, logger::SharedLogger},
    operator::{config::OperatorConfig, register_config::OperatorRegistrationConfig, Operator},
    signer::PrivateKeyConfig,
    task_manager::{
        impl_task_manager_from_defs_and_contract, response_calculator::response_calculator_from_fn,
        TaskManagerDefs, TaskManagerError,
    },
    task_spammer::TaskSpammerBuilder,
    testing_utils::{
        anvil::start_anvil_container_with_state,
        anvil_constants::{
            get_allocation_manager_address, get_avs_directory_address,
            get_delegation_manager_address, get_erc20_mock_strategy,
            get_operator_state_retriever_address, get_permission_controller_address,
            get_registry_coordinator_address, get_rewards_coordinator_address,
            get_strategy_manager_address,
        },
    },
};

use crate::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    IncredibleSquaringTaskManagerInstance, NewTaskCreated, TaskResponded,
};

const AGGREGATOR_RPC_URL: &str = "127.0.0.1:8080";
const AGGREGATOR_SIGNER: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
const TASK_INTERVAL: u64 = 5;
const NUM_TASKS: u32 = 3;
const INCREDIBLE_SQUARING_STATE_PATH: &str =
    "./examples/incredible-squaring/contracts/anvil/incredible-squaring-anvil-state/state.json";
const OPERATOR_SIGNER: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const OPERATOR_BLS_SIGNER: &str =
    "0x1371012690269088913462269866874713266643928125698382731338806296762673180359922";
const TASK_MANAGER_ADDRESS: &str = "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3";
const OPERATOR_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

type IncredibleInstance = IncredibleSquaringTaskManagerInstance<
    (),
    alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::JoinFill<
                alloy::providers::Identity,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::GasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::BlobGasFiller,
                        alloy::providers::fillers::JoinFill<
                            alloy::providers::fillers::NonceFiller,
                            alloy::providers::fillers::ChainIdFiller,
                        >,
                    >,
                >,
            >,
            alloy::providers::fillers::WalletFiller<EthereumWallet>,
        >,
        alloy::providers::RootProvider,
    >,
>;

#[tokio::test]
async fn test_incredible_squaring() {
    let (_container, http_endpoint, ws_endpoint) =
        start_anvil_container_with_state(INCREDIBLE_SQUARING_STATE_PATH).await;

    init_logger(LogLevel::Info);
    let logger = get_test_logger();

    let aggregator_handle = tokio::spawn(start_aggregator(
        logger,
        http_endpoint.clone(),
        ws_endpoint.clone(),
    ));

    // Wait for the aggregator to start
    tokio::time::sleep(Duration::from_secs(5)).await;
    let operator_handle = tokio::spawn(start_operator(http_endpoint.clone(), ws_endpoint.clone()));

    // Wait for the operator to start
    tokio::time::sleep(Duration::from_secs(5)).await;
    let challenger_handle =
        tokio::spawn(start_challenger(http_endpoint.clone(), ws_endpoint.clone()));

    start_spammer(http_endpoint.clone()).await;

    // Task Spammer will finished after spamming 3 tasks
    // Give some time to the aggregator to process the last task
    tokio::time::sleep(Duration::from_secs(TASK_INTERVAL)).await;

    for handle in [aggregator_handle, operator_handle, challenger_handle] {
        handle.abort();
    }

    verify_tasks_completed(http_endpoint.clone()).await;
}

async fn verify_tasks_completed(http_endpoint: String) {
    let contract = create_task_manager_contract(http_endpoint, AGGREGATOR_SIGNER.to_string()).await;

    let latest_task_num = contract.latestTaskNum().call().await.unwrap()._0;
    assert_eq!(latest_task_num, NUM_TASKS);

    for task_index in 0..latest_task_num {
        let response_hash = contract
            .allTaskResponses(task_index)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(FixedBytes::<32>::default(), response_hash);
    }
}

async fn create_task_manager_contract(http_endpoint: String, signer: String) -> IncredibleInstance {
    let task_manager_address = Address::from_str(TASK_MANAGER_ADDRESS).unwrap();
    let url = Url::parse(&http_endpoint).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(&signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider)
}

async fn start_aggregator(logger: SharedLogger, http_endpoint: String, ws_endpoint: String) {
    let config = create_aggregator_config(http_endpoint.clone(), ws_endpoint).await;
    let contract = create_task_manager_contract(http_endpoint, AGGREGATOR_SIGNER.to_string()).await;
    let task_processor =
        IndexingAggregatorProcessor::new(contract, Duration::from_secs(5), Duration::from_secs(2));

    let aggregator = Aggregator::new(config, task_processor, logger)
        .await
        .unwrap();
    aggregator.run().await.unwrap();
}

async fn start_spammer(http_endpoint: String) {
    let contract = create_task_manager_contract(http_endpoint, AGGREGATOR_SIGNER.to_string()).await;

    TaskSpammerBuilder::new(contract)
        .with_iter((0..NUM_TASKS).map(U256::from))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(TASK_INTERVAL))
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();
}

async fn start_operator(http_endpoint: String, ws_endpoint: String) {
    let config = create_operator_config(http_endpoint.clone(), ws_endpoint).await;
    let response_calculator = response_calculator_from_fn(square);
    let operator = Operator::new(config, response_calculator).await.unwrap();
    operator.run::<ISTaskManager>().await.unwrap();
}

async fn start_challenger(http_endpoint: String, ws_endpoint: String) {
    let config = ChallengerConfig {
        http_rpc_url: http_endpoint.clone(),
        ws_rpc_url: ws_endpoint,
    };
    let contract =
        create_task_manager_contract(config.http_rpc_url.clone(), OPERATOR_SIGNER.to_string())
            .await;
    let response_calculator = response_calculator_from_fn(square);
    let logic = verifier_from_compute_function(response_calculator);
    let task_processor = IndexingChallengerProcessor::new(contract, logic);

    let mut challenger = Challenger::new(config, task_processor);
    challenger.run().await.unwrap();
}

async fn create_aggregator_config(http_endpoint: String, ws_endpoint: String) -> AggregatorConfig {
    AggregatorConfig {
        server_address: AGGREGATOR_RPC_URL.to_string(),
        http_rpc_url: http_endpoint.clone(),
        ws_rpc_url: ws_endpoint,
        registry_coordinator: get_registry_coordinator_address(http_endpoint.clone()).await,
        operator_state_retriever: get_operator_state_retriever_address(http_endpoint).await,
    }
}

async fn create_operator_config(http_endpoint: String, ws_endpoint: String) -> OperatorConfig {
    OperatorConfig {
        bls_signer: BlsPrivateKeyConfig {
            private_key: OPERATOR_BLS_SIGNER.to_string(),
        }
        .into(),
        operator_address: Address::from_str(OPERATOR_ADDRESS).unwrap(),
        operator_name: "squaring".to_string(),
        ws_rpc_url: ws_endpoint,
        http_rpc_url: http_endpoint.clone(),
        registry_coordinator_address: get_registry_coordinator_address(http_endpoint.clone()).await,
        aggregator_ip_port: AGGREGATOR_RPC_URL.to_string(),
        registration: Some(create_registration_config(&http_endpoint).await),
    }
}

async fn create_registration_config(http_endpoint: &str) -> OperatorRegistrationConfig {
    OperatorRegistrationConfig {
        signer: PrivateKeyConfig {
            private_key: OPERATOR_SIGNER.to_string(),
        }
        .into(),
        metadata_uri: "metadata".to_string(),
        socket: "socket".to_string(),
        allocation_delay: 0,
        operator_set_id: 0,
        new_magnitude: vec![1000000000000000000],
        deposit_tokens: "5000000000000000000000".to_string(),
        permission_controller_address: get_permission_controller_address(http_endpoint.to_string())
            .await,
        rewards_coordinator_address: get_rewards_coordinator_address(http_endpoint.to_string())
            .await,
        allocation_manager_address: get_allocation_manager_address(http_endpoint.to_string()).await,
        registry_coordinator_address: get_registry_coordinator_address(http_endpoint.to_string())
            .await,
        delegation_manager_address: get_delegation_manager_address(http_endpoint.to_string()).await,
        avs_directory_address: get_avs_directory_address(http_endpoint.to_string()).await,
        strategy_manager_address: get_strategy_manager_address(http_endpoint.to_string()).await,
        // 0x2b961e3959b79326a8e7f64ef0d2d825707669b5
        erc20_strategy_address: get_erc20_mock_strategy(http_endpoint.to_string()).await,
        avs_address: get_avs_directory_address(http_endpoint.to_string()).await,
        strategies_addresses: vec![get_erc20_mock_strategy(http_endpoint.to_string()).await],
    }
}

pub fn square(_task_index: u32, number_to_be_squared: U256) -> Result<U256, TaskManagerError> {
    Ok(number_to_be_squared * number_to_be_squared)
}

pub struct ISTaskManager;

impl TaskManagerDefs for ISTaskManager {
    type Input = U256;
    type Output = U256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => IncredibleSquaringTaskManagerInstance);
