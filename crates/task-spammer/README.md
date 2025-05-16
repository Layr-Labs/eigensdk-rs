# Task Spammer

## What is a Task Spammer

A Task Spammer is a testing utility designed to generate tasks at configurable intervals. It serves as a simulation tool that allows developers to test how operators, aggregators, and challengers respond to a continuous stream of new tasks.

## How the Logic Works

The Task Spammer uses a builder pattern and follows this workflow:

1. **Task Generation Logic**:
   - Uses an iterator to produce a sequence of task inputs
   - Each iterator value becomes the input for a new task
   
2. **Task Submission**:
   - Connects to a TaskManager contract
   - Calls `create_new_task` with the generated input
   - Specifies quorum requirements for each task and a quorum threshold percentage, which is the percentage of operators that must respond to consider the task complete

## How to Set Up a Task Spammer

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

2. **Input Generator**: Define an iterator that creates appropriate input values for your specific AVS
   - The iterator can be infinite or finite depending on your needs
   - The input values will be passed to the `create_new_task` function on the TaskManager contract

    ```rust
        // Create your task input
        let task_inputs = (0..).map(U256::from);
    ```

3. **Quorum Configuration**:
   - Set the quorum threshold percentage (e.g., 50 means 50% of operators must respond)
   - Specify which quorums will process the tasks (e.g., `vec![0]` for the first quorum)

    ```rust
        // Set the quorum parameters (threshold percentage and quorum numbers)
        let quorum_threshold = 50;
        let quorum_numbers = vec![0];
    ```

4. **Interval Settings**: Define how frequently tasks should be created
   - The interval is the time between task creations in seconds

    ```rust
        // Set the interval between task creations (in seconds)
        let interval = Duration::from_secs(10);
    ``` 

5. **Build and Run the Task Spammer**:
   - Use the `TaskSpammerBuilder` to configure and build the task spammer
   - Call the `run` method to start the task spammer

    ```rust
        // Build and run the task spammer
        TaskSpammerBuilder::new(contract)
            .with_iter(task_inputs)
            .with_quorum(quorum_threshold, quorum_numbers)
            .with_interval(interval)
            .build()
            .unwrap()
            .run()
            .await
            .unwrap();
    ```

## Examples

Here's how to build a Task Spammer based on the examples:
- [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/awesome-vault-service/src/bin/task-spammer.rs)
- [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-1/examples/incredible-dot-product/src/bin/task-spammer.rs)
