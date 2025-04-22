//! This is a simple task generator that can be used to create tasks for the operators.
//! For testing purposes.

use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use error::TaskGeneratorError;
use std::time::Duration;
use task_manager::TaskManagerContract;
use tokio::time::sleep;

/// Task generator errors
pub mod error;
/// Task manager contract trait
pub mod task_manager;

/// Task generator builder
#[derive(Debug)]
pub struct TaskGeneratorBuilder<I, TM, T, P, N, Input> {
    iter: Option<I>,
    interval: Duration,
    quorum_threshold: Option<QuorumThresholdPercentage>,
    quorums: Option<Vec<QuorumNum>>,
    task_manager: TM,
    _phantom: std::marker::PhantomData<(T, P, N, Input)>,
}

impl<I, TM, T, P, N, Input> TaskGeneratorBuilder<I, TM, T, P, N, Input>
where
    TM: TaskManagerContract<Input, T, P, N>,
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::network::Network,
{
    /// Create a new task generator builder
    pub fn new(task_manager: TM) -> Self {
        Self {
            iter: None,
            interval: Duration::ZERO,
            quorum_threshold: None,
            quorums: None,
            task_manager,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the iterator
    pub fn with_iter(self, iter: I) -> TaskGeneratorBuilder<I, TM, T, P, N, Input>
    where
        I: Iterator + Send + 'static,
        I::Item: Send,
    {
        TaskGeneratorBuilder {
            iter: Some(iter),
            interval: self.interval,
            quorum_threshold: self.quorum_threshold,
            quorums: self.quorums,
            task_manager: self.task_manager,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set the quorum and threshold
    pub fn with_quorum(
        mut self,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Self {
        self.quorum_threshold = Some(quorum_threshold);
        self.quorums = Some(quorums);
        self
    }

    /// Build the task generator
    pub fn build(self) -> Result<TaskGenerator<I, TM, T, P, N, Input>, TaskGeneratorError>
    where
        TM: TaskManagerContract<Input, T, P, N>,
        T: alloy::contract::private::Transport + ::core::clone::Clone,
        P: alloy::contract::private::Provider<T, N>,
        N: alloy::network::Network,
    {
        Ok(TaskGenerator {
            iter: self.iter.ok_or(TaskGeneratorError::IteratorNotSet)?,
            interval: self.interval,
            quorum_threshold: self
                .quorum_threshold
                .ok_or(TaskGeneratorError::QuorumThresholdNotSet)?,
            quorums: self.quorums.ok_or(TaskGeneratorError::QuorumNotSet)?,
            task_manager: self.task_manager,
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Task generator struct
#[derive(Debug)]
pub struct TaskGenerator<I, TM, T, P, N, Input> {
    iter: I,
    interval: Duration,
    quorum_threshold: QuorumThresholdPercentage,
    quorums: Vec<QuorumNum>,
    task_manager: TM,
    _phantom: std::marker::PhantomData<(T, P, N, Input)>,
}

impl<I, TM, T, P, N, Input> TaskGenerator<I, TM, T, P, N, Input>
where
    TM: TaskManagerContract<Input, T, P, N> + Send + Sync,
    T: alloy::contract::private::Transport + Clone + Send + Sync,
    P: alloy::contract::private::Provider<T, N> + Send + Sync,
    N: alloy::providers::Network + Send + Sync,
{
    /// Run the task generator
    pub async fn run(self) -> Result<(), TaskGeneratorError>
    where
        I: Iterator<Item = Input> + Send,
        I::Item: Clone + Send + 'static,
    {
        for input in self.iter {
            self.task_manager
                .create_new_task(input, self.quorum_threshold, self.quorums.clone())
                .send()
                .await?
                .get_receipt()
                .await?;
            sleep(self.interval).await;
        }
        Ok(())
    }
}
