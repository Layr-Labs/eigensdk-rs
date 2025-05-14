use crate::{
    task::Task, task_response::TaskResponse, task_response_metadata_sol::TaskResponseMetadataSol,
};
use alloy::{
    dyn_abi::{abi::TokenSeq, SolType},
    primitives::Bytes,
    rpc::types::Log,
    sol_types::SolValue,
};
use eigen_crypto_bls::OperatorId;
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use eigen_utils::slashing::middleware::registrycoordinator::BN254::G1Point;
use thiserror::Error;

/// Error returned by the [`decode_new_task`] function.
#[derive(Debug, Error)]
pub enum AbiDecodeError {
    /// The topic list has less than 2 elements
    #[error("Topic list has less than 2 elements")]
    TopicListTooShort,
    /// The task index could not be decoded
    #[error("Task index could not be decoded")]
    TaskIndexDecodingFailed,
    /// Task data is too short
    #[error("Task data is too short")]
    TaskDataTooShort,
    /// Task data could not be decoded
    #[error("Task data could not be decoded")]
    LogDecodeFailed(#[from] alloy::sol_types::Error),
}

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

/// The tuple for SignedTaskResponse: (TaskResponse<Output>, G1Point, OperatorId)
pub type SignedTaskResponseTuple<Output> = (
    (<u32 as SolValue>::SolType, <Output as SolValue>::SolType),
    <G1Point as SolValue>::SolType,
    <OperatorId as SolValue>::SolType,
);

/// Decode the log of the NewTaskCreated event to get the task index and the task
///
/// # Arguments
///
/// * `log` - The log of the NewTaskCreated event
///
/// # Returns
///
/// * `Result<(u32, Task<Input>), DecodeNewTaskError>` - The task index and the task
pub fn decode_new_task<Input>(log: &Log) -> Result<(u32, Task<Input>), AbiDecodeError>
where
    Input: SolValue + From<<<Input as SolValue>::SolType as SolType>::RustType>,
{
    // event NewTaskCreated(uint32 indexed taskIndex, Task task);
    // Since taskIndex is indexed type, it is present in the topics array
    // The first element of the topic is the event hash signature, the second is the taskIndex
    let bytes: [u8; 32] = log
        .topics()
        .get(1)
        .ok_or(AbiDecodeError::TopicListTooShort)?
        .0;

    // u32 values are stored in the last 4 bytes of a 32 bytes array (left-padded).
    let task_index_bytes: [u8; 4] = bytes[28..32]
        .try_into()
        .map_err(|_| AbiDecodeError::TaskIndexDecodingFailed)?;
    let task_index = u32::from_be_bytes(task_index_bytes);

    // Skip the first 32 bytes of the ABI-encoded data (the dynamic offset pointer)
    // so we can decode the actual tuple payload that follows.
    let data = log
        .inner
        .data
        .data
        .0
        .get(32..)
        .ok_or(AbiDecodeError::TaskDataTooShort)?;

    let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
        decode_params::<NewTaskEventTuple<Input>>(data, false)?;

    Ok((
        task_index,
        Task::<Input> {
            input: input.into(),
            task_created_block,
            quorum_numbers,
            quorum_threshold_percentage,
        },
    ))
}

/// Decode the log of the TaskResponded event to get the task index, task response and task response metadata
///
/// # Arguments
///
/// * `log` - The log of the TaskResponded event
///
/// # Returns
///
/// * `Result<(u32, TaskResponse<Output>, TaskResponseMetadataSol), AbiDecodeError>` - The task index, task response and task response metadata
pub async fn decode_task_response_event<Output>(
    log: &Log,
) -> Result<(u32, TaskResponse<Output>, TaskResponseMetadataSol), AbiDecodeError>
where
    Output: SolValue + From<<<Output as SolValue>::SolType as SolType>::RustType> + Clone,
{
    let data = log.inner.data.data.0.clone();

    // Decode the tuple of the form: (TaskResponse<TM::Output>, TaskResponseMetadata)
    let decoded_task_response = decode_params::<TaskResponseEventTuple<Output>>(&data, false)?;

    let task_index = decoded_task_response.0 .0;
    let task_response = decoded_task_response.0 .1;
    let task_response_metadata = decoded_task_response.1;

    let task_response = TaskResponse::<Output> {
        task_index,
        response: task_response.into(),
    };

    Ok((task_index, task_response, task_response_metadata))
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
/// * `Result<T::RustType, AbiDecodeError>` - The decoded data
pub fn decode_params<T>(data: &[u8], validate: bool) -> Result<T::RustType, AbiDecodeError>
where
    T: SolType,
    for<'de> <T as SolType>::Token<'de>: TokenSeq<'de>,
{
    Ok(T::abi_decode_params(data, validate)?)
}
