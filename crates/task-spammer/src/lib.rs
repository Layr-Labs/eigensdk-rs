//! This is a simple task generator that can be used to create tasks for the operators.
//! For testing purposes.

<<<<<<< HEAD
use eigen_task_manager::TaskManager;
=======
use alloy::{contract::private::Provider, network::Network};
>>>>>>> v2-dev-0
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use error::TaskSpammerError;
use std::time::Duration;
use tokio::time::sleep;

/// Task spammer errors
pub mod error;

/// Task spammer builder
#[derive(Debug)]
<<<<<<< HEAD
pub struct TaskSpammerBuilder<I, TM> {
=======
pub struct TaskSpammerBuilder<I, TM, P, N, Input> {
>>>>>>> v2-dev-0
    iter: Option<I>,
    interval: Duration,
    quorum_threshold: Option<QuorumThresholdPercentage>,
    quorums: Option<Vec<QuorumNum>>,
    task_manager: TM,
<<<<<<< HEAD
}

impl<I, TM> TaskSpammerBuilder<I, TM>
where
    TM: TaskManager,
=======
    _phantom: std::marker::PhantomData<(P, N, Input)>,
}

impl<I, TM, P, N, Input> TaskSpammerBuilder<I, TM, P, N, Input>
where
    TM: TaskManagerContract<Input, P, N>,
    P: Provider<N>,
    N: Network,
>>>>>>> v2-dev-0
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
<<<<<<< HEAD
    pub fn with_iter(self, iter: I) -> Self
=======
    pub fn with_iter(self, iter: I) -> TaskSpammerBuilder<I, TM, P, N, Input>
>>>>>>> v2-dev-0
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
<<<<<<< HEAD
    pub fn build(self) -> Result<TaskSpammer<I, TM>, TaskSpammerError> {
=======
    pub fn build(self) -> Result<TaskSpammer<I, TM, P, N, Input>, TaskSpammerError>
    where
        TM: TaskManagerContract<Input, P, N>,
        P: Provider<N>,
        N: Network,
    {
>>>>>>> v2-dev-0
        Ok(TaskSpammer {
            iter: self.iter.ok_or(TaskSpammerError::IteratorNotSet)?,
            interval: self.interval,
            quorum_threshold: self
                .quorum_threshold
                .ok_or(TaskSpammerError::QuorumThresholdNotSet)?,
            quorums: self.quorums.ok_or(TaskSpammerError::QuorumNotSet)?,
            task_manager: self.task_manager,
        })
    }
}

/// Task spammer struct
#[derive(Debug)]
<<<<<<< HEAD
pub struct TaskSpammer<I, TM> {
=======
pub struct TaskSpammer<I, TM, P, N, Input> {
>>>>>>> v2-dev-0
    iter: I,
    interval: Duration,
    quorum_threshold: QuorumThresholdPercentage,
    quorums: Vec<QuorumNum>,
    task_manager: TM,
<<<<<<< HEAD
}

impl<I, TM> TaskSpammer<I, TM>
where
    TM: TaskManager + Send + Sync,
=======
    _phantom: std::marker::PhantomData<(P, N, Input)>,
}

impl<I, TM, P, N, Input> TaskSpammer<I, TM, P, N, Input>
where
    TM: TaskManagerContract<Input, P, N> + Send + Sync,
    P: Provider<N>,
    N: Network,
>>>>>>> v2-dev-0
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
