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
pub struct IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    task_manager: TM,
    tasks: HashMap<u32, Task<TM::Input>>,
}

impl<TM, T, P, N> ChallengerTaskProcessor for IndexingChallengerProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
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
        task_index: u32,
        task_response: TaskResponse<TM::Output>,
        task_response_metadata: TaskResponseMetadataSol,
        non_signing_operator_pub_keys: Vec<G1Point>,
        is_response_correct: impl Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Result<bool, ChallengerError>
            + Send,
    ) -> Result<(), ChallengerError> {
        if let Some(task) = self
            .tasks
            .get(&task_index)
            .cloned()
            .filter(|t| !is_response_correct(t.clone(), task_response.clone()).unwrap_or(false))
        {
            let tm = self.task_manager.clone();

            tokio::spawn(async move {
                tm.raise_challenge(
                    task,
                    task_response,
                    task_response_metadata,
                    non_signing_operator_pub_keys,
                )
                .await
                .inspect_err(|e| error!("raise_challenge failed: {:?}", e))
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
    P: Provider<T, N>,
    N: Network,
{
    pub fn new(task_manager: TM) -> Self {
        Self {
            task_manager,
            tasks: HashMap::new(),
        }
    }
}
