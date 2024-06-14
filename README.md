# eigen-rs

Rust <> EigenLayer developer tools

![360_F_303452599_eZMGXe7awggqAHTQXpjzBFehJBEyw4QR-overlay](https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5)

## Overview

List of crates in the repository :-

- [eigen-client-avsregistry](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/clients/avsregistry) - Read, Write and subscribe methods for AvsRegistry
- [eigen-client-builder](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/clients/builder)
- [eigen-client-elcontracts](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/clients/builder) - Convenience methods to call Eigenlayer contracts
- [eigen-chainio-utils](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/utils)
- [eigen-contracts-bindings](https://github.com/supernovahs/eigen-rs/tree/main/crates/contracts/bindings) - Contains json files for eigen contracts with support to generate ethers bindings .
- [eigen-crypto-bls](https://github.com/supernovahs/eigen-rs/tree/main/crates/crypto/bls) - bls utilities
- [eigen-crypto-bn254](https://github.com/supernovahs/eigen-rs/tree/main/crates/crypto/bn254) - bn254 utilities
- [eigen-metrics](https://github.com/supernovahs/eigen-rs/tree/main/crates/metrics) - performance , rpc and economic metrics 
- [eigen-services](https://github.com/supernovahs/eigen-rs/tree/main/crates/services) - Spawn tokio services for operators info , bls aggregation
- [eigen-types](https://github.com/supernovahs/eigen-rs/tree/main/crates/types) - Common types

## Examples

You can run any [example](https://github.com/supernovahs/eigensdk-rs/tree/main/examples) using the command cargo run -p <example-name>

Example : 
```
cargo run -p info-operator-service

```

## Contributor Guidelines

We are actively looking for contributors. Thanks for your interest.

Ensure the following passes when raising a PR

- `cargo test --workspace --all-features`
- `cargo +nightly fmt -- --check`

## Supported Rust Version

Rolling `MSRV` policy of 6 months. THe current `MSRV` is 1.79

# Credits

- [Layer-labs](https://github.com/Layr-Labs/eigensdk-go/tree/master)

## Contact

DM on Telegram - supernovahs444
