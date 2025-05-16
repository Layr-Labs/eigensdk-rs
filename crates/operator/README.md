# Operator

## What is an Operator

Operators are off-chain nodes that perform, sign, and submit verifiable computations for Autonomous Verifiable Services (AVSs) using Ethereum restaking for security. They first register on EigenLayer’s core contracts, then opt-in to provide a range of services to AVSs. Operators listen for new task events, execute the supplied computation logic, cryptographically sign the results with `BLS/ECDSA` keys, and finally send the proofs to an aggregator for final consolidation.

## How the Logic Works

The Operator functions through the following flow:

1. **Task Subscription**: 
   - The operator [subscribes](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/operator/src/lib.rs#L137-L141) to specific event signatures emitted by task processors
   - Uses WebSocket connection to listen to blockchain events
   - Filters only for the specific task type it's designed to handle

2. **Task Processing**:
   - When a new task is detected, it extracts the task index and input data
   - Applies a [computation function](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/operator/src/lib.rs#L145-L149) to the input data. This computation function is provided when starting the operator.

3. **Response Signing**:
   - [Signs](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/operator/src/lib.rs#L176-L191) the computed result using the operator's BLS Key Pair.
   - Creates a `SignedTaskResponse` containing the result, signature, and operator ID

4. **Response Submission**:
   - [Sends](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/crates/operator/src/client.rs#L47-L81) the signed response to an Aggregator service

## How to Set Up an Operator

Key components needed:

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

2. **Processing Logic**: Implement the computation function that processes task inputs and produces outputs
   - This function will be called when the operator receives a `NEW_TASK_EVENT_SELECTOR` event.

    ```rust
        // Your custom logic to process the input and generate a response. 
        // Example: square the input.
        pub async fn square(
            task_index: u32,
            number_to_be_squared: U256
        ) -> Result<U256, TaskManagerError> {
            Ok(number_to_be_squared * number_to_be_squared)
        }
    ```

3. **Response Calculator**: To abstract your computation into the operator, we provide a `ResponseCalculator` trait with a standar struct that implements the trait and helpers for turning your functions into implementations:
   - `response_calculator_from_fn`: Create a response calculator from your computation function.
   - `response_calculator_from_async_fn`: Create a response calculator from your async computation function.
   - This struct is useful if you want to save state in the operator.

    ```rust
        let response_calculator = response_calculator_from_fn(square);
    ```

   - Wrap your logic with `failing_response_calculator` to inject failures and a given failure rate. This will help you test what happens when the operator responds incorrectly to a task and see how slashing works. **Use this for testing purposes only.**
    
    ```rust
        let logic = failing_response_calculator(response_calculator, || U256::from(42), 60);
    ```

4. **Create the operator configuration**: Configure the operator with the following parameters:
   - `bls_private_key`: The BLS private key for
   - `operator_address`: The address of the operator
   - `operator_name`: The name of the operator
   - `ws_rpc_url`: The WebSocket RPC URL of the Ethereum node
   - `http_rpc_url`: The HTTP RPC URL of the Ethereum node
   - `registry_coordinator_address`: The address of the registry coordinator
   - `operator_state_retriever_address`: The address of the operator state retriever
   - `aggregator_ip_port`: The IP and port of the aggregator
   - `registration`: The registration of the operator. If you don't want to register the operator, you can set this to `None`.

    ```rust
        let config = OperatorConfig {
            bls_private_key,
            operator_address,
            operator_name,
            ws_rpc_url,
            http_rpc_url,
            registry_coordinator_address,
            operator_state_retriever_address,
            aggregator_ip_port: server_address,
            registration,
        };
    ```

5. **Run the operator**: Initialize the operator with the configuration and start it with the processing logic

    ```rust
        let operator = Operator::new(logger, config).await.unwrap();
        operator.start::<ISTaskManager>(logic).await.unwrap();
    ```

## Examples

Here are some examples of operators that are already implemented:

- [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/awesome-vault-service/src/bin/operator.rs)
- [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-dot-product/src/bin/operator.rs)


