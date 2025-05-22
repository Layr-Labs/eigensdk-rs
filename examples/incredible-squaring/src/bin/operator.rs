use alloy::primitives::U256;
use eigensdk::logging::log_level::LogLevel;
use eigensdk::logging::{get_logger, init_logger};
use eigensdk::operator::{config::OperatorConfig, Operator};
use eigensdk::task_manager::response_calculator::response_calculator_from_fn;
use eigensdk::testing_utils::task_processor::failing_response_calculator;
use incredible_squaring::{square, utils::load_config, ISTaskManager};

/// This example shows how to initialize an operator and start to listen for new task events.
/// For this example, Operator should be registered.
/// Follow the [`Operator`] documentation to set up an operator.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`])
/// 2. Create the [`OperatorConfig`]
/// 3. Create the logic to compute the task (Done in [`square`])
/// 4. Build the [`ResponseCalculator`](eigensdk::task_manager::response_calculator::ResponseCalculator)
/// 5. Use the [`failing_response_calculator`] to test how the operator behaves when
///    it responds incorrectly to a task and how slashing works
/// 6. Start the operator
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();

    // 2. Create the `OperatorConfig`
    let config: OperatorConfig = load_config("./src/config/squaring-operator.toml").unwrap();

    // 4. Build the `ResponseCalculator` with the computation function
    let response_calculator = response_calculator_from_fn(square);

    // 5. Use the `failing_response_calculator` with the wrong `Output` type and a given failure rate
    let logic = failing_response_calculator(response_calculator, || U256::from(42), 60);

    // 6. Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();
    operator.start::<ISTaskManager>(logic).await.unwrap();
}
