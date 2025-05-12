use crate::task::Task;
use alloy::{dyn_abi::SolType, primitives::Bytes, rpc::types::Log, sol_types::SolValue};
use thiserror::Error;

/// Error returned by the [`decode_new_task`] function.
#[derive(Debug, Error)]
pub enum DecodeNewTaskError {
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

/// Decode the log of the NewTaskCreated event to get the task index and the task
///
/// # Arguments
///
/// * `log` - The log of the NewTaskCreated event
///
/// # Returns
///
/// * `Result<(u32, Task<Input>), DecodeNewTaskError>` - The task index and the task
pub fn decode_new_task<Input>(log: &Log) -> Result<(u32, Task<Input>), DecodeNewTaskError>
where
    Input: SolValue + From<<<Input as SolValue>::SolType as SolType>::RustType>,
{
    // event NewTaskCreated(uint32 indexed taskIndex, Task task);
    // Since taskIndex is indexed type, it is present in the topics array
    // The first element of the topic is the event hash signature, the second is the taskIndex
    let bytes: [u8; 32] = log
        .topics()
        .get(1)
        .ok_or(DecodeNewTaskError::TopicListTooShort)?
        .0;

    // u32 values are stored in the last 4 bytes of a 32 bytes array (left-padded).
    let task_index_bytes: [u8; 4] = bytes[28..32]
        .try_into()
        .map_err(|_| DecodeNewTaskError::TaskIndexDecodingFailed)?;
    let task_index = u32::from_be_bytes(task_index_bytes);

    // Skip the first 32 bytes of the ABI-encoded data (the dynamic offset pointer)
    // so we can decode the actual tuple payload that follows.
    let data = log
        .inner
        .data
        .data
        .0
        .get(32..)
        .ok_or(DecodeNewTaskError::TaskDataTooShort)?;

    let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
        <(
            <Input as SolValue>::SolType,
            <u32 as SolValue>::SolType,
            <Bytes as SolValue>::SolType,
            <u32 as SolValue>::SolType,
        )>::abi_decode_params(data, false)?;

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
