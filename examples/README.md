# Examples

Examples demonstrating how to interact with Eigen layer contracts using eigen-rs crates.

## OperatorsInfo
```
cargo run -p info-operator-service
```

## AvsRegistry Read
```
cargo run -p examples-avsregistry-read
```

## Anvil utils(This requires a local anvil instance running using docker )
```
make start-anvil-chain-with-contracts-deployed
make start-anvil
cargo run -p examples-anvil-utils
```

