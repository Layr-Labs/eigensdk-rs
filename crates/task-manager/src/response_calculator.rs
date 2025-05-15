use rand::Rng;
use std::{future::Future, marker::PhantomData};
use tracing::info;

use crate::TaskManagerError;

/// Trait used to compute the response of a task.
pub trait ResponseCalculator<Input, Output> {
    /// Compute the response of a task.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task.
    /// * `input` - The input of the task.
    ///
    /// # Returns
    ///
    /// * `Result<Output, TaskManagerError>` - The response of the task.
    fn compute_response(
        &self,
        task_index: u32,
        input: Input,
    ) -> impl Future<Output = Result<Output, TaskManagerError>>;
}

/// Implementation of the [`ResponseCalculator`] trait that uses a function to compute the response.
#[derive(Debug)]
pub struct FunctionResponseCalculator<Input, Output, F>
where
    F: AsyncFn(u32, Input) -> Result<Output, TaskManagerError>,
{
    /// The function used to compute the response.
    pub compute_fn: F,
    _input: PhantomData<Input>,
    _output: PhantomData<Output>,
}

impl<Input, Output, F> FunctionResponseCalculator<Input, Output, F>
where
    F: AsyncFn(u32, Input) -> Result<Output, TaskManagerError>,
{
    /// Create a new `FunctionResponseCalculator`.
    ///
    /// # Arguments
    ///
    /// * `compute_fn` - The function used to compute the response.
    ///
    /// # Returns
    ///
    /// * `FunctionResponseCalculator<Input, Output, F>` - A new `FunctionResponseCalculator`.
    pub fn new(compute_fn: F) -> Self {
        Self {
            compute_fn,
            _input: PhantomData,
            _output: PhantomData,
        }
    }
}

impl<Input, Output, F> ResponseCalculator<Input, Output>
    for FunctionResponseCalculator<Input, Output, F>
where
    F: AsyncFn(u32, Input) -> Result<Output, TaskManagerError>,
{
    async fn compute_response(
        &self,
        task_index: u32,
        input: Input,
    ) -> Result<Output, TaskManagerError> {
        (self.compute_fn)(task_index, input).await
    }
}

/// Helper to wrap both correct and incorrect logic in a single closure.
/// USE THIS FOR TESTING PURPOSES ONLY
///
/// # Arguments
///
/// * `response_calculator` - The response calculator to use.
/// * `incorrect_logic` - The incorrect logic to respond to the task.
/// * `failure_rate_percentage` - The failure rate percentage.
///
/// # Returns
///
/// * `impl ResponseCalculator<Input, Output>` - The wrapped logic.
///
/// # Panics
///
/// Panics if `failure_rate_percentage` is greater than 100.
pub fn failing_response_calculator<Input, Output>(
    response_calculator: impl ResponseCalculator<Input, Output>,
    incorrect_logic: impl AsyncFn(u32, Input) -> Result<Output, TaskManagerError>,
    failure_rate_percentage: u32,
) -> impl ResponseCalculator<Input, Output>
where
    Input: Clone,
{
    assert!(
        failure_rate_percentage <= 100,
        "Failure rate percentage must be less than or equal to 100"
    );

    FunctionResponseCalculator::new(async move |task_index, input: Input| {
        let result = response_calculator
            .compute_response(task_index, input.clone())
            .await;

        let mut rng = rand::thread_rng();
        let should_fail = rng.gen_bool(failure_rate_percentage as f64 / 100.0);

        if should_fail {
            info!("Operator compute the task with a wrong response");
            incorrect_logic(task_index, input).await
        } else {
            info!("Operator compute the task successfully");
            result
        }
    })
}
