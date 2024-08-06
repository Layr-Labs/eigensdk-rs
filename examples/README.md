# Examples

Examples demonstrating how to interact with Eigen layer contracts using eigen-rs crates.

## OperatorsInfo
```
cargo run -p info-operator-service
```

## AvsRegistry Read
```
cargo run --example get_operator-from_id
```

## Anvil utils(This requires a local anvil instance running using docker )
```
make start-anvil-chain-with-contracts-deployed
make start-anvil
cargo run --example get_contracts_from_registry
```

