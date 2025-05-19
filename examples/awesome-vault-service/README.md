# Awesome Vault Service example

This example shows how to create a more complex AVS based on the SDK structure. The AVS shown here is a local Redis version that allows storing key-value pairs in the Task Manager. In this sense, the task for the operators to complete is storing the pair into a local Merkle tree of pairs and recomputing the root hash of the tree, submitting it as the response value to the Task Manager on-chain contract.

## Structure

### Types

The task type of the solidity contract is the following:

``` solidity
    struct TaskInput {
        string key;
        string value;
    }

    struct Task {
        TaskInput input;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
```

The input is the key-value pair mentioned at first.

The task response type is:

``` solidity
    struct TaskResponse {
        uint32 referenceTaskIndex;
        bytes32 result;
    }
```

The `result` field represents the root hash of the stored Merkle tree.

### Specific business logic

The challenger and operator use the `VaultServiceResponseCalculator.compute_response()` method for computing responses. Unlike the other examples that use the struct `FunctionResponseCalculator` provided by the SDK, this example defines a custom struct that implements `ResponseCalculator`. This is because we need to maintain internal state to store values over time. You can see the specific implementation in `examples/awesome-vault-service/src/response_calculator.rs`.

The method inserts the key-value pair into the vaults array representing the Merkle tree. Then, computes the tree root hash and returns it as the task response value.

For response validation, there should be an additional check to verify that the operator has uploaded the key-value pair, but for that, proof telling the operator has set the value should be added to the challenge cycle.

To create the sequence that passes input values to the task spammer, we use the sequence generator in the task manager main (in `examples/awesome-vault-service/src/bin/task-spammer.rs`), that creates a sequence that on each iteration advances on 1 and gives as input a fixed key-pair defined from the iteration number.

## How to run

This simple session illustrates the basic flow of the AVS:

Start anvil in a separate terminal:

```bash
anvil
```

Deploy contracts, set UAM permissions, and create a quorum in a single command:

```bash
make deploy-el-and-avs-contracts
```

Start the aggregator:

```bash
cargo run --bin aggregator
```

Start the operator:

```bash
cargo run --bin operator
```

The Operator will first check whether it is already registered on EigenLayer. If not, it will attempt to register automatically. To enable registration, create an [OperatorRegistrationConfig] and include it in the [OperatorConfig] struct.

The operator will produce invalid results often because it use `failing_response_calculator` method, which has a failure rate of 50% and returns `B256::default()` as the response.

These failures result in slashing once they're challenged. To see this in action, start the challenger with:

```bash
cargo run --bin challenger
```

To start the cycle, start the task spammer:

``` bash
cargo run --bin task-spammer
```
