# Incredible Dot Product example

This example proposes a more complex AVS than the proposed incredible squaring. The input type handled is a struct that contains two vectors, representing two points, that should be processed making the dot product, being the output type a `U256`, representing the result of the product. In this sense, the task for the operators to complete is executing the dot product between the two points, and returning the result as the response value submitted to the Task Manager on-chain contract.

## Structure

### Types

The task type of the solidity contract is the following:

``` solidity
    struct DotProductInput {
        uint256[] X;
        uint256[] Y;
    }

    struct Task {
        DotProductInput pointsToMultiply;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
```

The input is the pair of vectors X and Y, both represented by an `U256` array.

The task response type is:

``` solidity
    struct TaskResponse {
        uint32 referenceTaskIndex;
        uint256 result;
    }
```

The `result` field represents the result of the dot product operation between the two received points.

### Specific business logic

The challenger and operator needs a struct that implements the `ResponseCalculator` trait. This struct is in charge of computing the response of a task. In the example, we are using the `FunctionResponseCalculator` struct which is provided by the SDK and receives a function for calculating the logic.

In the specific case of the dot product, the function receives an input with two vectors, and performs the dot product of them, returning the result.

To create the sequence that passes input values to the task spammer, we use the sequence generator in the task manager main (in `examples/incredible-dot-product/src/bin/task-spammer.rs`), which creates a sequence that on each iteration advances on 1 and gives as input a fixed pair of vectors defined from the iteration number.

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

The operator will produce invalid results often because it use `failing_response_calculator` method, which has a failure rate of 40% and returns `U256::MAX` as the response.

These failures result in slashing once they're challenged. To see this in action, start the challenger with:

```bash
cargo run --bin challenger
```

To start the cycle, start the task spammer:

``` bash
cargo run --bin task-spammer
```
