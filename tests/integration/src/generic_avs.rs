//! Generic AVS Integration Testing Framework
//!
//! This module provides helpers methods to run a AVS integration test.
//!
//! ## Overview
//!
//! The main goal is to launch a complete AVS testing environment by simply providing
//! configuration parameters. The module handles the orchestration of all necessary components:
//!
//! - **Aggregator**: Collects and aggregates responses from operators
//! - **Operator**: Processes tasks and submits responses
//! - **Challenger**: Validates responses and challenges incorrect ones
//! - **Task Spammer**: Generates tasks at specified intervals for testing
//!
//! ## Configuration
//!
//! The module offers flexible configuration options:
//!
//! ### Default Configuration
//!
//! The module includes pre-configured default values ([`AvsConfig::with_default_addresses_and_keys`])
//! for all contract addresses, private keys, and operational parameters that are used
//! across the example AVS implementations. The value of the addresses were taken from the values
//! obtained by running the example contracts (`/examples`) with anvil. The private keys came from
//! anvil available accounts.
//!
//! ### Custom Configuration
//!
//! While defaults cover most use cases, you can easily customize any aspect of the configuration
//! to match your specific AVS requirements, test scenarios, or deployment environments
//! creating a new [`AvsConfig`] struct.
//!

use std::{fmt::Debug, str::FromStr, time::Duration};

use alloy::{dyn_abi::SolType, primitives::Address, sol_types::SolValue};
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig, AggregatorError, IndexingAggregatorProcessor},
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        error::ChallengerError,
        Challenger,
    },
    crypto_bls::BlsPrivateKeyConfig,
    logging::logger::SharedLogger,
    operator::{
        config::OperatorConfig,
        error::OperatorError,
        register_config::{
            AvsRegistrationConfig, DepositInfo, OperatorELConfig, OperatorRegistrationConfig,
        },
        Operator,
    },
    signer::PrivateKeyConfig,
    task_manager::{response_calculator::ResponseCalculator, TaskManager},
    task_spammer::{error::TaskSpammerError, TaskSpammerBuilder},
};
use tokio::task::JoinHandle;

// Contracts addresses
const AVS_ADDRESS: &str = "0x5f3f1dbd7b74c6b46e8c44f98792a1daf8d69154";
const REGISTRY_COORDINATOR: &str = "0x7bc06c482dead17c0e297afbc32f6e63d3846650";
const OPERATOR_STATE_RETRIEVER_ADDRESS: &str = "0x4c5859f0f772848b2d91f1d83e2fe57935348029";
const ALLOCATION_MANAGER_ADDRESS: &str = "0x2279b7a0a67db372996a5fab50d91eaa73d2ebe6";
const DELEGATION_MANAGER_ADDRESS: &str = "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0";
const STRATEGY_MANAGER_ADDRESS: &str = "0x0165878a594ca255338adfa4d48449f69242eb8f";
const ERC20_STRATEGY_ADDRESS: &str = "0x2b961e3959b79326a8e7f64ef0d2d825707669b5";
const REWARDS_COORDINATOR_ADDRESS: &str = "0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0";
const AVS_DIRECTORY_ADDRESS: &str = "0x610178da211fef7d417bc0e6fed39f05609ad788";
const PERMISSION_CONTROLLER_ADDRESS: &str = "0x59b670e9fa9d0a427751af201d676719a970857b";

// Aggregator config
const AGGREGATOR_SIGNER: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

// Operator config
const OPERATOR_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const OPERATOR_SIGNER: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const OPERATOR_BLS_SIGNER: &str =
    "1371012690269088913462269866874713266643928125698382731338806296762673180359922";
const METADATA_URI: &str = "https://example.com/metadata";
const SOCKET: &str = "127.0.0.1:0";

/// Generic AVS configuration for integration tests
#[derive(Debug)]
pub struct AvsConfig<TM, RP, F>
where
    TM: TaskManager,
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    F: FnMut(u64) -> TM::Input + Send + 'static,
{
    /// Logger
    logger: SharedLogger,

    // Task Manager related
    /// Aggregator task manager instance
    pub aggregator_task_manager: TM,
    /// Challenger task manager instance
    pub challenger_task_manager: TM,
    /// Task spammer task manager instance
    pub task_spammer_task_manager: TM,

    // Response calculator with the compute logic
    /// Response calculator instance
    pub response_calculator: fn() -> RP,

    // Ethereum RPC endpoints
    /// URL of the Ethereum HTTP RPC
    pub http_rpc_url: String,
    /// URL of the Ethereum WebSocket RPC
    pub ws_rpc_url: String,

    // Contract addresses
    /// Address of the AVS contract
    pub avs_address: Address,
    /// Registry Coordinator contract address
    pub registry_coordinator_address: Address,
    /// Operator State Retriever contract address
    pub operator_state_retriever_address: Address,
    /// AVS Directory contract address
    pub avs_directory_address: Address,
    /// Allocation Manager contract address
    pub allocation_manager_address: Address,
    /// Delegation Manager contract address
    pub delegation_manager_address: Address,
    /// Strategy Manager contract address
    pub strategy_manager_address: Address,
    /// ERC20 Strategy contract address
    pub strategy_address: Address,
    /// Rewards Coordinator contract address
    pub rewards_coordinator_address: Address,
    /// Permission Controller contract address
    pub permission_controller_address: Address,

    // Aggregator related
    /// Private key of the aggregator
    pub aggregator_private_key: String,
    /// IP address and port the aggregation server will use
    pub aggregator_ip_port: String,
    /// Time before expiry of the task response aggregation
    pub time_to_expiry: Duration,
    /// Duration of the window to wait for signatures after quorum is reached
    pub window_duration: Duration,

    // Task Spammer related
    /// Interval between tasks
    pub task_interval: u64,
    /// Quorum threshold
    pub quorum_threshold: u8,
    /// Quorum participants
    pub quorums: Vec<u8>,
    /// Number of tasks
    pub num_tasks: u64,
    /// Input generator
    pub input: F,
    /// Timeout duration
    pub timeout: Duration,

    // Challenger related
    /// Private key of the challenger
    pub challenger_private_key: String,

    // Operator related
    /// Address of the operator
    pub operator_address: Address,
    /// Private key of the operator
    pub operator_private_key: String,
    /// BLS private key of the operator
    pub operator_bls_private_key: String,

    // Operator registration config values
    /// Metadata URI
    pub metadata_uri: String,
    /// Socket address
    pub socket: String,
    /// Allocation delay
    pub allocation_delay: u32,
    /// Operator set ID
    pub operator_set_id: u32,
    /// New magnitude to allocate
    pub new_magnitude: Vec<u64>,
    /// Deposit tokens amount
    pub deposit_tokens: String,
}

impl<TM, RP, F> AvsConfig<TM, RP, F>
where
    TM: TaskManager + Clone + Send + Sync + 'static,
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    F: FnMut(u64) -> TM::Input + Send + 'static,
{
    /// Create a new AVS configuration with pre-configured default addresses and keys.
    ///
    /// This function provides a convenient way to create an AVS configuration using the standard
    /// contract addresses and private keys from the example AVS implementations.
    ///
    /// The values of the addresses were taken from the values obtained by running the example contracts
    /// (`/examples`) with anvil. The private keys came from anvil available accounts.
    ///
    /// # Arguments
    ///
    /// * `aggregator_task_manager` - Task manager instance for the aggregator component
    /// * `challenger_task_manager` - Task manager instance for the challenger component  
    /// * `task_spammer_task_manager` - Task manager instance for the task spammer component
    /// * `response_calculator` - Function that returns a response calculator with the compute logic
    /// * `input` - Task input generator
    /// * `logger` - Logger
    /// * `timeout` - TaskSpammer timeout duration
    /// * `http_rpc_url` - HTTP RPC endpoint URL
    /// * `ws_rpc_url` - WebSocket RPC endpoint URL
    /// * `time_to_expiry` - Time until the task expires
    /// * `window_duration` - Duration of the window to wait for signatures after quorum is reached
    /// * `task_interval` - Interval between the creation of tasks
    /// * `quorum_threshold` - Thresholds for each quorum
    /// * `quorums` - Quorum numbers which should respond to the task
    /// * `num_tasks` - Number of tasks to spam
    /// * `deposit_tokens` - Deposit tokens amount
    /// * `new_magnitude` - New magnitude to allocate
    ///
    /// # Returns
    ///
    /// A fully configured [`AvsConfig`] with all default addresses and keys populated.
    #[allow(clippy::too_many_arguments)]
    pub fn with_default_addresses_and_keys(
        aggregator_task_manager: TM,
        challenger_task_manager: TM,
        task_spammer_task_manager: TM,
        response_calculator: fn() -> RP,
        input: F,
        logger: SharedLogger,
        timeout: Duration,
        http_rpc_url: String,
        ws_rpc_url: String,
        aggregator_ip_port: String,
        time_to_expiry: Duration,
        window_duration: Duration,
        task_interval: u64,
        quorum_threshold: u8,
        quorums: Vec<u8>,
        num_tasks: u64,
        deposit_tokens: String,
        new_magnitude: Vec<u64>,
    ) -> Self {
        AvsConfig {
            logger,

            // Task managers
            aggregator_task_manager,
            challenger_task_manager,
            task_spammer_task_manager,

            // Response calculator
            response_calculator,

            // Default RPC endpoints
            http_rpc_url,
            ws_rpc_url,

            // Aggregator defaults
            aggregator_private_key: AGGREGATOR_SIGNER.to_string(),
            aggregator_ip_port,
            time_to_expiry,
            window_duration,

            // Spammer defaults
            task_interval,
            quorum_threshold,
            quorums,
            num_tasks,
            input,
            timeout,

            // Challenger
            challenger_private_key: OPERATOR_SIGNER.to_string(),

            // Operator defaults
            operator_address: Address::from_str(OPERATOR_ADDRESS).unwrap(),
            operator_private_key: OPERATOR_SIGNER.to_string(),
            operator_bls_private_key: OPERATOR_BLS_SIGNER.to_string(),

            // Registration defaults
            metadata_uri: METADATA_URI.to_string(),
            socket: SOCKET.to_string(),
            allocation_delay: 0,
            operator_set_id: 0,
            new_magnitude,
            deposit_tokens,

            // AVS deployment
            avs_address: Address::from_str(AVS_ADDRESS).unwrap(),
            registry_coordinator_address: Address::from_str(REGISTRY_COORDINATOR).unwrap(),
            operator_state_retriever_address: Address::from_str(OPERATOR_STATE_RETRIEVER_ADDRESS)
                .unwrap(),

            // Core deployment
            avs_directory_address: Address::from_str(AVS_DIRECTORY_ADDRESS).unwrap(),
            allocation_manager_address: Address::from_str(ALLOCATION_MANAGER_ADDRESS).unwrap(),
            delegation_manager_address: Address::from_str(DELEGATION_MANAGER_ADDRESS).unwrap(),
            strategy_manager_address: Address::from_str(STRATEGY_MANAGER_ADDRESS).unwrap(),
            strategy_address: Address::from_str(ERC20_STRATEGY_ADDRESS).unwrap(),
            rewards_coordinator_address: Address::from_str(REWARDS_COORDINATOR_ADDRESS).unwrap(),
            permission_controller_address: Address::from_str(PERMISSION_CONTROLLER_ADDRESS)
                .unwrap(),
        }
    }
}

/// Start the AVS integration tests.
///
/// This will start the aggregator, operator, challenger and spammer services
/// and will wait for the spammer to finish. When the spammer completes, the other
/// three services are aborted to free resources.
///
/// The function will panic if any service returns an error or if the spammer does not finish
/// before the timeout.
///
/// # Arguments
///
/// * `config` - The configuration for the AVS integration tests.
pub async fn start_avs<TM, RP, F>(config: &AvsConfig<TM, RP, F>)
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: SolValue + Clone + PartialEq,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    TM::Input: Clone + Send + 'static,
    F: FnMut(u64) -> TM::Input + Send + 'static + Clone,
{
    let mut aggregator_handle = start_aggregator(config, config.logger.clone()).await;

    // Wait until the aggregator is ready
    tokio::time::sleep(Duration::from_secs(5)).await;

    let mut operator_handle = start_operator(config).await;
    let mut challenger_handle = start_challenger(config).await;

    // Wait until the operator and challenger are ready
    tokio::time::sleep(Duration::from_secs(5)).await;

    let mut spammer_handle = start_spammer(config, config.input.clone()).await;

    tokio::select! {
        // Spammer finished
        res = &mut spammer_handle => {
            res.unwrap().unwrap();
        }

        // Check if the services finished or if there was an error
        res = &mut aggregator_handle => {
            res.unwrap().unwrap();
        }
        res = &mut operator_handle => {
            res.unwrap().unwrap();
        }
        res = &mut challenger_handle => {
            res.unwrap().unwrap();
        }

        // Task spammer should finish before the timeout
        _ = tokio::time::sleep(config.timeout) => {
            panic!("timeout: TaskSpammer took too long. Aborting...");
        }
    }

    // Abort the handles
    aggregator_handle.abort();
    operator_handle.abort();
    challenger_handle.abort();
}

/// Start the aggregator.
///
/// # Arguments
///
/// * `config` - The configuration for the aggregator.
/// * `logger` - The logger.
///
/// # Returns
///
/// * `JoinHandle<()>` - The handle for the aggregator.
async fn start_aggregator<TM, RP, F>(
    config: &AvsConfig<TM, RP, F>,
    logger: SharedLogger,
) -> JoinHandle<Result<(), AggregatorError>>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    F: FnMut(u64) -> TM::Input + Send + 'static,
{
    let aggregator_config = AggregatorConfig {
        server_address: config.aggregator_ip_port.clone(),
        http_rpc_url: config.http_rpc_url.clone(),
        ws_rpc_url: config.ws_rpc_url.clone(),
        registry_coordinator: config.registry_coordinator_address,
        operator_state_retriever: config.operator_state_retriever_address,
    };
    let task_processor = IndexingAggregatorProcessor::new(
        config.aggregator_task_manager.clone(),
        config.time_to_expiry,
        config.window_duration,
    );
    Aggregator::new(aggregator_config, task_processor, logger)
        .await
        .unwrap()
        .start()
}

/// Start the operator
///
/// # Arguments
///
/// * `config` - The configuration for the operator
/// * `response_calculator` - The response calculator with the compute logic
///
/// # Returns
///
/// * `JoinHandle<()>` - The handle for the operator
async fn start_operator<RP, TM, F>(
    config: &AvsConfig<TM, RP, F>,
) -> JoinHandle<Result<(), OperatorError>>
where
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: SolValue + Clone,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    F: FnMut(u64) -> TM::Input + Send + 'static,
{
    let operator_global_config = OperatorELConfig {
        metadata_uri: Some(config.metadata_uri.clone()),
        allocation_delay: Some(config.allocation_delay),
        delegation_manager_address: Some(config.delegation_manager_address),
    };

    let avs_registration_config = AvsRegistrationConfig {
        avs_address: config.avs_address,
        operator_set_ids: vec![config.operator_set_id],
        socket: Some(config.socket.clone()),
        allocation_manager_address: Some(config.allocation_manager_address),
        registry_coordinator_address: Some(config.registry_coordinator_address),
        strategy_manager_address: Some(config.strategy_manager_address),
        deposits: vec![DepositInfo {
            strategy_address: config.strategy_address,
            amount: config.deposit_tokens.clone(),
            allocation_magnitude: config.new_magnitude[0],
        }],
    };

    let registration_config = OperatorRegistrationConfig {
        signer: PrivateKeyConfig {
            private_key: config.operator_private_key.clone(),
        }
        .into(),
        operator_global_config,
        avs_registration_config,
    };

    let operator_config = OperatorConfig {
        http_rpc_url: config.http_rpc_url.clone(),
        ws_rpc_url: config.ws_rpc_url.clone(),
        bls_signer: BlsPrivateKeyConfig {
            private_key: config.operator_bls_private_key.clone(),
        }
        .into(),
        operator_address: config.operator_address,
        registry_coordinator_address: config.registry_coordinator_address,
        aggregator_ip_port: config.aggregator_ip_port.clone(),
        registration: Some(registration_config),
    };
    Operator::new(operator_config, (config.response_calculator)())
        .await
        .unwrap()
        .start::<TM>()
}

/// Start the challenger
///
/// # Arguments
///
/// * `config` - The configuration for the challenger
/// * `response_calculator` - The response calculator with the compute logic
///
/// # Returns
///
/// * `JoinHandle<()>` - The handle for the challenger
async fn start_challenger<RP, TM, F>(
    config: &AvsConfig<TM, RP, F>,
) -> JoinHandle<Result<(), ChallengerError>>
where
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: SolValue + Clone + PartialEq,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    F: FnMut(u64) -> TM::Input + Send + 'static,
{
    let challenger_config = ChallengerConfig {
        http_rpc_url: config.http_rpc_url.clone(),
        ws_rpc_url: config.ws_rpc_url.clone(),
    };

    let logic = verifier_from_compute_function((config.response_calculator)());
    let challenger_task_processor =
        IndexingChallengerProcessor::new(config.challenger_task_manager.clone(), logic);
    Challenger::new(challenger_config, challenger_task_processor).start()
}

/// Start the spammer
///
/// # Arguments
///
/// * `config` - The configuration for the spammer
/// * `input` - The input that will be used to spam the tasks
///
/// # Returns
///
/// * `JoinHandle<Result<(), TaskSpammerError>>` - The handle for the spammer
async fn start_spammer<TM, RP, F>(
    config: &AvsConfig<TM, RP, F>,
    input: F,
) -> JoinHandle<Result<(), TaskSpammerError>>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: Clone + Send + 'static,
    F: FnMut(u64) -> TM::Input + Send + 'static,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
{
    TaskSpammerBuilder::new(config.task_spammer_task_manager.clone())
        .with_iter((0..config.num_tasks).map(input))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(config.task_interval))
        .build()
        .unwrap()
        .start()
}
