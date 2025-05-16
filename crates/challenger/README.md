# Challenger

## What is a Challenger

A Challenger in the context of EigenLayer is a validator component that monitors the network for the creation of new tasks and task responses submitted by operators, verifies their correctness, and raises challenges when incorrect responses are detected. If the challenge is successful, the operator will be slashed.

## How the Logic Works

The Challenger operates through a well-defined event-driven workflow:

1. **Event Subscription**:
   - [Subscribes](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/challenger/src/lib.rs#L71-L82) to blockchain events for new tasks and task responses using WebSocket connections
   - Monitors for `NewTaskEvent` to track new tasks created in the system
   - Watches for `TaskResponseEvent` when operators submit responses to tasks

2. **Verification Process**:
   - When a [new task is detected](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/challenger/src/lib.rs#L86-L89), user defined logic is used to process the new task
   - When a [task response is received](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/challenger/src/lib.rs#L90-L103), user defined logic is used to process the task response and verify it
   - Uses a user-defined verification function to determine if the response is correct
   - The verification logic can be customized based on the specific AVS requirements

3. **Challenge Mechanism**:
   - If a response is verified as correct, the challenger logs the result and takes no action
   - If a response is determined to be incorrect, the challenger raises a challenge
   - Includes identifying the non-signing operators who might have abstained from the incorrect response

## How to Set Up a Challenger

1. **Task Manager Definition**: Create a struct implementing the `TaskManagerDefs` trait that defines:
   - `Input` and `Output` types for your tasks
   - `NEW_TASK_EVENT_SELECTOR` - the event signature for new task events
   - Use the `impl_task_manager_from_defs_and_contract` macro to build your `TaskManager`.

    ```rust
        // Implement the [`TaskManagerDefs`] trait for a unit struct.
        // You need to specify the input and output types of the task.
        // You also need to specify the selectors for the new task event and the task responded event.
        pub struct ISTaskManager;

        impl TaskManagerDefs for ISTaskManager {
            type Input = U256;
            type Output = U256;
            const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
            const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
        }

        impl_task_manager_from_defs_and_contract!(ISTaskManager => YOUR_BINDING_CONTRACT_INSTANCE);
    ```


2. **Task Verification Logic**: Define a function that computes the expected result for a task, which will be used to verify operator responses
   - This would be the logic to compute a new task.

    ```rust
        pub fn square(_task_index: u32, number_to_be_squared: U256) -> Result<U256, TaskManagerError> {
            Ok(number_to_be_squared * number_to_be_squared)
        }
    ```

3. **Response Calculator**: Create a response calculator from your verification function. We provide a `ResponseCalculator` trait with a standar `FunctionResponseCalculator` struct. This implements the trait and helpers for turning your functions into implementations:
   - `response_calculator_from_fn`: Create a response calculator from your sync function.
   - `response_calculator_from_async_fn`: Create a response calculator from your async function.

    ```rust
        let response_calculator = response_calculator_from_fn(square);
    ```

4. **Verifier**: Create a verifier from the response calculator.
   - This will be in charge computing the response of a task and comparing it with the operator's response.

    ```rust
        let logic = verifier_from_compute_function(response_calculator);
    ```

5. **Challenger Task Processor**: Create a `ChallengerTaskProcessor` trait implementation from the task manager and the verifier.
   - This will be in charge of processing the task and the response.
   - We provide a standard `IndexingChallengerProcessor` implementation that can be used as a starting point.

    ```rust
        let task_processor = IndexingChallengerProcessor::new(contract, logic);
    ```

6. **Challenger Configuration**: Configure the challenger with the following parameters:
   - `http_rpc_url`: The HTTP RPC URL of the Ethereum node
   - `ws_rpc_url`: The WebSocket RPC URL of the Ethereum node

    ```rust
        let config = ChallengerConfig {
            http_rpc_url,
            ws_rpc_url,
        };
    ```

7. **Challenger Initialization**: Initialize the challenger with the configuration and start it with the processing logic

    ```rust
        let mut challenger = Challenger::new(config, task_processor);
        challenger.start_challenger().await?;
    ```

## Examples

Here are some examples of operators that are already implemented:

- [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/awesome-vault-service/src/bin/challenger.rs)
- [Incredible Squaring](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-squaring/src/bin/challenger.rs)
