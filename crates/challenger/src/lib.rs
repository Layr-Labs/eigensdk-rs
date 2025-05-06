#![allow(missing_docs)]
use alloy::{
    consensus::Transaction,
    dyn_abi::{abi::TokenSeq, SolType},
    primitives::Bytes,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::{SolEvent, SolValue},
};
use challenger::ChallengerTaskProcessor;
use challenger_processor::TaskResponseMetadataSol;
use eigen_common::{get_provider, get_ws_provider};
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use eigen_utils::slashing::middleware::iblssignaturechecker::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature, BN254::G1Point,
};
use error::ChallengerError;
use futures_util::StreamExt;
use tracing::info;

pub mod challenger;
pub mod challenger_processor;
pub mod error;
pub mod task_manager;

/// The tuple for NewTaskCreated: (u32, Input)
pub type NewTaskEventTuple<Input> = (
    <Input as SolValue>::SolType,
    <u32 as SolValue>::SolType,
    <Bytes as SolValue>::SolType,
    <u32 as SolValue>::SolType,
);

/// The tuple for TaskResponded: ((u32, Output), Metadata)
pub type TaskResponseEventTuple<Output> = (
    (<u32 as SolValue>::SolType, <Output as SolValue>::SolType),
    <TaskResponseMetadataSol as SolValue>::SolType,
);

/// The tuple for RespondToTaskCalldata: (Input, (TaskIndex, Output), NonSignerStakesAndSignature)
pub type RespondToTaskCalldata<Input, Output> = (
    (
        <Input as SolValue>::SolType,
        <u32 as SolValue>::SolType,
        <Bytes as SolValue>::SolType,
        <u32 as SolValue>::SolType,
    ),
    (<u32 as SolValue>::SolType, <Output as SolValue>::SolType),
    <NonSignerStakesAndSignature as SolValue>::SolType,
);

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
    /// * `rpc_url` - The rpc url
    /// * `ws_url` - The websocket url
    /// * `task_processor` - The task processor
    ///
    /// # Returns
    ///
    /// * `Self` - The challenger
    pub fn new(rpc_url: String, ws_url: String, task_processor: TP) -> Self {
        Self {
            rpc_url,
            ws_url,
            task_processor,
        }
    }

    /// Start the service and start listening for new tasks and task responses events
    /// It also checks if the response is correct, if not it raises a challenge.
    ///
    /// # Arguments
    ///
    /// # Returns
    pub async fn start_challenger(&mut self) -> Result<(), ChallengerError> {
        info!("challenger crate launched");

        let ws_provider = get_ws_provider(&self.ws_url).await?;

        // Subscribe to NewTaskEvent
        let task_filter = Filter::new().event_signature(TP::NewTaskEvent::SIGNATURE_HASH);
        let mut task_stream = ws_provider
            .subscribe_logs(&task_filter)
            .await?
            .into_stream();

        // Subscribe to TaskResponseEvent
        let responded_filter = Filter::new().event_signature(TP::TaskResponseEvent::SIGNATURE_HASH);
        let mut responded_stream = ws_provider
            .subscribe_logs(&responded_filter)
            .await?
            .into_stream();

        loop {
            tokio::select! {
                Some(log) = task_stream.next() => {
                    let (task_index, task) = self.decode_task_creation_event(log)?;
                    self.task_processor.handle_task_creation(task_index, task).await?;
                },
                Some(log) = responded_stream.next() => {
                    let (task_index, task_response, task_response_metadata, non_signing_operator_pub_keys) =
                        self.decode_task_response_event(log).await?;
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

    fn decode_task_creation_event(
        &self,
        log: Log,
    ) -> Result<(u32, Task<TP::Input>), ChallengerError> {
        // event NewTaskCreated(uint32 indexed taskIndex, Task task);
        // Since taskIndex is indexed type, it is present in the topics array
        // The first element of the topic is the event hash signature, the second is the taskIndex
        let bytes = log
            .topics()
            .get(1)
            .ok_or(ChallengerError::TaskIndexMissingInTopics)?
            .0;

        // u32 values are stored in the last 4 bytes of a 32 bytes array (left-padded).
        let task_index_bytes: [u8; 4] = bytes[28..32]
            .try_into()
            .map_err(|_| ChallengerError::InvalidTaskIndexConversion)?;
        let task_index = u32::from_be_bytes(task_index_bytes);

        // Skip the first 32 bytes of the ABI-encoded data (the dynamic offset pointer)
        let data = log
            .inner
            .data
            .data
            .0
            .get(32..)
            .ok_or(ChallengerError::EmptyDecodedData)?;

        // Decode the tuple of the form: Task<TM::Input>
        let decoded_task = decode_params::<NewTaskEventTuple<TP::Input>>(data, false)?;

        let task = Task::<TP::Input> {
            input: decoded_task.0.into(),
            task_created_block: decoded_task.1,
            quorum_numbers: decoded_task.2,
            quorum_threshold_percentage: decoded_task.3,
        };

        Ok((task_index, task))
    }

    async fn decode_task_response_event(
        &self,
        log: Log,
    ) -> Result<
        (
            u32,
            TaskResponse<TP::Output>,
            TaskResponseMetadataSol,
            Vec<G1Point>,
        ),
        ChallengerError,
    > {
        let data = log.inner.data.data.0.clone();

        // Decode the tuple of the form: (TaskResponse<TM::Output>, TaskResponseMetadata)
        let decoded_task_response =
            decode_params::<TaskResponseEventTuple<TP::Output>>(&data, false)?;

        let task_index = decoded_task_response.0 .0;
        let task_response = decoded_task_response.0 .1;
        let task_response_metadata = decoded_task_response.1;

        let task_response = TaskResponse::<TP::Output> {
            task_index,
            response: task_response.into(),
        };

        let non_signing_operator_pub_keys = self.get_non_signing_operator_pub_keys(log).await?;

        Ok((
            task_index,
            task_response,
            task_response_metadata,
            non_signing_operator_pub_keys,
        ))
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

/// Decode generic type
///
/// # Arguments
///
/// * `data` - The data to decode
/// * `validate` - Whether to validate the data
///
/// # Returns
///
/// * `Result<T::RustType, ChallengerError>` - The decoded data
pub fn decode_params<T>(data: &[u8], validate: bool) -> Result<T::RustType, ChallengerError>
where
    T: SolType,
    for<'de> <T as SolType>::Token<'de>: TokenSeq<'de>,
{
    Ok(T::abi_decode_params(data, validate)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{hex::decode, primitives::U256, sol};

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
