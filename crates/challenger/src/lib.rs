//! # Challenger
//!
//! ## What is a Challenger
//!
//! A Challenger is a validator component that monitors the network for the creation of new tasks
//! and task responses submitted by operators, verifies their correctness, and raises challenges
//! when incorrect responses are detected. If the challenge is successful, the operator will be slashed.
//!
//! ## How the Logic Works
//!
//! The Challenger operates through a well-defined workflow:
//!
//! 1. **Event Subscription**:
//!    - Subscribes to blockchain events for new tasks and task responses
//!    - Monitors for `NewTaskEvent` to track new tasks created in the system
//!    - Watches for `TaskResponseEvent` when operators submit responses to tasks
//!
//! 2. **Verification Process**:
//!    - When a new task is detected, user defined logic is used to process the new task
//!    - When a task response is received, user defined logic is used to process the task response and verify it
//!    - Uses a user-defined verification function to determine if the response is correct
//!    - The verification logic can be customized based on the specific AVS requirements
//!
//! 3. **Challenge Mechanism**:
//!    - If a response is verified as correct, the challenger logs the result and takes no action
//!    - If a response is determined to be incorrect, the challenger raises a challenge
//!    - Includes identifying the non-signing operators who might have abstained from the incorrect response
//!
//! ## How to Set Up a Challenger
//!
//! 1. **Task Manager Definition**: Create a struct implementing the `TaskManagerDefs` trait that defines:
//!    - `Input` and `Output` types for your tasks
//!    - `NEW_TASK_EVENT_SELECTOR` - the event signature for new task events
//!    - Use the `impl_task_manager_from_defs_and_contract` macro to build your `TaskManager`.
//!
//!     ```ignore
//!         // Implement the [`TaskManagerDefs`] trait for a unit struct.
//!         // You need to specify the input and output types of the task.
//!         // You also need to specify the selectors for the new task event and the task responded event.
//!         pub struct ISTaskManager;
//!
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
//!
//! 2. **Task Verification Logic**: Define a function that computes the expected result for a task, which will be used to verify operator responses
//!    - This would be the logic to compute a new task.
//!
//!     ```ignore
//!         pub fn square(_task_index: u32, number_to_be_squared: U256) -> Result<U256, TaskManagerError> {
//!             Ok(number_to_be_squared * number_to_be_squared)
//!         }
//!     ```
//!
//! 3. **Response Calculator**: To abstract your computation into the operator, we provide a
//!    `ResponseCalculator` trait with a standard `FunctionResponseCalculator` struct.
//!    This struct implements the trait and helpers for turning your functions into implementations:
//!      - `response_calculator_from_fn`: Create a response calculator from your computation function.
//!      - `response_calculator_from_async_fn`: Create a response calculator from your async computation function.
//!
//!     ```ignore
//!         let response_calculator = response_calculator_from_fn(square);
//!     ```
//!
//! 4. **Verifier**: Create a verifier from the response calculator.
//!    - This will be in charge of computing the response of a task and comparing it with the operator's response.
//!
//!     ```ignore
//!         let logic = verifier_from_compute_function(response_calculator);
//!     ```
//!
//! 5. **Task Manager Contract**: Create an instance of your `TaskManager` contract:
//!     - This struct should come from your bindings.
//!
//!     ```ignore
//!         let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);
//!     ```
//!
//! 6. **Challenger Task Processor**: Create a [`ChallengerTaskProcessor`] trait implementation.
//!    - This will be in charge of processing the task and the response.
//!    - We provide a standard [`IndexingChallengerProcessor`](crate::challenger_processor::IndexingChallengerProcessor) implementation that can be used as a starting point.
//!
//!     ```ignore
//!         let task_processor = IndexingChallengerProcessor::new(contract, logic);
//!     ```
//!
//! 7. **Challenger Configuration**: Create a [`ChallengerConfig`] struct. This struct implements `Serialize` and `Deserialize` so you can load from a file.
//!    - Attributes:
//!      - `http_rpc_url`: The HTTP RPC URL of the Ethereum node
//!      - `ws_rpc_url`: The WebSocket RPC URL of the Ethereum node
//!
//! 8. **Challenger Initialization**: Initialize the [`Challenger`] with the configuration and start it with the processing logic
//!
//!     ```ignore
//!         let mut challenger = Challenger::new(config, task_processor);
//!         challenger.start_challenger().await?;
//!     ```
//!
//! ## Examples
//!
//! Here are some examples of challenger implementations:
//!
//! - [Incredible Squaring](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-squaring/src/bin/challenger.rs)
//! - [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-dot-product/src/bin/challenger.rs)
//! - [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/awesome-vault-service/src/bin/challenger.rs)

use alloy::{
    consensus::Transaction,
    dyn_abi::SolType,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::SolValue,
};
use challenger::ChallengerTaskProcessor;
use config::ChallengerConfig;
use eigen_common::{get_provider, get_ws_provider};
use eigen_task_manager::event_decoder::{
    decode_new_task, decode_params, decode_task_response_event, RespondToTaskCalldata,
};
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;
use error::ChallengerError;
use futures_util::StreamExt;
use tracing::info;

/// Challenger Task Processor trait
pub mod challenger;
/// Challenger Task Processor implementation
pub mod challenger_processor;
/// Challenger config
pub mod config;
/// Challenger error
pub mod error;

/// Main Challenger struct
#[derive(Debug)]
pub struct Challenger<TP: ChallengerTaskProcessor> {
    /// The rpc url
    rpc_url: String,
    /// The websocket url
    ws_url: String,
    /// The task processor
    task_processor: TP,
}

impl<TP: ChallengerTaskProcessor> Challenger<TP>
where
    TP::Input: From<<<TP::Input as SolValue>::SolType as SolType>::RustType>,
    TP::Output: From<<<TP::Output as SolValue>::SolType as SolType>::RustType>,
{
    /// Create a new challenger
    ///
    /// # Arguments
    ///
    /// * `config` - The challenger config
    /// * `task_processor` - The task processor
    ///
    /// # Returns
    ///
    /// * `Self` - The challenger
    pub fn new(config: ChallengerConfig, task_processor: TP) -> Self {
        Self {
            rpc_url: config.http_rpc_url,
            ws_url: config.ws_rpc_url,
            task_processor,
        }
    }

    /// Start the service and start listening for new tasks and task responses events
    /// It also checks if the response is correct, if not it raises a challenge.
    ///
    /// # Returns
    ///
    /// * `Result<(), ChallengerError>` - The result of the challenger
    pub async fn start_challenger(&mut self) -> Result<(), ChallengerError> {
        info!("challenger crate launched");

        let ws_provider = get_ws_provider(&self.ws_url).await?;

        // Subscribe to NewTaskEvent
        let task_filter = Filter::new().event_signature(TP::NEW_TASK_EVENT_SELECTOR);
        let mut task_stream = ws_provider
            .subscribe_logs(&task_filter)
            .await?
            .into_stream();

        // Subscribe to TaskResponseEvent
        let responded_filter = Filter::new().event_signature(TP::TASK_RESPONDED_EVENT_SELECTOR);
        let mut responded_stream = ws_provider
            .subscribe_logs(&responded_filter)
            .await?
            .into_stream();

        loop {
            tokio::select! {
                Some(log) = task_stream.next() => {
                    let (task_index, task) = decode_new_task(&log)?;
                    self.task_processor.handle_task_creation(task_index, task).await?;
                },
                Some(log) = responded_stream.next() => {
                    let (task_index, task_response, task_response_metadata) =
                        decode_task_response_event(&log).await?;

                    let non_signing_operator_pub_keys = self.get_non_signing_operator_pub_keys(log).await?;

                    self.task_processor
                        .handle_task_response(
                            task_index,
                            task_response,
                            task_response_metadata,
                            non_signing_operator_pub_keys,
                        )
                        .await?;
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    info!("challenger: No more logs to process, exiting loop.");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn get_non_signing_operator_pub_keys(
        &self,
        log: Log,
    ) -> Result<Vec<G1Point>, ChallengerError> {
        let tx_hash = log
            .transaction_hash
            .ok_or(ChallengerError::TransactionHashNotFound)?;
        let provider = get_provider(&self.rpc_url);

        // TODO: Review this, rust-analyzer is not able to infer the type of the transaction
        let tx: alloy::rpc::types::Transaction =
            provider
                .get_transaction_by_hash(tx_hash)
                .await?
                .ok_or(ChallengerError::TransactionNotFound(tx_hash.to_string()))?;

        // The first 4 bytes are the selector, so we skip them
        let calldata = tx
            .inner
            .input()
            .get(4..)
            .ok_or(ChallengerError::InvalidCalldata)?;

        // Decode the tuple of the form: Task<TM::Input>, TaskResponse<TM::Output>, NonSignerStakesAndSignature
        let decoded_calldata =
            decode_params::<RespondToTaskCalldata<TP::Input, TP::Output>>(calldata, false)?;

        Ok(decoded_calldata
            .2
            .nonSignerPubkeys
            .into_iter()
            .map(|pk| G1Point { X: pk.X, Y: pk.Y })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        hex::decode,
        primitives::{Bytes, U256},
        sol,
    };
    use eigen_task_manager::event_decoder::{NewTaskEventTuple, TaskResponseEventTuple};

    // The data of the log was taken from the IS example, it creates a new task with input 1, quorum 0 and threshold 40% in block 226
    #[tokio::test]
    async fn test_decode_new_task_event() {
        // Data from the log - NewTaskCreated event: (1, 226, 0, 40)
        let raw_hex = "\
            0000000000000000000000000000000000000000000000000000000000000020\
            0000000000000000000000000000000000000000000000000000000000000001\
            00000000000000000000000000000000000000000000000000000000000000e2\
            0000000000000000000000000000000000000000000000000000000000000080\
            0000000000000000000000000000000000000000000000000000000000000028\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000000";

        let raw_bytes: Vec<u8> = decode(raw_hex).unwrap();

        let data = raw_bytes.get(32..).unwrap().to_vec();

        let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
            decode_params::<NewTaskEventTuple<U256>>(&data, false).unwrap();

        assert_eq!(input, U256::ONE);
        assert_eq!(task_created_block, 226);
        assert_eq!(quorum_numbers, Bytes::from_static(&[0]));
        assert_eq!(quorum_threshold_percentage, 40);
    }

    // The data of the log was taken from the IS example, it responds with value 1 in block 227
    #[tokio::test]
    async fn test_decode_task_response_event() {
        let raw_hex = "\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000001\
            00000000000000000000000000000000000000000000000000000000000000e3\
            b569c9609dde655467765df81ebf2a34e4b9f40806475961b6676a2ec6115e61";

        let raw_bytes: Vec<u8> = decode(raw_hex).unwrap();

        let data = raw_bytes.as_slice();

        let ((task_index, response), metadata) =
            decode_params::<TaskResponseEventTuple<U256>>(data, false).unwrap();

        assert_eq!(task_index, 0);
        assert_eq!(response, U256::ONE);
        assert_eq!(metadata.taskResponsedBlock, 227);
    }

    // The data of the transaction was taken from the IS example, it is the calldata of respondToTask function
    #[tokio::test]
    async fn test_decode_non_signing_operator_pub_keys() {
        let raw_hex = "0x5baec9a0\
            0000000000000000000000000000000000000000000000000000000000000080\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000140\
            0000000000000000000000000000000000000000000000000000000000000001\
            00000000000000000000000000000000000000000000000000000000000000e2\
            0000000000000000000000000000000000000000000000000000000000000080\
            0000000000000000000000000000000000000000000000000000000000000028\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000180\
            00000000000000000000000000000000000000000000000000000000000001c0\
            0000000000000000000000000000000000000000000000000000000000000220\
            05ef93c6e1837bba80b06a34e998441d5f261b0ad76a8e415225bd45925d48df\
            24e25997f59c740a8a60c865f98e08bc5147e7fb7282394370e47222e2b8973d\
            19bdc911b43a044cb10296a0801fcc5c1c6fde70db274ba787f6a09fff1c4c90\
            2a8079e40eeca075fc24bdc26037ad71a62bdb1a43d9fd607114be25a33b0a24\
            0fa282ec956f0af83171f5ace5d00928d81fcc8c078d4bcf81332b55f5981f52\
            22e6f3d15ced518f0865c288b89b85603533a65b53e5decd369ec6133c91cf6c\
            0000000000000000000000000000000000000000000000000000000000000280\
            00000000000000000000000000000000000000000000000000000000000002c0\
            0000000000000000000000000000000000000000000000000000000000000300\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000001\
            009d50828897fe208275d989abddcad762bf1bb1a089d5ad40ca5dc78e20faac\
            256c79f6817fd79f3a4898e41b5212ccae66d5e9441c9c76f239a2966f24ba5e\
            0000000000000000000000000000000000000000000000000000000000000001\
            119b88fed50cc89205f5ebf794693f993b3e8489389c159676e9028f1a197b04\
            0e7dd29df4d13e5503470f1d7b113ec87ccb589ea361c5b3ce85a8202efa2e8c\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000020\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000000";

        let raw_bytes = decode(raw_hex).unwrap();

        // Remove the selector - 4 bytes
        let calldata = raw_bytes.get(4..).unwrap();

        let decoded_calldata =
            decode_params::<RespondToTaskCalldata<U256, U256>>(calldata, false).unwrap();

        let expected_pub_key = G1Point {
            X: U256::from_str_radix(
                "277950648056014144722774518899051149098728246263316284984520891067822832300",
                10,
            )
            .unwrap(),
            Y: U256::from_str_radix(
                "16927236637669640540790285431111034664564710839671197540688155537113438534238",
                10,
            )
            .unwrap(),
        };

        assert_eq!(
            decoded_calldata.2.nonSignerPubkeys.first().unwrap().X,
            expected_pub_key.X
        );
        assert_eq!(
            decoded_calldata.2.nonSignerPubkeys.first().unwrap().Y,
            expected_pub_key.Y
        );
    }

    #[test]
    fn test_decode_complex_type() {
        sol! {

            event NewComplex(
                uint32 indexed idx,
                ComplexInputSol input
            );

            #[derive(Debug, PartialEq)]
            struct DeepestSol {
                uint256 big;
                bytes    blob;
            }

            #[derive(Debug, PartialEq)]
            struct InnerSol {
                uint32      id;
                DeepestSol  d;
                uint32[]    refs;
            }

            #[derive(Debug, PartialEq)]
            struct ComplexInputSol {
                InnerSol inner;
                bytes     note;
                uint256 amount;
            }
        }

        let deepest = DeepestSol {
            big: U256::from(1u8),
            blob: Bytes::from_static(&[0xA0]),
        };

        let inner = InnerSol {
            id: 1,
            d: deepest.clone(),
            refs: vec![2],
        };

        let input = ComplexInputSol {
            inner: inner.clone(),
            note: Bytes::from_static(&[0xBB]),
            amount: U256::from(1u8),
        };

        // Data from the log - NewComplex event: (1, ComplexInputSol)
        let raw_hex = "\
            0000000000000000000000000000000000000000000000000000000000000020\
            0000000000000000000000000000000000000000000000000000000000000060\
            0000000000000000000000000000000000000000000000000000000000000180\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000060\
            00000000000000000000000000000000000000000000000000000000000000e0\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000040\
            0000000000000000000000000000000000000000000000000000000000000001\
            a000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000001\
            0000000000000000000000000000000000000000000000000000000000000002\
            0000000000000000000000000000000000000000000000000000000000000001\
            bb00000000000000000000000000000000000000000000000000000000000000";

        let raw_bytes = decode(raw_hex).unwrap();

        let data = raw_bytes.get(32..).unwrap();

        let decoded = decode_params::<ComplexInputSol>(data, false).unwrap();

        assert_eq!(decoded, input);
    }
}
