//! This is a simple task generator that can be used to create tasks for the operators.
//! For testing purposes.

use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use error::TaskGeneratorError;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Task generator errors
pub mod error;
pub mod event_task_created;
pub mod indexing_task_processor;
pub mod new_task_event_generic;
pub mod task;
pub mod task_manager_contract;
pub mod task_response;

/// Task generator struct
#[derive(Debug)]
pub struct TaskGenerator();

impl TaskGenerator {}

/// Builder for the task generator
#[derive(Debug, Default)]
pub struct TaskGeneratorBuilder<I> {
    iter: Option<I>,
    interval: Duration,
    quorum_threshold: Option<QuorumThresholdPercentage>,
    quorums: Option<Vec<QuorumNum>>,
}

impl<I: std::default::Default> TaskGeneratorBuilder<I> {
    /// Builder for the task generator
    ///
    /// # Returns
    ///
    /// * `TaskGeneratorBuilder` - The builder for the task generator
    pub fn builder() -> TaskGeneratorBuilder<I> {
        TaskGeneratorBuilder::default()
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
    /// * `TaskGeneratorBuilder` - The builder for the task generator
    pub fn with_iter(mut self, iter: I) -> TaskGeneratorBuilder<I>
    where
        I: Iterator + Send + 'static,
        I::Item: Send,
    {
        self.iter = Some(iter);
        self
    }

    /// Set the interval for the task creation
    ///
    /// # Arguments
    ///
    /// * `interval` - The interval for the task creation
    ///
    /// # Returns
    ///
    /// * `TaskGeneratorBuilder` - The builder for the task generator
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
    /// * `TaskGeneratorBuilder` - The builder for the task generator
    pub fn with_quorum(
        mut self,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Self {
        self.quorum_threshold = Some(quorum_threshold);
        self.quorums = Some(quorums);
        self
    }

    /// Run the task generator
    ///
    /// # Arguments
    ///
    /// * `task_fn` - The function for the task creation. This is a function that takes a task number and returns a future that resolves to a result.
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskGeneratorError>` - The result of the task
    pub async fn run<F, Fut>(self, task_fn: F) -> Result<(), TaskGeneratorError>
    where
        I: Iterator + Send,
        I::Item: Send + 'static,
        F: Fn(I::Item, QuorumThresholdPercentage, Vec<QuorumNum>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), TaskGeneratorError>> + Send,
    {
        let iter = self.iter.ok_or(TaskGeneratorError::IteratorNotSet)?;
        let quorum_threshold = self
            .quorum_threshold
            .ok_or(TaskGeneratorError::QuorumThresholdNotSet)?;
        let quorums = self.quorums.ok_or(TaskGeneratorError::QuorumNotSet)?;

        for input in iter {
            task_fn(input, quorum_threshold, quorums.clone()).await?;
            sleep(self.interval).await;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_task_generator_with_string() {
        let names = vec!["John", "Jane", "Jim", "Jill"];

        let builder = TaskGeneratorBuilder::builder();
        builder
            .with_iter(names.into_iter())
            .with_quorum(50, vec![0])
            .with_interval(Duration::from_millis(10))
            .run(|input, _, _| async move {
                println!("Hello, {}", input);
                Ok(())
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_task_generator_with_struct() {
        #[derive(Clone, Debug)]
        struct Input {
            description: String,
            value: u32,
        }

        let inputs = vec![
            Input {
                description: "Task 1".to_string(),
                value: 10,
            },
            Input {
                description: "Task 2".to_string(),
                value: 20,
            },
        ];

        TaskGeneratorBuilder::builder()
            .with_iter(inputs.into_iter())
            .with_quorum(50, vec![0])
            .with_interval(Duration::from_millis(50))
            .run(|input, _, _| async move {
                println!("Description: {}", input.description);
                println!("Value: {}", input.value);
                Ok(())
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_task_generator_without_quorum() {
        let result = TaskGeneratorBuilder::builder()
            .with_iter(0..5)
            .with_interval(Duration::from_millis(50))
            .run(|_, _, _| async move { Ok(()) })
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_task_generator_without_interval() {
        let result = TaskGeneratorBuilder::builder()
            .with_iter(0..5)
            .with_quorum(50, vec![0])
            .run(|_, _, _| async move { Ok(()) })
            .await;

        assert!(result.is_ok());
    }
}
