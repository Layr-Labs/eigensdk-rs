pub mod task_generator;

/// Task generator.
pub struct TaskGenerator;

impl TaskGenerator {
    /// Returns the builder to configure the task generator.
    /// The generic parameter `T` is the type of the elements of the iterator.
    pub fn builder() -> TaskGeneratorBuilder {
        TaskGeneratorBuilder { iter: None }
    }
}

/// Builder to configure the task generator.

pub struct TaskGeneratorBuilder {
    iter: Option<Box<dyn Iterator<Item = u32> + Send>>,
}

/// Builder to configure the task generator.
impl TaskGeneratorBuilder {
    /// Set the iterator.
    pub fn with_iter<I>(mut self, iter: I) -> Self
    where
        I: Iterator<Item = u32> + Send + 'static,
    {
        self.iter = Some(Box::new(iter));
        self
    }

    /// Execute the task generator.
    pub async fn run(self) {
        let iter = self.iter.expect("Iterator not set. Usa with_iter()");

        for item in iter {
            println!("Item: {}", &item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_generator_builder() {
        // Configuramos el generador
        TaskGenerator::builder().with_iter(0..10).run().await;
    }
}
