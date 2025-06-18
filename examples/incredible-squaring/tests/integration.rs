use std::{str::FromStr, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
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
    task_manager::response_calculator::response_calculator_from_fn,
    task_spammer::TaskSpammerBuilder,
    testing_utils::{
        anvil::start_anvil_container_with_incredible_squaring_state,
        task_processor::failing_response_calculator,
    },
};
use incredible_squaring::{
    bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance,
    square, ISTaskManager,
};

const AGGREGATOR_RPC_URL: &str = "127.0.0.1:8080";

#[tokio::test]
async fn test_incredible_squaring() {
    let (_container, http_endpoint, ws_endpoint) =
        start_anvil_container_with_incredible_squaring_state().await;

    println!("http_endpoint: {}", http_endpoint);
    println!("ws_endpoint: {}", ws_endpoint);

    init_logger(LogLevel::Info);
    let logger = get_test_logger();

    let aggregator_handle =
        tokio::spawn(start_aggregator(logger, http_endpoint.clone(), ws_endpoint));
    let spammer_handle = tokio::spawn(start_spammer(http_endpoint.clone()));

    aggregator_handle.await.unwrap();
    spammer_handle.await.unwrap();
}

async fn start_aggregator(logger: SharedLogger, http_endpoint: String, ws_endpoint: String) {
    let config = AggregatorConfig {
        server_address: AGGREGATOR_RPC_URL.to_string(),
        http_rpc_url: http_endpoint,
        ws_rpc_url: ws_endpoint,
        registry_coordinator: Address::from_str("0x7bc06c482dead17c0e297afbc32f6e63d3846650")
            .unwrap(),
        operator_state_retriever: Address::from_str("0x4c5859f0f772848b2d91f1d83e2fe57935348029")
            .unwrap(),
    };

    let signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&config.http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    let task_processor = IndexingAggregatorProcessor::new(
        contract,
        Duration::from_secs(60),
        Duration::from_secs(15),
    );
    let aggregator = Aggregator::new(config, task_processor, logger)
        .await
        .unwrap();
    aggregator.run().await.unwrap();
}

async fn start_spammer(http_endpoint: String) {
    let signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&http_endpoint).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    TaskSpammerBuilder::new(contract)
        .with_iter((0..).map(U256::from))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();
}

async fn start_operator(http_endpoint: String, ws_endpoint: String) {
    let registration_config = OperatorRegistrationConfig {
        signer: PrivateKeyConfig {
            private_key: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                .to_string(),
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
        registry_coordinator_address: Address::from_str(
            "0x7bc06c482dead17c0e297afbc32f6e63d3846650",
        )
        .unwrap(),
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
    };

    let config = OperatorConfig {
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
        registry_coordinator_address: Address::from_str(
            "0x7bc06c482dead17c0e297afbc32f6e63d3846650",
        )
        .unwrap(),
        aggregator_ip_port: AGGREGATOR_RPC_URL.to_string(),
        registration: Some(registration_config),
    };

    let response_calculator = response_calculator_from_fn(square);
    let logic = failing_response_calculator(response_calculator, || U256::from(42), 60);
    let operator = Operator::new(config, logic).await.unwrap();
    operator.run::<ISTaskManager>().await.unwrap();
}

async fn start_challenger(http_endpoint: String, ws_endpoint: String) {
    let config = ChallengerConfig {
        http_rpc_url: http_endpoint,
        ws_rpc_url: ws_endpoint,
    };

    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&config.http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);
    let response_calculator = response_calculator_from_fn(square);

    let logic = verifier_from_compute_function(response_calculator);
    let task_processor = IndexingChallengerProcessor::new(contract, logic);
    let mut challenger = Challenger::new(config, task_processor);
    challenger.run().await.unwrap();
}
