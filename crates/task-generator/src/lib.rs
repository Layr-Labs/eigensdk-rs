//! Task generator for the Eigen Layer

use async_trait::async_trait;
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use std::error::Error;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

/// Task generator errors
pub mod error;

#[async_trait]
/// Trait for processing tasks
pub trait TaskProcess<T> {
    /// Create a new task with a generic input
    ///
    /// # Arguments
    ///
    /// * `task_number` - The number of the task
    /// * `input` - The input for the task
    /// * `quorum_threshold` - The quorum threshold for the task
    /// * `quorum` - The quorum for the task
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error + Send + Sync>>` - The result of the task
    async fn create_new_task(
        &self,
        task_index: u32,
        input: T,
        quorum_threshold: QuorumThresholdPercentage, // TODO: It's ok to have this here?
        quorum: Vec<QuorumNum>,                      // TODO: It's ok to have this here?
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

/// Task generator struct
#[derive(Debug)]
pub struct TaskGenerator;

impl TaskGenerator {
    /// Builder for the task generator
    ///
    /// # Returns
    ///
    /// * `TaskGeneratorBuilder` - The builder for the task generator
    pub fn builder() -> TaskGeneratorBuilder {
        TaskGeneratorBuilder {
            iter: None,
            interval: None,
        }
    }
}

/// Builder for the task generator
pub struct TaskGeneratorBuilder {
    iter: Option<Box<dyn Iterator<Item = u32> + Send>>,
    interval: Option<Duration>,
}

impl TaskGeneratorBuilder {
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
        I: Iterator<Item = u32> + Send + 'static,
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
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Run the task generator
    ///
    /// # Arguments
    ///
    /// * `sender` - The sender for the task creation. This is a function that takes a task number and returns a future that resolves to a result.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error + Send + Sync>>` - The result of the task
    pub async fn run<F, Fut>(self, sender: F)
    where
        F: Fn(u32) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send,
    {
        // TODO: Find a better way to handle the iterator unwrap
        let iter = self.iter.unwrap_or(Box::new(0..10));
        let interval = self.interval.unwrap_or(Duration::ZERO);

        for task_index in iter {
            // TODO: Handle the error
            let _ = sender(task_index)
                .await
                .map_err(|e| error!("Error creating task {}: {:?}", task_index, e));
            sleep(interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::error::Error;
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_task_generator_with_u64() {
        #[derive(Clone)]
        struct MyTaskProcessor;

        #[async_trait]
        impl TaskProcess<u64> for MyTaskProcessor {
            async fn create_new_task(
                &self,
                task_index: u32,
                input: u64,
                _quorum_threshold: QuorumThresholdPercentage,
                _quorum: Vec<QuorumNum>,
            ) -> Result<(), Box<dyn Error + Send + Sync>> {
                println!("Task {} created. Processing input {}", task_index, input);
                Ok(())
            }
        }

        let processor = MyTaskProcessor;
        TaskGenerator::builder()
            .with_iter(0..10)
            .with_interval(Duration::from_millis(10))
            .run(|i| {
                let processor = processor.clone();
                async move { processor.create_new_task(i, 32, 100, vec![0]).await }
            })
            .await;
    }

    #[tokio::test]
    async fn test_task_generator_with_struct() {
        #[derive(Clone, Debug)]
        #[allow(dead_code)]
        struct Input {
            description: String,
            value: u32,
        }

        #[derive(Clone)]
        struct TaskProcessor;

        #[async_trait]
        impl TaskProcess<Input> for TaskProcessor {
            async fn create_new_task(
                &self,
                task_index: u32,
                input: Input,
                quorum_threshold: QuorumThresholdPercentage,
                quorum: Vec<QuorumNum>,
            ) -> Result<(), Box<dyn Error + Send + Sync>> {
                println!(
                    "Task {} created with input: {:?}, quorum_threshold: {}, quorum: {}",
                    task_index,
                    input,
                    quorum_threshold,
                    quorum.first().unwrap(),
                );
                Ok(())
            }
        }

        let processor = Arc::new(TaskProcessor);
        TaskGenerator::builder()
            .with_iter(0..5)
            .with_interval(Duration::from_millis(50))
            .run(|i| {
                let processor = processor.clone();
                let input = Input {
                    description: format!("Description for task {}", i),
                    value: i * 10,
                };
                async move { processor.create_new_task(i, input, 50, vec![0]).await }
            })
            .await;
    }
}
