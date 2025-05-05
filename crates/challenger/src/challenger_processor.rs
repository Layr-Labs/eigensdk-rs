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

        let ((task_index, response), metadata) = <(
            (<u32 as SolValue>::SolType, <U256 as SolValue>::SolType),
            <TaskResponseMetadataSol as SolValue>::SolType,
        )>::abi_decode_params(data, false)
        .unwrap();

        assert_eq!(task_index, 0);
        assert_eq!(response, U256::ONE);
        assert_eq!(metadata.taskResponsedBlock, 227);
    }

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

        let (
            (input, task_created_block, _, _),
            (task_index_response, response),
            non_signer_stakes_and_signature,
        ) = <(
            (
                <U256 as SolValue>::SolType,
                <u32 as SolValue>::SolType,
                <Bytes as SolValue>::SolType,
                <u32 as SolValue>::SolType,
            ),
            (<u32 as SolValue>::SolType, <U256 as SolValue>::SolType),
            <NonSignerStakesAndSignature as SolValue>::SolType,
        )>::abi_decode_params(calldata, false)
        .unwrap();

        assert_eq!(input, U256::ONE);
        assert_eq!(task_created_block, 226);

        assert_eq!(task_index_response, 0);
        assert_eq!(response, U256::ONE);
        assert_eq!(non_signer_stakes_and_signature.totalStakeIndices, vec![2]);

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
            non_signer_stakes_and_signature
                .nonSignerPubkeys
                .first()
                .unwrap()
                .X,
            expected_pub_key.X
        );
        assert_eq!(
            non_signer_stakes_and_signature
                .nonSignerPubkeys
                .first()
                .unwrap()
                .Y,
            expected_pub_key.Y
        );
    }
}
