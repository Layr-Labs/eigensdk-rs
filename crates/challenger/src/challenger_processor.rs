use crate::{
    challenger::ChallengerTaskProcessor, error::ChallengerError, task_manager::TaskManagerContract,
};
use alloy::dyn_abi::SolType;
use alloy::sol;
use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
    sol_types::SolValue,
};

use eigen_task_processor::{task::Task, task_response::TaskResponse};
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
pub struct IndexingChallengerProcessor<TM, T, P, N, F>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
    F: Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Result<bool, ChallengerError> + Send + Sync,
{
    task_manager: TM,
    tasks: HashMap<u32, Task<TM::Input>>,
    is_response_correct: F,
}

impl<TM, T, P, N, F> ChallengerTaskProcessor for IndexingChallengerProcessor<TM, T, P, N, F>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
    F: Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Result<bool, ChallengerError> + Send + Sync,
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
        &self,
        task_index: u32,
        task_response: TaskResponse<TM::Output>,
        task_response_metadata: TaskResponseMetadataSol,
        non_signing_operator_pub_keys: Vec<G1Point>,
    ) -> Result<(), ChallengerError> {
        if let Some(task) = self.tasks.get(&task_index).filter(|&t| {
            !((self.is_response_correct)(t.clone(), task_response.clone()).unwrap_or(false))
        }) {
            let tm = self.task_manager.clone();
            let task = task.clone();

            tokio::spawn(async move {
                tm.raise_challenge(
                    task,
                    task_response,
                    task_response_metadata,
                    non_signing_operator_pub_keys,
                )
                .await
                .inspect_err(|e| error!("Challenge failed for task {}: {}", task_index, e))
            });
        }

        Ok(())
    }
}

impl<TM, T, P, N, F> IndexingChallengerProcessor<TM, T, P, N, F>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
    F: Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Result<bool, ChallengerError> + Send + Sync,
{
    pub fn new(task_manager: TM, is_response_correct: F) -> Self {
        Self {
            task_manager,
            tasks: HashMap::new(),
            is_response_correct,
        }
    }
}
