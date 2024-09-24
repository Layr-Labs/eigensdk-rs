# Examples

Examples demonstrating how to interact with Eigen layer contracts using eigen-rs crates.

## OperatorsInfo

Get all the operators registered at a block range and  get the operator info using an async channel.

```sh
cargo run -p info-operator-service 
```

## AvsRegistry Read

Example : Getting an operator address from the operator id

```sh
cargo run --example get_operator_from_id
```

## Anvil utils(This requires a local anvil instance running using docker )

Get EigenLayer contract addresses for local anvil testing . These are predetermined addresses that are stored in a mapping using these [scripts](https://github.com/Layr-Labs/eigensdk-rs/blob/d9b40d806b4939c64bb7d3df0f6f2a542499bd27/crates/contracts/script/DeployMockAvsRegistries.s.sol#L202).

```sh
make start-anvil-chain-with-contracts-deployed
make start-anvil
cargo run --example get_contracts_from_registry
```
