# Aggregator

## What is an Aggregator

An Aggregator in the context of EigenLayer is a service component that collects signed responses from multiple operators for a given task, combines them using BLS signature aggregation, and submits the aggregated result to the chain. It serves as a critical coordination point between operators and the on-chain smart contracts.

## What an Aggregator does for EigenLayer

An Aggregator in EigenLayer performs several essential functions:

1. **Task Management**: Monitors the blockchain for new tasks created in the AVS
2. **Response Collection**: Operates an RPC server to receive signed responses from operators
3. **BLS Signature Aggregation**: Combines multiple signatures into a single aggregated signature
4. **Quorum Verification**: Ensures that enough operators (meeting the quorum threshold) have signed responses
5. **On-chain Submission**: Submits the aggregated response and signature to the blockchain
6. **Non-signer Tracking**: Identifies operators who did not participate in signing responses

## How the Logic Works

The Aggregator operates through three main asynchronous processes:

1. **RPC Server Process**:
   - Runs a TARPC-based server that listens for incoming operator responses
   - When an operator submits a signed task response, it validates and stores it
   - Forwards the signature to the BLS aggregation service for accumulation

2. **Task Monitoring Process**:
   - Subscribes to blockchain events for new tasks using WebSocket connections
   - When a new task is detected, it creates a task metadata record
   - Initializes the BLS aggregation service to start collecting signatures for the task

3. **Aggregation Process**:
   - Listens for aggregated results from the BLS aggregation service
   - When enough signatures are collected (meeting the quorum threshold), processes the result
   - Submits the aggregated signature along with information about non-signing operators to the blockchain

## Key Components

The Aggregator consists of several key components that work together:

1. **Task Processor**: Manages the lifecycle of tasks, from creation to completion
   - `IndexingTaskProcessor` is the standard implementation provided by the SDK
   - It tracks tasks with timeout durations and window durations for processing

2. **BLS Aggregation Service**: Handles the cryptographic combining of signatures
   - Maintains the state of collected signatures for each task
   - Determines when a quorum has been reached for task completion

3. **Operator Info Service**: Retrieves and manages operator information
   - Tracks which operators are registered and their state
   - Provides operator BLS public keys for signature verification

4. **AVS Registry Service**: Interacts with the on-chain registry
   - Verifies operator registrations
   - Manages quorum assignments and membership

## How to Set Up an Aggregator

Here's how to build an Aggregator based on the examples:

```rust
use alloy::primitives::Address;
use eigensdk::{
    aggregator::{task_processor::IndexingTaskProcessor, Aggregator, AggregatorConfig},
    common::get_signer,
};
use std::{str::FromStr, time::Duration};
use your_avs_module::YourTaskManagerInstance;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Set up connection parameters
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let wallet = get_signer("YOUR_PRIVATE_KEY", &http_rpc_url);
    let aggregator_server_address = "127.0.0.1:8080".to_string();
    
    // 2. Set contract addresses
    let task_manager_address = Address::from_str("YOUR_TASK_MANAGER_ADDRESS")?;
    let registry_coordinator_address = Address::from_str("YOUR_REGISTRY_COORDINATOR_ADDRESS")?;
    let operator_state_retriever_address = Address::from_str("YOUR_OPERATOR_STATE_RETRIEVER_ADDRESS")?;
    
    // 3. Initialize the task manager contract instance
    let contract = YourTaskManagerInstance::new(task_manager_address, wallet);
    
    // 4. Create the task processor with appropriate timeouts
    let task_processor = IndexingTaskProcessor::new(
        contract,
        Duration::from_secs(60),  // Task timeout - how long tasks are kept active
        Duration::from_secs(15)   // Task window - how long completed tasks are kept
    );
    
    // 5. Configure and initialize the aggregator
    let config = AggregatorConfig {
        server_address: aggregator_server_address,
        http_rpc_url,
        ws_rpc_url,
        registry_coordinator: registry_coordinator_address,
        operator_state_retriever: operator_state_retriever_address,
    };
    
    // 6. Create and start the aggregator
    let aggregator = Aggregator::new(config, task_processor).await?;
    aggregator.start().await?;
    
    Ok(())
}
```

## Security and Performance Considerations

When deploying an Aggregator for production use, consider these important factors:

1. **High Availability**: The Aggregator is a critical service that must be available to collect operator responses; consider redundancy and failover mechanisms.

2. **Network Connectivity**: Ensure reliable connections to both blockchain nodes and operator services.

3. **Resource Scaling**: The Aggregator needs to handle concurrent connections from potentially many operators.

4. **Signature Verification**: BLS signature verification is computationally intensive; ensure adequate CPU resources.

5. **Key Management**: Properly secure any private keys used by the Aggregator for submitting transactions.

6. **Task Timeouts**: Configure appropriate task timeout values based on the complexity of tasks and expected operator response times.

7. **Gas Management**: Ensure sufficient funds for submitting aggregated responses to the blockchain, particularly during high gas price periods.

The Aggregator plays a central role in the EigenLayer ecosystem by efficiently collecting and combining operator responses, enabling scalable distributed computation with cryptographic security.

