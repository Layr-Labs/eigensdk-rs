# eigen-rs

Rust <> EigenLayer developer tools

![eigen-rs](https://github.com/user-attachments/assets/bf1d1090-db70-487a-a49a-40f727849251)

## Overview

List of crates in the repository :-

- [eigen-client-avsregistry](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/chainio/clients/avsregistry) - Read, Write and subscribe methods for AvsRegistry
- [eigen-client-elcontracts](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/chainio/clients/elcontracts) - Convenience methods to call Eigenlayer contracts
- [eigen-chainio-utils](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/chainio/utils)
- [eigen-contracts-bindings](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/contracts/bindings) - Generate ethers bindings for Eigen Layer.
- [eigen-crypto-bls](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/crypto/bls) - bls utilities
- [eigen-crypto-bn254](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/crypto/bn254) - bn254 utilities
- [eigen-metrics](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/metrics) - performance, rpc and economic metrics
- [eigen-services](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/services) - Spawn tokio services for operators info, bls aggregation
- [eigen-types](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/types) - Common types
- [eigen-utils](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/utils) - Publicly exportable `m2-mainnet` compatible alloy bindings.
- [eigen-testing-utils](https://github.com/Layr-Labs/eigensdk-rs/tree/main/testing/testing-utils) - Contains publicly exportable anvil, holesky, mainnet addresses for eigen contracts .

## Examples

You can run any [example](https://github.com/Layr-Labs/eigensdk-rs/tree/main/examples) using the command `cargo run --example <example-name>`

Example :
  
```bash
cargo run --example get_quorum_count
```

## Contributor Guidelines

We are actively looking for contributors. Thank you for your interest. We have strict ci checks in place. In case of any questions and support, feel free to raise an issue.

### PR

To test locally and raise a PR :-

You need `foundry` , `docker` and `make` and `nightly rust` to successfully run it. Also, the `docker` engine has to be running for all the tests to execute correctly.

```bash
make pr
```

At least 1 `approving` review is required to merge the PR.
  
### To run fireblocks tests

Add the following variables to your env

- FIREBLOCKS_API_KEY
- FIREBLOCKS_PRIVATE_KEY_PATH
- FIREBLOCKS_API_URL

 and then run the following command

```bash
make fireblocks-tests

```

Note: The tests were written using sandbox environment, you would need to modify the testing parameters according to your own asset id's, tx-id etc.

### Test Coverage

Test coverage should aim to be around 80%.

Here's how to generate test coverage reports:

Install llvm tools:

```bash
make deps
```

Run the tests with coverage instrumentations:

```bash
make coverage
```

Open the coverage html report in a web browser:

```bash
open target/llvm-cov/html/index.html
```

## Supported Rust Version

Rolling `MSRV` policy of 6 months. The current `MSRV` is 1.79

## Disclaimer

This software is `unaudited`.This is experimental software and is provided on an "as is" and "as available" basis and may not work at all. It should not be used in production.

## Credits

- [Layer-labs](https://github.com/Layr-Labs/eigensdk-go/tree/master)

## Security Bugs

Please report security vulnerabilities to <security@eigenlabs.org>. Do NOT report security bugs via Github Issues.

## Support

Join our [telegram](https://t.me/+0_kYjD7TTCRjMjZh) channel .
