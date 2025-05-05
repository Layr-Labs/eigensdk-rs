use crate::{
    challenger::ChallengerTaskProcessor, error::ChallengerError, task_manager::TaskManagerContract,
};
use alloy::consensus::Transaction;
use alloy::dyn_abi::SolType;
use alloy::providers::Provider;
use alloy::sol;
use alloy::{
    contract::private::{Provider as PrivateProvider, Transport},
    network::Network,
    primitives::Bytes,
    rpc::types::Log,
    sol_types::SolValue,
};

use eigen_common::get_provider;
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;
use std::collections::HashMap; // Check which type to use

sol! {
    #[derive(Debug)]
    struct TaskResponseMetadataSol {
        uint32 taskResponsedBlock;
        bytes32 hashOfNonSigners;
    }
}

#[derive(Debug)]
pub struct IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static,
    T: Transport + Clone + Send + Sync,
    P: PrivateProvider<T, N>,
    N: Network,
{
    task_manager: TM,
    tasks: HashMap<u32, Task<TM::Input>>,
    rpc_url: String,
}

impl<TM, T, P, N> ChallengerTaskProcessor for IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    T: Transport + Clone + Send + Sync,
    P: PrivateProvider<T, N>,
    N: Network,
{
    type NewTaskEvent = TM::NewTaskEvent;

    type TaskResponseEvent = TM::TaskRespondedEvent;

    type Input = TM::Input;

    type Output = TM::Output;

    async fn handle_task_creation(&mut self, log: Log) -> Result<(), ChallengerError> {
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
        // so we can decode the actual tuple payload that follows.
        let data = log
            .inner
            .data
            .data
            .0
            .get(32..)
            .ok_or(ChallengerError::EmptyDecodedData)?;

        // Decode Task<TM::Input>
        let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
            <(
                <TM::Input as SolValue>::SolType,
                <u32 as SolValue>::SolType,
                <Bytes as SolValue>::SolType,
                <u32 as SolValue>::SolType,
            )>::abi_decode_params(data, false)?;

        let task = Task::<TM::Input> {
            input: input.into(),
            task_created_block,
            quorum_numbers,
            quorum_threshold_percentage,
        };
        self.tasks.insert(task_index, task);

        Ok(())
    }

    async fn handle_task_response(
        &mut self,
        log: Log,
        is_response_correct: impl Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Result<bool, ChallengerError>
            + Send,
    ) -> Result<(), ChallengerError> {
        let data = log.inner.data.data.0.clone();

        // Decode a tuple of the form: (TaskResponse<TM::Output>, TaskResponseMetadata)
        let ((task_index, response), task_response_metadata) =
            <(
                (
                    <u32 as SolValue>::SolType,
                    <TM::Output as SolValue>::SolType,
                ),
                <TaskResponseMetadataSol as SolValue>::SolType,
            )>::abi_decode_params(&data, false)?;

        let task_response = TaskResponse::<TM::Output> {
            task_index,
            response: response.into(),
        };

        let non_signing_operator_pub_keys = self.get_non_signing_operator_pub_keys(log).await?;

        if let Some(task) = self.tasks.get(&task_index) {
            if is_response_correct(task.clone(), task_response.clone())? {
                // TODO: Call the challenge in another thread?
                self.task_manager
                    .raise_challenge(
                        task.clone(),
                        task_response,
                        task_response_metadata,
                        non_signing_operator_pub_keys,
                    )
                    .await?;
            }
        };

        Ok(())
    }
}

impl<TM, T, P, N> IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    T: Transport + Clone + Send + Sync,
    P: PrivateProvider<T, N>,
    N: Network,
{
    pub fn new(rpc_url: String, task_manager: TM) -> Self {
        Self {
            rpc_url,
            task_manager,
            tasks: HashMap::new(),
        }
    }

    pub async fn get_non_signing_operator_pub_keys(
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

        let calldata = tx.inner.input();

        // Decode tuple of the form: Task<TM::Input>, TaskResponse<TM::Output>, NonSignerStakesAndSignature)
        let (_, _, non_signer_stakes_and_signature) =
            <(
                (
                    <TM::Input as SolValue>::SolType,
                    <u32 as SolValue>::SolType,
                    <Bytes as SolValue>::SolType,
                    <u32 as SolValue>::SolType,
                ),
                (
                    <u32 as SolValue>::SolType,
                    <TM::Output as SolValue>::SolType,
                ),
                <NonSignerStakesAndSignature as SolValue>::SolType,
            )>::abi_decode_params(calldata, false)?;

        Ok(non_signer_stakes_and_signature
            .nonSignerPubkeys
            .into_iter()
            .map(|pk| G1Point { X: pk.X, Y: pk.Y })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use alloy::{hex::decode, primitives::U256};

    use super::*;

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

        let raw_bytes: Vec<u8> = decode(raw_hex).expect("hex inválido");

        let data = raw_bytes.get(32..).unwrap().to_vec();

        let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
            <(
                <U256 as SolValue>::SolType,
                <u32 as SolValue>::SolType,
                <Bytes as SolValue>::SolType,
                <u32 as SolValue>::SolType,
            )>::abi_decode_params(&data, true)
            .unwrap();

        assert_eq!(input, U256::ONE);
        assert_eq!(task_created_block, 226);
        assert_eq!(quorum_numbers, Bytes::from_static(&[0]));
        assert_eq!(quorum_threshold_percentage, 40);
    }

    #[tokio::test]
    async fn test_decode_task_response_event() {
        let raw_hex = "\
        0000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000000000000000000000000001\
        00000000000000000000000000000000000000000000000000000000000000e3\
        b569c9609dde655467765df81ebf2a34e4b9f40806475961b6676a2ec6115e61";

        let raw_bytes: Vec<u8> = decode(raw_hex).unwrap();

        let data = raw_bytes.as_slice();

        let ((task_index, response), metadata) = <(
            (<u32 as SolValue>::SolType, <U256 as SolValue>::SolType),
            <TaskResponseMetadataSol as SolValue>::SolType,
        )>::abi_decode_params(data, false)
        .unwrap();

        assert_eq!(task_index, 0);
        assert_eq!(response, U256::ONE);
        assert_eq!(metadata.taskResponsedBlock, 227);
        // assert_eq!(metadata.hashOfNonSigners, Bytes::from_static(&[0]));
    }
}
