use std::future::Future;

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
pub struct FunctionResponseCalculator<F>(F);

impl<F, Fut, Input, Output> ResponseCalculator<Input, Output> for FunctionResponseCalculator<F>
where
    F: Fn(u32, Input) -> Fut + Send,
    Fut: Future<Output = Result<Output, TaskManagerError>> + Send,
{
    async fn compute_response(
        &self,
        task_index: u32,
        input: Input,
    ) -> Result<Output, TaskManagerError> {
        (self.0)(task_index, input).await
    }
}

/// Create a new [`FunctionResponseCalculator`] from an async function.
///
/// NOTE: This function is outside of the `FunctionResponseCalculator` struct to avoid
/// specifying the type of `F` when creating a new `FunctionResponseCalculator`.
///
/// # Arguments
///
/// * `compute_fn` - The async function to compute the response.
///
/// # Returns
///
/// * [`FunctionResponseCalculator`] - The new [`FunctionResponseCalculator`].
pub fn response_calculator_from_async_fn<CF, Fut, Input, Output>(
    compute_fn: CF,
) -> FunctionResponseCalculator<CF>
where
    CF: Fn(u32, Input) -> Fut + Send,
    Fut: Future<Output = Result<Output, TaskManagerError>> + Send,
{
    FunctionResponseCalculator(compute_fn)
}

/// Create a new [`FunctionResponseCalculator`] from a sync function.
/// This function will be converted to an async function.
///
/// NOTE: This function is outside of the `FunctionResponseCalculator` struct to avoid
/// specifying the type of `F` when creating a new `FunctionResponseCalculator`.
///
/// # Arguments
///
/// * `compute_fn` - The sync function to compute the response.
///
/// # Returns
///
/// * [`FunctionResponseCalculator`] - The new [`FunctionResponseCalculator`].
pub fn response_calculator_from_fn<CF, Input, Output>(
    compute_fn: CF,
) -> FunctionResponseCalculator<impl AsyncFn(u32, Input) -> Result<Output, TaskManagerError>>
where
    CF: Fn(u32, Input) -> Result<Output, TaskManagerError>,
{
    FunctionResponseCalculator(async move |a, b| compute_fn(a, b))
}
