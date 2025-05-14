//! Task manager

use alloy::primitives::B256;
use eigen_services_blsaggregation::bls_agg::TaskMetadata;
use eigen_task_manager::task::Task;
use eigen_task_manager::task_response::TaskResponse;
use eigen_task_manager::TaskManager;
use eigen_types::avs::TaskResponseDigest;
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, fmt::Debug};
use tokio::sync::Mutex;
use tracing::info;

use super::{TaskProcessor, TaskProcessorError};

type TaskResponsesMap<O> = HashMap<u32, HashMap<TaskResponseDigest, TaskResponse<O>>>;

/// Indexing task processor
#[derive(Debug, Clone)]
pub struct IndexingTaskProcessor<TM>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
{
    /// Hashmap to store the created tasks
    tasks: Arc<Mutex<HashMap<u32, Task<TM::Input>>>>,
    /// Hashmap to store the task responses
    task_responses: Arc<Mutex<TaskResponsesMap<TM::Output>>>,
    /// Avs writer
    task_manager: TM,
    /// Time that the task will be available for processing
    task_timeout: Duration,
    /// Time that a completed task will be available for receiving aggregated responses
    task_window_duration: Duration,
}

impl<TM> IndexingTaskProcessor<TM>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
{
    /// Create a new task processor
    ///
    /// # Arguments
    ///
    /// * `task_manager` - The task manager
    ///
    /// # Returns
    ///
    /// A new task processor
    pub fn new(task_manager: TM, task_timeout: Duration, task_window_duration: Duration) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::default())),
            task_responses: Arc::new(Mutex::new(HashMap::default())),
            task_manager,
            task_timeout,
            task_window_duration,
        }
    }
}

impl<TM> TaskProcessor for IndexingTaskProcessor<TM>
where
    TM: TaskManager + Debug + Send + Sync + 'static + Clone,
{
    type Output = TM::Output;

    type Input = TM::Input;

    const NEW_TASK_EVENT_SELECTOR: B256 = TM::NEW_TASK_EVENT_SELECTOR;

    async fn process_new_task(
        &mut self,
        task_index: u32,
        task: Task<TM::Input>,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        self.tasks.lock().await.insert(task_index, task.clone());

        let quorum_numbers: Vec<u8> = task.quorum_numbers.into();
        let quorum_threshold_percentages =
            std::iter::repeat_n(task.quorum_threshold_percentage as u8, quorum_numbers.len())
                .collect();
        Ok(TaskMetadata::new(
            task_index,
            task.task_created_block.into(),
            quorum_numbers,
            quorum_threshold_percentages,
            self.task_timeout,
        )
        .with_window_duration(self.task_window_duration))
    }

    async fn process_task_response(
        &mut self,
        response: TaskResponse<TM::Output>,
    ) -> Result<B256, TaskProcessorError> {
        if !self.tasks.lock().await.contains_key(&response.task_index) {
            info!("Task not found for task index: {}", response.task_index);
            return Err(TaskProcessorError::TaskNotFound);
        }

        let digest = alloy::primitives::keccak256(response.encode());

        self.task_responses
            .lock()
            .await
            .entry(response.task_index)
            .or_default()
            .entry(digest)
            .or_insert(response);

        Ok(digest)
    }

    async fn process_aggregated_response(
        &self,
        task_index: u32,
        task_response_digest: B256,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> Result<(), TaskProcessorError> {
        info!(
            "Aggregated response received for task {}: {:?}",
            task_index, task_response_digest
        );

        let (task, task_response) = {
            let tasks_lock = self.tasks.lock().await;
            let task = tasks_lock
                .get(&task_index)
                .ok_or(TaskProcessorError::TaskNotFound)?
                .clone();

            let responses_lock = self.task_responses.lock().await;
            let task_response = responses_lock
                .get(&task_index)
                .and_then(|map| map.get(&task_response_digest))
                .ok_or(TaskProcessorError::TaskResponseNotFound)?
                .clone();

            (task, task_response)
        };

        self.task_manager
            .respond_to_task(task, task_response, non_signer_stakes_and_signature)
            .await
            .map_err(TaskProcessorError::TaskManagerError)
            .inspect(|_| info!("Aggregated response sent to contract"))?;

        self.tasks.lock().await.remove(&task_index);
        self.task_responses.lock().await.remove(&task_index);

        Ok(())
    }
}
