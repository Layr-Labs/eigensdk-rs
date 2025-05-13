use crate::{challenger::ChallengerTaskProcessor, error::ChallengerError};
use alloy::dyn_abi::SolType;
use alloy::primitives::B256;
use alloy::sol_types::SolValue;

use eigen_task_processor::task_manager::{TaskManager, TaskManagerError};
use eigen_task_processor::task_response_metadata_sol::TaskResponseMetadataSol;
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;
use futures_util::future::BoxFuture;
use std::collections::HashMap;
use std::future::Future;
use tracing::{error, info};

#[derive(Debug)]
pub struct IndexingChallengerProcessor<TM, F, Fut>
where
    TM: TaskManager + Send + Sync + 'static + Clone,
    F: Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<bool, TaskManagerError>>,
{
    task_manager: TM,
    tasks: HashMap<u32, Task<TM::Input>>,
    is_response_correct: F,
}

impl<TM, F, Fut> ChallengerTaskProcessor for IndexingChallengerProcessor<TM, F, Fut>
where
    TM: TaskManager + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    F: Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<bool, TaskManagerError>> + Send,
{
    type Input = TM::Input;

    type Output = TM::Output;

    /// New task event
    const NEW_TASK_EVENT_SELECTOR: B256 = TM::NEW_TASK_EVENT_SELECTOR;

    /// Task responded event
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TM::TASK_RESPONDED_EVENT_SELECTOR;

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
        let Some(task) = self.tasks.get(&task_index) else {
            info!("Task {task_index} not found");
            return Ok(());
        };

        let is_correct = (self.is_response_correct)(task.clone(), task_response.clone()).await?;

        // If the response is correct, we don't need to raise a challenge
        if is_correct {
            info!("Task {task_index} is correct");
            return Ok(());
        }

        // If the response is incorrect, we need to raise a challenge
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
            .inspect_err(|e| error!("Challenge failed for task {task_index}: {e}"))
        });

        Ok(())
    }
}

impl<TM, F, Fut> IndexingChallengerProcessor<TM, F, Fut>
where
    TM: TaskManager + Send + Sync + 'static + Clone,
    TM::Input: From<<<TM::Input as SolValue>::SolType as SolType>::RustType>,
    TM::Output: From<<<TM::Output as SolValue>::SolType as SolType>::RustType>,
    F: Fn(Task<TM::Input>, TaskResponse<TM::Output>) -> Fut + Send + Sync,
    Fut: Future<Output = Result<bool, TaskManagerError>>,
{
    pub fn new(task_manager: TM, is_response_correct: F) -> Self {
        Self {
            task_manager,
            tasks: HashMap::new(),
            is_response_correct,
        }
    }
}

pub fn verifier_from_compute_function<Input, Output, OpFn, OpFut>(
    op_fn: OpFn,
) -> impl Fn(Task<Input>, TaskResponse<Output>) -> BoxFuture<'static, Result<bool, TaskManagerError>>
       + Clone
       + Send
       + Sync
where
    OpFn: Fn(u32, Input) -> OpFut + Clone + Send + Sync + 'static,
    OpFut: Future<Output = Result<Output, TaskManagerError>> + Send + 'static,
    Output: SolValue + Clone + PartialEq + Send + 'static,
    Input: Send + 'static,
{
    move |task, task_response| {
        let fut = op_fn(task_response.task_index, task.input);

        // Devolvemos otro Future boxed
        Box::pin(async move {
            let computed = fut.await?;
            Ok(computed == task_response.response)
        })
    }
}
