use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::{Address, FixedBytes, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol_types::SolEvent,
    transports::http::reqwest::Url,
};
use eigen_testing_utils::anvil::start_anvil_container_with_state;
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
};

use crate::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
    IncredibleSquaringTaskManagerInstance, NewTaskCreated, TaskResponded,
};

const AGGREGATOR_RPC_URL: &str = "127.0.0.1:8080";
const TASK_INTERVAL: u64 = 5;
const NUM_TASKS: u32 = 3;

const INCREDIBLE_SQUARING_STATE_PATH: &str =
    "../../../examples/incredible-squaring/contracts/anvil/incredible-squaring-anvil-state/state.json";

const AGGREGATOR_SIGNER: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
const OPERATOR_SIGNER: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const TASK_MANAGER_ADDRESS: &str = "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3";
const REGISTRY_COORDINATOR: &str = "0x7bc06c482dead17c0e297afbc32f6e63d3846650";

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

    verify_tasks_completed(&http_endpoint).await;
}

async fn verify_tasks_completed(http_endpoint: &str) {
    let contract = create_task_manager_contract(http_endpoint, AGGREGATOR_SIGNER).await;

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

async fn create_task_manager_contract(http_endpoint: &str, signer: &str) -> IncredibleInstance {
    let task_manager_address = Address::from_str(TASK_MANAGER_ADDRESS).unwrap();
    let url = Url::parse(http_endpoint).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider)
}

async fn start_aggregator(logger: SharedLogger, http_endpoint: String, ws_endpoint: String) {
    let config = create_aggregator_config(http_endpoint, ws_endpoint);
    let contract = create_task_manager_contract(&config.http_rpc_url, AGGREGATOR_SIGNER).await;
    let task_processor =
        IndexingAggregatorProcessor::new(contract, Duration::from_secs(5), Duration::from_secs(2));

    let aggregator = Aggregator::new(config, task_processor, logger)
        .await
        .unwrap();
    aggregator.run().await.unwrap();
}

async fn start_spammer(http_endpoint: String) {
    let contract = create_task_manager_contract(&http_endpoint, AGGREGATOR_SIGNER).await;

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
    let config = create_operator_config(http_endpoint, ws_endpoint);
    let response_calculator = response_calculator_from_fn(square);
    let operator = Operator::new(config, response_calculator).await.unwrap();
    operator.run::<ISTaskManager>().await.unwrap();
}

async fn start_challenger(http_endpoint: String, ws_endpoint: String) {
    let config = ChallengerConfig {
        http_rpc_url: http_endpoint,
        ws_rpc_url: ws_endpoint,
    };
    let contract = create_task_manager_contract(&config.http_rpc_url, OPERATOR_SIGNER).await;
    let response_calculator = response_calculator_from_fn(square);
    let logic = verifier_from_compute_function(response_calculator);
    let task_processor = IndexingChallengerProcessor::new(contract, logic);

    let mut challenger = Challenger::new(config, task_processor);
    challenger.run().await.unwrap();
}

fn create_aggregator_config(http_endpoint: String, ws_endpoint: String) -> AggregatorConfig {
    AggregatorConfig {
        server_address: AGGREGATOR_RPC_URL.to_string(),
        http_rpc_url: http_endpoint,
        ws_rpc_url: ws_endpoint,
        registry_coordinator: Address::from_str(REGISTRY_COORDINATOR).unwrap(),
        operator_state_retriever: Address::from_str("0x4c5859f0f772848b2d91f1d83e2fe57935348029")
            .unwrap(),
    }
}

fn create_operator_config(http_endpoint: String, ws_endpoint: String) -> OperatorConfig {
    OperatorConfig {
        bls_signer: BlsPrivateKeyConfig {
            private_key:
                "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                    .to_string(),
        }
        .into(),
        operator_address: Address::from_str("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266").unwrap(),
        operator_name: "squaring".to_string(),
        ws_rpc_url: ws_endpoint,
        http_rpc_url: http_endpoint,
        registry_coordinator_address: Address::from_str(REGISTRY_COORDINATOR).unwrap(),
        aggregator_ip_port: AGGREGATOR_RPC_URL.to_string(),
        registration: Some(create_registration_config()),
    }
}

fn create_registration_config() -> OperatorRegistrationConfig {
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
        permission_controller_address: Address::from_str(
            "0x59b670e9fa9d0a427751af201d676719a970857b",
        )
        .unwrap(),
        rewards_coordinator_address: Address::from_str(
            "0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0",
        )
        .unwrap(),
        allocation_manager_address: Address::from_str("0x2279b7a0a67db372996a5fab50d91eaa73d2ebe6")
            .unwrap(),
        registry_coordinator_address: Address::from_str(REGISTRY_COORDINATOR).unwrap(),
        delegation_manager_address: Address::from_str("0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0")
            .unwrap(),
        avs_directory_address: Address::from_str("0x610178da211fef7d417bc0e6fed39f05609ad788")
            .unwrap(),
        strategy_manager_address: Address::from_str("0x0165878a594ca255338adfa4d48449f69242eb8f")
            .unwrap(),
        erc20_strategy_address: Address::from_str("0x2b961e3959b79326a8e7f64ef0d2d825707669b5")
            .unwrap(),
        avs_address: Address::from_str("0x5f3f1dbd7b74c6b46e8c44f98792a1daf8d69154").unwrap(),
        strategies_addresses: vec![
            Address::from_str("0x2b961e3959b79326a8e7f64ef0d2d825707669b5").unwrap(),
        ],
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
