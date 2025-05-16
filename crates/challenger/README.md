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
   - This would be the logic to compute the correct response for a task. Since `Challeger` has the input and output types, the function should be of the form `fn(index: u32, input: Input) -> Output`
   - 

    ```rust
        pub fn square(_task_index: u32, number_to_be_squared: U256) -> Result<U256, TaskManagerError> {
            Ok(number_to_be_squared * number_to_be_squared)
        }

        let response_calculator = response_calculator_from_fn(square);
    ```

3. **Response Calculator**: Create a response calculator from your verification function using the provided utility functions

4. **RPC Configuration**: Set up both HTTP and WebSocket connections to an Ethereum node to monitor events and submit challenges


Here's how to build a Challenger based on the examples:

```rust
use alloy::primitives::Address;
use eigensdk::{
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        Challenger,
    },
    task_manager::response_calculator::response_calculator_from_fn,
};
use your_avs_module::{YourTaskManagerInstance, compute_correct_response};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Set up connection parameters
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let private_key = "YOUR_PRIVATE_KEY";
    let task_manager_address = Address::from_str("YOUR_TASK_MANAGER_ADDRESS")?;
    
    // 2. Initialize the task manager contract instance
    let wallet = get_signer(private_key, &http_rpc_url);
    let contract = YourTaskManagerInstance::new(task_manager_address, wallet);
    
    // 3. Create the response calculator and verifier
    // This is the logic that computes what the correct response SHOULD be
    let response_calculator = response_calculator_from_fn(compute_correct_response);
    let verifier = verifier_from_compute_function(response_calculator);
    
    // 4. Initialize the challenger processor with the verifier
    let task_processor = IndexingChallengerProcessor::new(contract, verifier);
    
    // 5. Configure and start the challenger
    let config = ChallengerConfig {
        http_rpc_url,
        ws_rpc_url,
    };
    let mut challenger = Challenger::new(config, task_processor);
    
    // 6. Start the challenger service
    challenger.start_challenger().await?;

    Ok(())
}
```

