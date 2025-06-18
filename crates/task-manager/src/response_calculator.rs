use std::{future::Future, sync::Arc};

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
    ) -> impl Future<Output = Result<Output, TaskManagerError>> + Send;
}

/// Implementation of the [`ResponseCalculator`] trait that uses a function to compute the response.
#[derive(Debug)]
pub struct FunctionResponseCalculator<F>(F);

impl<F, Fut, Input, Output> ResponseCalculator<Input, Output> for FunctionResponseCalculator<F>
where
    F: Fn(u32, Input) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Output, TaskManagerError>> + Send,
    Input: Send,
    Output: Send,
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
) -> FunctionResponseCalculator<impl AsyncFnSend<Input, Output>>
where
    CF: Fn(u32, Input) -> Result<Output, TaskManagerError> + Send + Sync,
    Input: Send,
    Output: Send,
{
    let compute = Arc::new(compute_fn);
    FunctionResponseCalculator(move |a, b| {
        let compute = compute.clone();
        async move { compute(a, b) }
    })
}

/// Async function closure alias
///
/// This helper trait exists only to express the type returned by
/// [`response_calculator_from_fn`]: a closure that returns a `Future`
/// whose output is `Result<Output, TaskManagerError>` and is `Send`, so it can
/// be used in any context that requires the `Send` bound.
///
/// Stable Rust can’t write that type directly, so we wrap it in this alias.
/// **NOTE: users should not implement it manually.**
pub trait AsyncFnSend<Input, Output>: Fn(u32, Input) -> Self::Future + Send {
    /// Future type returned by the closure
    type Future: Future<Output = Result<Output, TaskManagerError>> + Send;
}

impl<F, Fut, Input, Output> AsyncFnSend<Input, Output> for F
where
    F: Fn(u32, Input) -> Fut + Send,
    Fut: Future<Output = Result<Output, TaskManagerError>> + Send,
{
    type Future = Fut;
}
