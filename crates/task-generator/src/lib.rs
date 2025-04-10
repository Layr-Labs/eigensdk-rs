//! This is a simple task generator that can be used to create tasks for the operators.
//! For testing purposes.

use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use error::TaskGeneratorError;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Task generator errors
pub mod error;

/// Task generator struct
#[derive(Debug)]
pub struct TaskGenerator;

impl TaskGenerator {
    /// Builder for the task generator
    ///
    /// # Returns
    ///
    /// * `TaskGeneratorBuilder` - The builder for the task generator
    pub fn builder<T>() -> TaskGeneratorBuilder<T> {
        TaskGeneratorBuilder {
            iter: None,
            interval: Duration::ZERO,
            quorum_threshold: None,
            quorums: None,
        }
    }
}

/// Builder for the task generator
pub struct TaskGeneratorBuilder<T> {
    iter: Option<Box<dyn Iterator<Item = T> + Send>>,
    interval: Duration,
    quorum_threshold: Option<QuorumThresholdPercentage>,
    quorums: Option<Vec<QuorumNum>>,
}

impl<T> TaskGeneratorBuilder<T> {
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
    pub fn with_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = T> + Send + 'static,
    {
        self.iter = Some(Box::new(iter));
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
        F: Fn(T, QuorumThresholdPercentage, Vec<QuorumNum>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), TaskGeneratorError>> + Send,
    {
        let iter = self.iter.ok_or(TaskGeneratorError::IteratorNotSet)?;
        let quorum_threshold = self
            .quorum_threshold
            .ok_or(TaskGeneratorError::QuorumThresholdNotSet)?;
        let quorums = self
            .quorums
            .clone()
            .ok_or(TaskGeneratorError::QuorumNotSet)?;

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
    use std::iter::Empty;
    use std::time::Duration;

    #[tokio::test]
    async fn test_task_generator_with_u64() {
        let names = vec!["John", "Jane", "Jim", "Jill"];

        TaskGenerator::builder()
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

        TaskGenerator::builder()
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
        let result = TaskGenerator::builder()
            .with_iter(0..5)
            .with_interval(Duration::from_millis(50))
            .run(|_, _, _| async move { Ok(()) })
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_task_generator_without_iter() {
        // Need to specify the type for the iterator when not provided
        let result = TaskGenerator::builder::<Empty<u32>>()
            .with_interval(Duration::from_millis(50))
            .run(|_, _, _| async move { Ok(()) })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_task_generator_without_interval() {
        let result = TaskGenerator::builder()
            .with_iter(0..5)
            .with_quorum(50, vec![0])
            .run(|_, _, _| async move { Ok(()) })
            .await;

        assert!(result.is_ok());
    }
}
