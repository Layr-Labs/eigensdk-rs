# eigen-rs

Rust <> EigenLayer developer tools

![360_F_303452599_eZMGXe7awggqAHTQXpjzBFehJBEyw4QR-overlay](https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5)

## Overview

List of crates in the repository :-

- [eigen-client-avsregistry](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/clients/avsregistry) - Read, Write and subscribe methods for AvsRegistry
- [eigen-client-elcontracts](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/clients/elcontracts) - Convenience methods to call Eigenlayer contracts
- [eigen-chainio-utils](https://github.com/supernovahs/eigen-rs/tree/main/crates/chainio/utils)
- [eigen-contracts-bindings](https://github.com/supernovahs/eigen-rs/tree/main/crates/contracts/bindings) - Generate ethers bindings for Eigen Layer.
- [eigen-crypto-bls](https://github.com/supernovahs/eigen-rs/tree/main/crates/crypto/bls) - bls utilities
- [eigen-crypto-bn254](https://github.com/supernovahs/eigen-rs/tree/main/crates/crypto/bn254) - bn254 utilities
- [eigen-metrics](https://github.com/supernovahs/eigen-rs/tree/main/crates/metrics) - performance , rpc and economic metrics 
- [eigen-services](https://github.com/supernovahs/eigen-rs/tree/main/crates/services) - Spawn tokio services for operators info , bls aggregation
- [eigen-types](https://github.com/supernovahs/eigen-rs/tree/main/crates/types) - Common types
- [eigen-utils](https://github.com/supernovahs/eigen-rs/tree/main/crates/utils) - Publicly exportable `m2-mainnet` compatible alloy bindings. 
- [eigen-testing-utils](https://github.com/supernovahs/eigen-rs/tree/main/testing/testing-utils) - Contains publicly exportable  anvil , holesky , mainnet addresses for eigen contracts .

## Examples

You can run any [example](https://github.com/supernovahs/eigen-rs/tree/main/examples) using the command cargo run --example <example-name>

Example : 
```
cargo run --example get_quorum_count
```

## Contributor Guidelines

We are actively looking for contributors. Thank you for your interest. We have strict ci checks in place. In case of any questions and support , feel free to raise an issue.

To test locally and raise a PR :- 

You need `foundry` , `docker` and `make`  and `nightly rust` to successfully run it.
```
make pr 
```

## Supported Rust Version

Rolling `MSRV` policy of 6 months. The current `MSRV` is 1.79

## Disclaimer

This software is `unaudited`.This is experimental software and is provided on an "as is" and "as available" basis and may not work at all. It should not be used in production.

# Credits

- [Layer-labs](https://github.com/Layr-Labs/eigensdk-go/tree/master)

## Contact

Telegram - supernovahs444
