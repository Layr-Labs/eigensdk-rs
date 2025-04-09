use async_trait::async_trait;
use std::error::Error;
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

#[async_trait]
pub trait TaskProcess<T> {
    async fn create_new_task(
        &self,
        task_number: u64,
        input: T,
        quorum_threshold: u64, // REVIEW
        quorum: u64,           // REVIEW
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct TaskGenerator;

impl TaskGenerator {
    pub fn builder() -> TaskGeneratorBuilder {
        TaskGeneratorBuilder {
            iter: None,
            interval: None,
        }
    }
}

pub struct TaskGeneratorBuilder {
    iter: Option<Box<dyn Iterator<Item = u64> + Send>>,
    interval: Option<Duration>,
}

impl TaskGeneratorBuilder {
    pub fn with_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = u64> + Send + 'static,
    {
        self.iter = Some(Box::new(iter));
        self
    }

    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = Some(interval);
        self
    }

    pub async fn run<F, Fut>(self, sender: F)
    where
        F: Fn(u64) -> Fut + Send + Sync,
        Fut: Future<Output = Result<(), Box<dyn Error + Send + Sync>>> + Send,
    {
        let iter = self.iter.expect("Iterator not set. Use with_iter()");
        let interval = self.interval.unwrap_or(Duration::ZERO);

        for task_number in iter {
            sender(task_number)
                .await
                .map_err(|e| error!("Error creating task {}: {:?}", task_number, e))
                .unwrap();
            sleep(interval).await;
        }
    }
}
