use eigen_task_manager::{
    response_calculator::{FunctionResponseCalculator, ResponseCalculator},
    TaskManagerError,
};
use rand::Rng;
use tracing::info;

/// Helper to wrap both correct and incorrect logic in a single closure.
/// USE THIS FOR TESTING PURPOSES ONLY
///
/// # Arguments
///
/// * `response_calculator` - The response calculator to use.
/// * `invalid_values_builder` - The incorrect logic to respond to the task.
/// * `failure_rate_percentage` - The failure rate percentage.
///
/// # Returns
///
/// * `impl ResponseCalculator<Input, Output>` - The wrapped logic.
///
/// # Panics
///
/// Panics if `failure_rate_percentage` is greater than 100.
pub fn failing_response_calculator<F, Input, Output>(
    response_calculator: impl ResponseCalculator<Input, Output>,
    invalid_values_builder: impl Fn() -> Output,
    failure_rate_percentage: u32,
) -> impl ResponseCalculator<Input, Output>
where
    Input: Clone,
{
    assert!(
        failure_rate_percentage <= 100,
        "Failure rate percentage must be less than or equal to 100"
    );

    FunctionResponseCalculator::<F>::new_async(async move |task_index, input: Input| {
        let result = response_calculator
            .compute_response(task_index, input.clone())
            .await;

        let mut rng = rand::thread_rng();
        let should_fail = rng.gen_bool(failure_rate_percentage as f64 / 100.0);

        if should_fail {
            info!("Operator compute the task with a wrong response");
            Ok(invalid_values_builder())
        } else {
            info!("Operator compute the task successfully");
            result
        }
    })
}
