# Incredible Squaring example

This example is a basic proposal of AVS, where the input and output type are `U256` values, representing the number to be squared and the number squared. In this sense, the task for the operators to complete is squaring the received number, and returning the result of the operation as the response value submitted to the Task Manager on-chain contract.

## Structure

### Types

The task type of the solidity contract is the following:

``` solidity
    struct Task {
        uint256 numberToBeSquared;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
 }
```

The input is an `uint256` representing the number to be squared.

The task response type is:

``` solidity
    struct TaskResponse {
        uint32 referenceTaskIndex;
        uint256 numberSquared;
 }
```

The `numberSquared` field represents the result of the squaring operation with the received number to square.

### Specific business logic

The challenger and operator needs a struct that implements the `ResponseCalculator` trait. This struct is in charge of computing the response of a task. In the example, we are using the `FunctionResponseCalculator` struct which is provided by the SDK and receives a function for calculating the logic.

In the specific case of squaring, the function receives an input of an `U256` type, and performs the squaring of it, returning the result.

To create the sequence that passes input values to the task spammer, we use the sequence generator in the task manager main (in `examples/incredible-squaring/src/bin/task-spammer.rs`) which creates a sequence that advances on 1 and gives the iteration number as input on each iteration.

## How to run

This simple session illustrates the basic flow of the AVS:

Initialize the Middleware and Forge submodules:

```bash
git submodule update --init --recursive
```

Start anvil in a separate terminal:

```bash
anvil
```

Deploy contracts, set UAM permissions, and create a quorum in a single command:

```bash
deploy-el-and-avs-contracts
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

The operator will produce invalid results often because it use `failing_response_calculator` method, which has a failure rate of 60% and returns `U256::from(42)` as the response.

These failures result in slashing once they're challenged. To see this in action, start the challenger with:

```bash
cargo run --bin challenger
```

To start the cycle, start the task spammer:

``` bash
cargo run --bin task-spammer
```
