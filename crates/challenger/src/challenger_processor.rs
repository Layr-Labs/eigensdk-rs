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
use std::collections::HashMap;
use tracing::error;

// Metadata of the task response
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
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
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
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
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

    async fn handle_task_creation(
        &mut self,
        task_index: u32,
        task: Task<TM::Input>,
    ) -> Result<(), ChallengerError> {
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

        if let Some(task) = self
            .tasks
            .get(&task_index)
            .cloned()
            .filter(|t| !is_response_correct(t.clone(), task_response.clone()).unwrap_or(false))
        {
            let tm = self.task_manager.clone();

            tokio::spawn(async move {
                if let Err(e) = tm
                    .raise_challenge(
                        task,
                        task_response,
                        task_response_metadata,
                        non_signing_operator_pub_keys,
                    )
                    .await
                {
                    error!("raise_challenge failed: {:?}", e);
                }
            });
        }

        Ok(())
    }
}

impl<TM, T, P, N> IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
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

        // The first 4 bytes are the selector, so we skip them
        let calldata = tx
            .inner
            .input()
            .get(4..)
            .ok_or(ChallengerError::InvalidCalldata)?;

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
