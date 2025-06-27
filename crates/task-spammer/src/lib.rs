//! # Task Spammer
//!
//! ## What is a Task Spammer
//!
//! A Task Spammer is a testing utility designed to generate tasks at configurable intervals.
//! It serves as a simulation tool that allows developers to test how operators, aggregators
//! and challengers respond to a continuous stream of new tasks.
//!
//! ## How the Logic Works
//!
//! The Task Spammer uses a builder pattern and follows this workflow:
//!
//! 1. **Task Generation Logic**:
//!    - Uses an iterator to produce a sequence of task inputs
//!    - Each iterator value becomes the input for a new task
//!    
//! 2. **Task Submission**:
//!    - Connects to a TaskManager contract
//!    - Calls `create_new_task` with the generated input
//!    - Specifies quorum requirements for each task and a quorum threshold percentage,
//!      which is the percentage of operators that must respond to consider the task complete
//!
//! ## How to Set Up a Task Spammer
//!
//! 1. **Task Manager Definition**: Create a struct implementing the `TaskManagerDefs` trait that defines:
//!    - `Input` and `Output` types for your tasks
//!    - `NEW_TASK_EVENT_SELECTOR` - the event signature for new task events
//!    - Use the `impl_task_manager_from_defs_and_contract` macro to build your `TaskManager`.
//!
//!       ```ignore
//!           // Implement the [`TaskManagerDefs`] trait for a unit struct.
//!           // You need to specify the input and output types of the task.
//!           // You also need to specify the selectors for the new task event and the task responded event.
//!           pub struct ISTaskManager;
//!
//!           impl TaskManagerDefs for ISTaskManager {
//!               type Input = U256;
//!               type Output = U256;
//!               const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
//!               const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
//!           }
//!
//!           impl_task_manager_from_defs_and_contract!(ISTaskManager => IncredibleSquaringTaskManagerInstance);
//!       ```
//!
//! 2. **Task Manager Contract**: Create an instance of your `TaskManager` contract:
//!     - This struct should come from your bindings
//!
//!       ```ignore
//!           let contract = IncredibleSquaringTaskManagerInstance(task_manager_address, provider);
//!       ```
//! 3. **Build the Task Spammer**:
//!    - Use the [`TaskSpammerBuilder`] to configure the task spammer
//!
//!       ```ignore
//!           let mut builder = TaskSpammerBuilder::new(contract);
//!       ```
//!
//! 4. **Input Generator**: Define an iterator that creates appropriate input values for your specific AVS
//!    - The iterator can be infinite or finite depending on your needs
//!    - The input values will be passed to the `create_new_task` function on the TaskManager contract
//!
//!       ```ignore
//!           builder = builder.with_iter((0..).map(U256::from));
//!       ```
//!
//! 5. **Quorum Configuration**:
//!    - Set the quorum threshold percentage
//!    - Specify to which quorums the task will be sent
//!
//!       ```ignore
//!           builder = builder.with_quorum(50, vec![0]);
//!       ```
//!
//! 6. **Interval Settings**: Define how frequently tasks should be created
//!    - The interval is the time between task creations
//!
//!       ```ignore
//!           builder = builder.with_interval(Duration::from_secs(10));
//!       ```
//! 7. **Build and Run the Task Spammer**:
//!    - Build the task spammer using the [`build()`](TaskSpammerBuilder::build) method
//!    - Call the [`run()`](TaskSpammer::run) method to start the task spammer
//!
//!       ```ignore
//!           TaskSpammerBuilder::new(contract)
//!               .with_iter((0..).map(U256::from)) // (3)
//!               .with_quorum(50, vec![0]) // (4)
//!               .with_interval(Duration::from_secs(10)) // (5)
//!               .build()
//!               .unwrap()
//!               .run()
//!               .await
//!               .unwrap();
//!       ```
//!
//! ## Examples
//!
//! Here are some examples of task spammer implementations:
//!
//! - [Incredible Squaring](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-squaring/src/bin/task_spammer.rs)
//! - [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-dot-product/src/bin/task-spammer.rs)
//! - [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/awesome-vault-service/src/bin/task-spammer.rs)
//!

use eigen_task_manager::TaskManager;
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use error::TaskSpammerError;
use std::time::Duration;
use tokio::{task::JoinHandle, time::sleep};
use tracing::info;

/// Task spammer errors
pub mod error;

/// A Task Spammer is a testing utility designed to generate tasks at configurable intervals.
/// It serves as a simulation tool that allows developers to test how operators, aggregators
/// and challengers respond to a continuous stream of new tasks.
///
/// To more in-depth details about the task spammer, refer to the [module documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L110).
#[derive(Debug)]
pub struct TaskSpammerBuilder<I, TM> {
    iter: Option<I>,
    interval: Duration,
    quorum_threshold: Option<QuorumThresholdPercentage>,
    quorums: Option<Vec<QuorumNum>>,
    task_manager: TM,
}

impl<I, TM> TaskSpammerBuilder<I, TM>
where
    TM: TaskManager,
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
    pub fn with_iter(self, iter: I) -> Self
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
    pub fn build(self) -> Result<TaskSpammer<I, TM>, TaskSpammerError> {
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
pub struct TaskSpammer<I, TM> {
    iter: I,
    interval: Duration,
    quorum_threshold: QuorumThresholdPercentage,
    quorums: Vec<QuorumNum>,
    task_manager: TM,
}

impl<I, TM> TaskSpammer<I, TM>
where
    TM: TaskManager + Send + Sync + 'static,
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
        info!("Starting task spammer");
        for input in self.iter {
            info!("Creating new task with input {:?}", input);
            self.task_manager
                .create_new_task(input, self.quorum_threshold, self.quorums.clone())
                .await?;
            sleep(self.interval).await;
        }
        Ok(())
    }

    /// Starts the task spammer in the background.
    ///
    /// Equivalent to [`Self::run`], but spawns it in the background and returns a
    /// [`JoinHandle`] to the background task.
    ///
    /// # Returns
    ///
    /// * `JoinHandle<Result<(), TaskSpammerError>>` - The handle to the background task
    pub fn start(self) -> JoinHandle<Result<(), TaskSpammerError>>
    where
        I: Iterator<Item = TM::Input> + Send + 'static,
        I::Item: Clone + Send + 'static,
    {
        tokio::spawn(self.run())
    }
}
