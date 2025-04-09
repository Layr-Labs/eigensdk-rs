use futures_util::future::BoxFuture;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

pub mod task_generator;

/// Task generator.
pub struct TaskGenerator;

impl TaskGenerator {
    /// Returns the builder to configure the task generator.
    /// The generic parameter `T` is the type of the elements of the iterator.
    pub fn builder<T>() -> TaskGeneratorBuilder<T> {
        TaskGeneratorBuilder {
            iter: None,
            sender: None,
            interval: None,
        }
    }
}

/// Builder to configure the task generator.
pub struct TaskGeneratorBuilder<T> {
    iter: Option<Box<dyn Iterator<Item = T> + Send>>,
    sender: Option<Box<dyn Fn(T) -> BoxFuture<'static, ()> + Send>>,
    interval: Option<Duration>,
}

/// Builder to configure the task generator.
impl<T> TaskGeneratorBuilder<T> {
    /// Set the iterator.
    pub fn with_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = T> + Send + 'static,
    {
        self.iter = Some(Box::new(iter));
        self
    }

    /// Set the sender function.
    pub fn with_sender<F, Fut>(mut self, sender: F) -> Self
    where
        F: Fn(T) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.sender = Some(Box::new(move |item: T| -> BoxFuture<'static, ()> {
            Box::pin(sender(item))
        }));
        self
    }

    /// Set the interval between each task execution.
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    /// Execute the task generator.
    pub async fn run(self) {
        let iter = self.iter.expect("Iterator not set. Usa with_iter()");
        let sender = self.sender.expect("Sender not set. Usa with_sender()");
        let interval = self
            .interval
            .expect("Interval not set. Usa with_interval()");

        for item in iter {
            sender(item).await;
            sleep(interval).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::time::Duration;

    #[derive(Debug)]
    struct TaskManager {
        quorum: usize,
        quorum_threshold: usize,
    }

    impl TaskManager {
        fn new(quorum: usize, quorum_threshold: usize) -> Self {
            Self {
                quorum,
                quorum_threshold,
            }
        }

        async fn create_new_task(&self, task_id: i32) {
            dbg!(&self, task_id);
        }
    }

    #[tokio::test]
    async fn test_task_generator_builder() {
        // Creamos un contador atómico para verificar cuántas veces se ejecuta el sender

        let task_manager = Arc::new(TaskManager::new(0, 60));
        let task_manager_clone = task_manager.clone();

        // Configuramos el generador
        TaskGenerator::builder()
            .with_iter(0..10)
            .with_sender(move |item| {
                let task_manager = task_manager_clone.clone();
                async move {
                    task_manager.create_new_task(item).await;
                }
            })
            .with_interval(Duration::from_secs(10))
            .run()
            .await;
    }
}
