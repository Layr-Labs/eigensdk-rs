//! This is a simple task generator that can be used to create tasks for the operators.
//! For testing purposes.

use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
};
use eigen_task_processor::task_manager::TaskManagerContract;
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use error::TaskSpammerError;
use std::time::Duration;
use tokio::time::sleep;

/// Task spammer errors
pub mod error;

/// Task spammer builder
#[derive(Debug)]
pub struct TaskSpammerBuilder<I, TM, T, P, N, Input> {
    iter: Option<I>,
    interval: Duration,
    quorum_threshold: Option<QuorumThresholdPercentage>,
    quorums: Option<Vec<QuorumNum>>,
    task_manager: TM,
    _phantom: std::marker::PhantomData<(T, P, N, Input)>,
}

impl<I, TM, T, P, N> TaskSpammerBuilder<I, TM, T, P, N, TM::Input>
where
    TM: TaskManagerContract<T, P, N>,
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    /// Create a new task spammer builder
    ///
    /// # Arguments
    ///
    /// * `task_manager` - Trait that wraps the task manager contract.
    ///
    /// # Returns
    ///
    /// A new task spammer builder.
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

    /// Set the iterator for the task creation
    /// This will be used to create N tasks
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator for the task creation
    ///
    /// # Returns
    ///
    /// * `TaskSpammerBuilder` - The builder for the task spammer
    pub fn with_iter(self, iter: I) -> TaskSpammerBuilder<I, TM, T, P, N, TM::Input>
    where
        I: Iterator + Send + 'static,
        I::Item: Send,
    {
        TaskSpammerBuilder {
            iter: Some(iter),
            interval: self.interval,
            quorum_threshold: self.quorum_threshold,
            quorums: self.quorums,
            task_manager: self.task_manager,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the interval for the task creation
    ///
    /// # Arguments
    ///
    /// * `interval` - The interval for the task creation
    ///
    /// # Returns
    ///
    /// * `TaskSpammerBuilder` - The builder for the task spammer
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set the quorum where the task will be created and the threshold, indicating when
    /// a task is considered completed
    ///
    /// # Arguments
    ///
    /// * `quorum_threshold` - The quorum threshold for the task creation
    /// * `quorums` - The quorums for the task creation
    ///
    /// # Returns
    ///
    /// * `TaskSpammerBuilder` - The builder for the task spammer
    pub fn with_quorum(
        mut self,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Self {
        self.quorum_threshold = Some(quorum_threshold);
        self.quorums = Some(quorums);
        self
    }

    /// Build the task spammer
    ///
    /// # Returns
    ///
    /// * `TaskSpammer` - The task spammer to be run
    pub fn build(self) -> Result<TaskSpammer<I, TM, T, P, N, TM::Input>, TaskSpammerError> {
        Ok(TaskSpammer {
            iter: self.iter.ok_or(TaskSpammerError::IteratorNotSet)?,
            interval: self.interval,
            quorum_threshold: self
                .quorum_threshold
                .ok_or(TaskSpammerError::QuorumThresholdNotSet)?,
            quorums: self.quorums.ok_or(TaskSpammerError::QuorumNotSet)?,
            task_manager: self.task_manager,
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Task spammer struct
#[derive(Debug)]
pub struct TaskSpammer<I, TM, T, P, N, Input> {
    iter: I,
    interval: Duration,
    quorum_threshold: QuorumThresholdPercentage,
    quorums: Vec<QuorumNum>,
    task_manager: TM,
    _phantom: std::marker::PhantomData<(T, P, N, Input)>,
}

impl<I, TM, T, P, N> TaskSpammer<I, TM, T, P, N, TM::Input>
where
    TM: TaskManagerContract<T, P, N> + Send + Sync,
    T: Transport + Clone + Send + Sync,
    P: Provider<T, N>,
    N: Network,
{
    /// Run the task spammer
    /// This will create N tasks, where N is the number of items in the iterator
    /// We use the elements of the iterator as input for the task manager contract
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskSpammerError>` - The result of the task spammer
    pub async fn run(self) -> Result<(), TaskSpammerError>
    where
        I: Iterator<Item = TM::Input> + Send,
        I::Item: Clone + Send + 'static,
    {
        for input in self.iter {
            self.task_manager
                .create_new_task(input, self.quorum_threshold, self.quorums.clone())
                .await?;
            sleep(self.interval).await;
        }
        Ok(())
    }
}
