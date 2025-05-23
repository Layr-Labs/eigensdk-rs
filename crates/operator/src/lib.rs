//! # Operator
//!
//! ## What is an Operator
//!
//! Operators are off-chain nodes that perform, sign, and submit verifiable computations for
//! Autonomous Verifiable Services (AVSs) using Ethereum restaking for security. They first register on
//! EigenLayer’s core contracts, then opt-in to provide a range of services to AVSs.
//! Operators listen for new task events, execute the supplied computation logic,
//! cryptographically sign the results with `BLS/ECDSA` keys, and finally send the proofs
//! to an aggregator for final consolidation.
//!
//! ## How the Logic Works
//!
//! The Operator functions through the following flow:
//!
//! 1. **Task Subscription**:
//!    - The operator subscribes to specific event signatures emitted by task processors
//!    - Uses WebSocket connection to listen to blockchain events
//!    - Filters only for the specific task type it's designed to handle
//!
//! 2. **Task Processing**:
//!    - When a new task is detected, it extracts the task index and input data
//!    - Applies a computation function to the input data. This computation function is provided when starting the operator.
//!
//! 3. **Response Signing**:
//!    - Signs the computed result using the operator's BLS Key Pair.
//!    - Creates a `SignedTaskResponse` containing the result, signature, and operator ID
//!
//! 4. **Response Submission**:
//!    - Sends the signed response to an Aggregator service through a RPC request.
//!
//! ## How to Set Up an Operator
//!
//! 1. **Task Manager Definition**: Create a struct implementing the [`TaskManagerDefs`] trait:
//!    - [`Input`](eigen_task_manager::TaskManagerDefs::Input) and [`Output`](eigen_task_manager::TaskManagerDefs::Output)
//!      types for your tasks. This should come from your bindings.
//!    - [`NEW_TASK_EVENT_SELECTOR`](eigen_task_manager::TaskManagerDefs::NEW_TASK_EVENT_SELECTOR) - the event signature for new task events
//!    - Use the [`impl_task_manager_from_defs_and_contract`](eigen_task_manager::impl_task_manager_from_defs_and_contract)
//!      macro to build your `TaskManager`.
//!
//!     ```ignore
//!         impl TaskManagerDefs for ISTaskManager {
//!             type Input = U256;
//!             type Output = U256;
//!             const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
//!             const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
//!         }
//!
//!         impl_task_manager_from_defs_and_contract!(ISTaskManager => YOUR_BINDING_CONTRACT_INSTANCE);
//!     ```
//!
//! 2. **Create the operator configuration**: Create a [`OperatorConfig`](crate::config::OperatorConfig) struct.
//!    This structs implements `Serialize` and `Deserialize` so you can load from a file.
//!    - Attributes:
//!      - `bls_private_key`: The BLS private key for
//!      - `operator_address`: The address of the operator
//!      - `operator_name`: The name of the operator
//!      - `ws_rpc_url`: The WebSocket RPC URL of the Ethereum node
//!      - `http_rpc_url`: The HTTP RPC URL of the Ethereum node
//!      - `registry_coordinator_address`: The address of the registry coordinator
//!      - `operator_state_retriever_address`: The address of the operator state retriever
//!      - `aggregator_ip_port`: The IP and port of the aggregator
//!      - `registration`: The registration of the operator. If you don't want to register the operator, you can set this to `None`.
//!
//! 3. **Processing Logic**: Implement the computation function that processes task inputs and produces outputs
//!    - This function will be called when the operator receives a `NEW_TASK_EVENT_SELECTOR` event.
//!
//!     ```ignore
//!         // Your custom logic to process the input and generate a response.
//!         // Example: square the input.
//!         pub async fn square(
//!             task_index: u32,
//!             number_to_be_squared: U256
//!         ) -> Result<U256, TaskManagerError> {
//!             Ok(number_to_be_squared * number_to_be_squared)
//!         }
//!     ```
//!
//! 4. **Response Calculator**: To abstract your computation into the operator, we provide a `ResponseCalculator`
//!    trait with a standar `FunctionResponseCalculator` struct. This struct implements the trait and helpers
//!    for turning your functions into implementations:
//!    - `response_calculator_from_fn`: Create a response calculator from your computation function.
//!    - `response_calculator_from_async_fn`: Create a response calculator from your async computation function.
//!
//!     ```ignore
//!         let response_calculator = response_calculator_from_fn(square);
//!     ```
//!
//!    - In case you need to save state in the operator, you can use your own struct implementing the `ResponseCalculator` trait.
//!
//! 5. **Failing Response Calculator**: If you want to test what happens when the operator responds incorrectly
//!    to a task and see how slashing works, you can wrap your logic with `failing_response_calculator` (from
//!    `eigen-testing-utils`), to inject failures and a given failure rate. **Use this for testing purposes only.**
//!     
//!     ```ignore
//!         let logic = failing_response_calculator(response_calculator, || U256::from(42), 60);
//!     ```
//!
//! 6. **Run the operator**: Initialize the [`Operator`] with the configuration and start it with the processing logic
//!
//!     ```ignore
//!         let operator = Operator::new(logger, config).await.unwrap();
//!         operator.start::<ISTaskManager>(logic).await.unwrap();
//!     ```
//!
//! ## Examples
//!
//! Here are some examples of operators that are already implemented:
//!
//! - [Incredible Squaring](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-squaring/src/bin/operator.rs)
//! - [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-dot-product/src/bin/operator.rs)
//! - [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/awesome-vault-service/src/bin/operator.rs)
//!
//! ## How to implement a custom Response Calculator
//!
//! To implement a custom Response Calculator, you need to implement the [`ResponseCalculator`] trait.
//!
//! The struct needs to implement the [`compute_response`](ResponseCalculator::compute_response) method. This method will be called
//! when the operator receives a [`NEW_TASK_EVENT_SELECTOR`](eigen_task_manager::TaskManagerDefs::NEW_TASK_EVENT_SELECTOR)
//! event. This should contain the logic to compute the response for a given task.
//!
//! We recommend implementing your own Response Calculator if you need to save state between operator responses.
//! If you don't need to save a state, you can use the standard [`FunctionResponseCalculator`](eigen_task_manager::response_calculator::FunctionResponseCalculator)
//! implementation.
//!
//! Refer to the [`FunctionResponseCalculator`](eigen_task_manager::response_calculator::FunctionResponseCalculator)
//! implementation for an example of how to implement a custom Response Calculator.
//!

use alloy::{
    dyn_abi::SolType,
    primitives::keccak256,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    sol_types::SolValue,
};
use client::ClientAggregator;
use eigen_aggregator::SignedTaskResponse;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::logger::SharedLogger;
use eigen_task_manager::{event_decoder::decode_new_task, task_response::TaskResponse};
use eigen_task_manager::{response_calculator::ResponseCalculator, TaskManagerDefs};
use eigen_types::operator::OperatorId;
use error::OperatorError;
use futures_util::StreamExt;
use registration::register_operator;
use tracing::{error, info};

/// Tarpc Client
pub mod client;
/// Operator config
pub mod config;
/// Operator error
pub mod error;
/// Operator registration config
pub mod register_config;
/// Operator registration utils
pub mod registration;

/// The operator listens for [`NEW_TASK_EVENT_SELECTOR`](eigen_task_manager::TaskManagerDefs::NEW_TASK_EVENT_SELECTOR)
/// events, computes the task response, and signs it with the operator's BLS key pair.
/// Then, it sends the signed task response to the aggregator via RPC.
///
/// To more in-depth details about the operator, refer to the [module documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L110).
#[derive(Debug)]
pub struct Operator {
    operator_id: OperatorId,
    operator_name: String,
    client_aggregator: ClientAggregator,
    ws_rpc_url: String,
    key_pair: BlsKeyPair,
}

impl Operator {
    /// Initialize a new operator.
    /// This method does not register the operator.
    ///
    /// # Arguments
    ///
    /// * `key_pair` - The key pair of the operator.
    /// * `operator_address` - The address of the operator.
    /// * `operator_name` - The name of the operator.
    /// * `logger` - The logger.
    /// * `ws_rpc_url` - The URL of the WebSocket RPC.
    /// * `http_rpc_url` - The URL of the HTTP RPC.
    /// * `registry_coordinator_address` - The address of the registry coordinator.
    /// * `operator_state_retriever_address` - The address of the operator state retriever.
    /// * `aggregator_ip_port` - The IP and port of the aggregator.
    ///
    /// # Returns
    ///
    /// * `Result<Self, OperatorError>` - The operator.
    pub async fn new(
        logger: SharedLogger,
        config: config::OperatorConfig,
    ) -> Result<Self, OperatorError> {
        let config::OperatorConfig {
            bls_private_key,
            operator_address,
            operator_name,
            ws_rpc_url,
            http_rpc_url,
            registry_coordinator_address,
            operator_state_retriever_address,
            aggregator_ip_port,
            registration: _,
        } = config;
        let avs_registry_reader = AvsRegistryChainReader::new(
            logger.clone(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_rpc_url.to_string(),
        )
        .await?;

        let key_pair = BlsKeyPair::new(bls_private_key)?;

        // Check if the operator is registered with EigenLayer
        if !avs_registry_reader
            .is_operator_registered(operator_address)
            .await?
        {
            // Check if a registration config was provided
            let Some(registration_config) = config.registration else {
                error!(
                    "Operator {} not registered and no registration config was provided",
                    operator_name
                );
                return Err(OperatorError::RegistrationError);
            };

            register_operator(registration_config, logger, http_rpc_url, key_pair.clone()).await?;
            info!("Operator {} registered successfully", operator_name);
        }

        let client_aggregator = ClientAggregator::new(aggregator_ip_port).await?;

        let operator_id = avs_registry_reader
            .get_operator_id(operator_address)
            .await
            .map_err(|_| OperatorError::OperatorIdError)?;

        Ok(Self {
            operator_id,
            operator_name: operator_name.to_string(),
            ws_rpc_url: ws_rpc_url.to_string(),
            client_aggregator: client_aggregator.clone(),
            key_pair,
        })
    }

    /// Start listening for new task events. Operator subscribe to the event signature of the task processor.
    /// When a new task is created, the operator will process it and send the signed task response to the aggregator.
    /// User must to use the structs generated by the bindings because they implement the SolType and SolValue traits.
    ///
    /// # Arguments
    ///
    /// * `self` - The operator.
    /// * `response_calculator` - The response calculator that computes the response of a task
    ///
    /// # Returns
    ///
    /// * `Result<(), OperatorError>` - The result of the operation.
    pub async fn start<TM>(
        &self,
        response_calculator: impl ResponseCalculator<TM::Input, TM::Output>,
    ) -> Result<(), OperatorError>
    where
        TM: TaskManagerDefs,
        TM::Input:
            From<<<<TM as TaskManagerDefs>::Input as SolValue>::SolType as SolType>::RustType>,
        TM::Output: SolValue + Clone,
        TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    {
        let ws = WsConnect::new(&self.ws_rpc_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .map_err(|_| OperatorError::TransportError)?;

        let filter = Filter::new().event_signature(TM::NEW_TASK_EVENT_SELECTOR);
        let sub = provider
            .subscribe_logs(&filter)
            .await
            .map_err(|_| OperatorError::SubscribeLogsError)?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let (task_index, task) = decode_new_task::<TM::Input>(&log)?;

            info!("{} picked up a new task", self.operator_name);

            let output = response_calculator
                .compute_response(task_index, task.input)
                .await?;
            let task_response = TaskResponse {
                task_index,
                response: output,
            };
            let signed_task_response =
                Self::sign_task_response(&self.key_pair, &self.operator_id, task_response)?;

            self.client_aggregator
                .send_signed_task_response(signed_task_response)
                .await?;
        }

        Ok(())
    }

    /// Sign the task response for the aggregator.
    ///
    /// # Arguments
    ///
    /// * `key_pair` - The key pair of the operator.
    /// * `operator_id` - The id of the operator.
    /// * `task_response` - The task response to sign.
    ///
    /// # Returns
    ///
    /// * `Result<SignedTaskResponse<Response>, OperatorError>` - The signed task response.
    fn sign_task_response<Response>(
        key_pair: &BlsKeyPair,
        operator_id: &OperatorId,
        task_response: TaskResponse<Response>,
    ) -> Result<SignedTaskResponse<Response>, OperatorError>
    where
        Response: SolValue + Clone,
        Response: From<<<Response as SolValue>::SolType as SolType>::RustType>,
    {
        let encoded = task_response.encode();
        let hash_msg = keccak256(encoded);
        let signed_msg = key_pair.sign_message(&hash_msg);
        let signed_task_response = SignedTaskResponse::new(task_response, signed_msg, *operator_id);
        info!("Operator signed task response");
        Ok(signed_task_response)
    }
}
