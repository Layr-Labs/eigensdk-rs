use std::{fmt::Debug, time::Duration};

use alloy::{dyn_abi::SolType, primitives::Address, sol_types::SolValue};
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig, IndexingAggregatorProcessor},
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        Challenger,
    },
    crypto_bls::BlsPrivateKeyConfig,
    logging::logger::SharedLogger,
    operator::{config::OperatorConfig, register_config::OperatorRegistrationConfig, Operator},
    signer::PrivateKeyConfig,
    task_manager::{response_calculator::ResponseCalculator, TaskManager},
    task_spammer::TaskSpammerBuilder,
};
use tokio::task::JoinHandle;

/// Generic AVS configuration for integration tests
#[derive(Clone)]
pub struct AvsConfig<TM>
where
    TM: TaskManager,
{
    // Task Manager related
    /// Aggregator task manager instance
    pub aggregator_task_manager: TM,
    /// Challenger task manager instance
    pub challenger_task_manager: TM,
    /// Address of the task manager contract
    pub task_manager_address: Address,

    // Ethereum RPC
    /// URL of the Ethereum HTTP RPC
    pub http_rpc_url: String,
    /// URL of the Ethereum WebSocket RPC
    pub ws_rpc_url: String,

    // Avs deployment Addresses
    /// Address of the AVS contract
    pub avs_address: Address,
    /// Registry Coordinator contract address
    pub registry_coordinator_address: Address,
    /// Operator State Retriever contract address
    pub operator_state_retriever_address: Address,

    // Core deployment addresses
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

    // Aggregator RPC
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

    // Entities private keys
    /// Private key of the challenger
    pub challenger_private_key: String,

    // Operator related
    /// Address of the operator
    pub operator_address: Address,
    /// Name of the operator
    pub operator_name: String,
    /// Private key of the operator
    pub operator_private_key: String,
    /// BLS private key of the operator
    pub operator_bls_private_key: String,

    // Operator registration config values
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

/// Start the AVS integration tests.
///
/// # Arguments
///
/// * `config` - The configuration for the AVS integration tests.
/// * `response_calculator` - The response calculator with the compute logic
/// * `logger` - The logger
/// * `input` - The input that will be used to spam the tasks
///
/// # Returns
///
/// * `(JoinHandle<()>, JoinHandle<()>, JoinHandle<()>, JoinHandle<()>)` - The handles for the AVS integration tests.
pub async fn start_avs<TM, RP, F>(
    config: AvsConfig<TM>,
    response_calculator: RP,
    logger: SharedLogger,
    input: F,
) -> (
    JoinHandle<()>,
    JoinHandle<()>,
    JoinHandle<()>,
    JoinHandle<()>,
)
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: SolValue + Clone + PartialEq,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static + Clone,
    F: FnMut(u64) -> TM::Input + Send + 'static,
    TM::Input: Clone + Send + 'static,
{
    let aggregator_handle = start_aggregator(config.clone(), logger).await;

    // Wait until the aggregator is ready
    tokio::time::sleep(Duration::from_secs(5)).await;

    let operator_handle = start_operator(config.clone(), response_calculator.clone()).await;
    let challenger_handle = start_challenger(config.clone(), response_calculator).await;

    // Wait until the operator and challenger are ready
    tokio::time::sleep(Duration::from_secs(5)).await;

    let spammer_handle = start_spammer(config.clone(), input).await;

    (
        aggregator_handle,
        operator_handle,
        challenger_handle,
        spammer_handle,
    )
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
async fn start_aggregator<TM>(config: AvsConfig<TM>, logger: SharedLogger) -> JoinHandle<()>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
{
    let aggregator_config = AggregatorConfig {
        server_address: config.aggregator_ip_port,
        http_rpc_url: config.http_rpc_url,
        ws_rpc_url: config.ws_rpc_url,
        registry_coordinator: config.registry_coordinator_address,
        operator_state_retriever: config.operator_state_retriever_address,
    };
    let task_processor = IndexingAggregatorProcessor::new(
        config.aggregator_task_manager,
        config.time_to_expiry,
        config.window_duration,
    );
    let aggregator = Aggregator::new(aggregator_config, task_processor, logger)
        .await
        .unwrap();
    tokio::spawn(async move { aggregator.run().await.unwrap() })
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
async fn start_operator<RP, TM>(config: AvsConfig<TM>, response_calculator: RP) -> JoinHandle<()>
where
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: SolValue + Clone,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
{
    let registration_config = OperatorRegistrationConfig {
        signer: PrivateKeyConfig {
            private_key: config.operator_private_key,
        }
        .into(),
        metadata_uri: config.metadata_uri,
        socket: config.socket,
        allocation_delay: config.allocation_delay,
        operator_set_id: config.operator_set_id,
        new_magnitude: config.new_magnitude,
        deposit_tokens: config.deposit_tokens,
        permission_controller_address: config.permission_controller_address,
        rewards_coordinator_address: config.rewards_coordinator_address,
        allocation_manager_address: config.allocation_manager_address,
        registry_coordinator_address: config.registry_coordinator_address,
        delegation_manager_address: config.delegation_manager_address,
        avs_directory_address: config.avs_directory_address,
        strategy_manager_address: config.strategy_manager_address,
        erc20_strategy_address: config.strategy_address,
        avs_address: config.avs_address,
        strategies_addresses: vec![config.strategy_address],
    };

    let config = OperatorConfig {
        http_rpc_url: config.http_rpc_url,
        ws_rpc_url: config.ws_rpc_url,
        bls_signer: BlsPrivateKeyConfig {
            private_key: config.operator_bls_private_key,
        }
        .into(),
        operator_address: config.operator_address,
        operator_name: config.operator_name,
        registry_coordinator_address: config.registry_coordinator_address,
        aggregator_ip_port: config.aggregator_ip_port,
        registration: Some(registration_config),
    };
    let operator = Operator::new(config, response_calculator).await.unwrap();
    tokio::spawn(async move { operator.run::<TM>().await.unwrap() })
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
async fn start_challenger<RP, TM>(config: AvsConfig<TM>, response_calculator: RP) -> JoinHandle<()>
where
    RP: ResponseCalculator<TM::Input, TM::Output> + Send + Sync + 'static,
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: SolValue + Clone + PartialEq,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
{
    let challenger_config = ChallengerConfig {
        http_rpc_url: config.http_rpc_url,
        ws_rpc_url: config.ws_rpc_url,
    };

    let logic = verifier_from_compute_function(response_calculator);
    let challenger_task_processor =
        IndexingChallengerProcessor::new(config.challenger_task_manager, logic);
    let mut challenger = Challenger::new(challenger_config, challenger_task_processor);
    tokio::spawn(async move { challenger.run().await.unwrap() })
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
/// * `JoinHandle<()>` - The handle for the spammer
async fn start_spammer<TM, F>(config: AvsConfig<TM>, input: F) -> JoinHandle<()>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
    TM::Input: Clone + Send + 'static,
    F: FnMut(u64) -> TM::Input + Send + 'static,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
{
    tokio::spawn(async move {
        TaskSpammerBuilder::new(config.aggregator_task_manager)
            .with_iter((0..config.num_tasks).map(input))
            .with_quorum(50, vec![0])
            .with_interval(Duration::from_secs(config.task_interval))
            .build()
            .unwrap()
            .run()
            .await
            .unwrap()
    })
}
