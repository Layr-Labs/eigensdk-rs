# EigenSDK-rs

EigenSDK-rs is an initiative for rust developers to build AVSs on eigenlayer.

## Installation

 ```bash
cargo add eigensdk --features full
```

## Overview

 List of crates in the repository :-

- [eigen-client-avsregistry](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/chainio/clients/avsregistry) - Read, Write and subscribe methods for AvsRegistry
- [eigen-client-elcontracts](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/chainio/clients/elcontracts) - Convenience methods to call Eigenlayer contracts
- [eigen-crypto-bls](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/crypto/bls) - New bls key pair, sign message, conversion utilites between alloy and arkworks bn254.
- [eigen-crypto-bn254](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/crypto/bn254) - verify message on G2, map to curve.
- [eigen-metrics](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/metrics) - performance, rpc and economic metrics
- [eigen-services](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/services) - Spawn tokio services for operators info, bls aggregation
- [eigen-types](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/types) - Common types
- [eigen-utils](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/utils) - Publicly exportable `mainnet rewards v0.4.3` compatible alloy bindings.
- [eigen-testing-utils](https://github.com/Layr-Labs/eigensdk-rs/tree/main/testing/testing-utils) - Contains publicly exportable anvil, holesky, mainnet addresses for eigen contracts.
- [eigen-cli](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/eigen-cli) - ECDSA, BLS keystore cli
- [eigen-nodeapi](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/nodeapi) - NodeApi implementation for EigenLayer.
- [eigen-logging](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/logging) - Logging utilities
- [eigen-common](https://github.com/Layr-Labs/eigensdk-rs/tree/main/crates/common) - Common utilities like provider and signer getters.

## Examples

You can run any [example](https://github.com/Layr-Labs/eigensdk-rs/tree/main/examples) using the command `cargo run --example <example-name>`

Example :
  
```bash
cargo run --example get_quorum_count
```

## Contract Bindings

The main branch of this repo is intended to be syncronized with mainnet version of core contracts.

To update the bindings of this repo, run:

```bash
make bindings
```

This command will generate the bindings files in the folder: `crates/utils`.

**Important:** this command requires Docker installed and running since it uses a container to generate the bindings. On Mac, you may get an error similar to:

```text
v0.3.0: Pulling from foundry-rs/foundry
docker: no matching manifest for linux/arm64/v8 in the manifest list entries.
```

In that case, you'll need to pull the foundry image for the `amd64` platform with:

```bash
docker pull --platform linux/amd64/v8 ghcr.io/foundry-rs/foundry:v0.3.0
```

And then run `make bindings` again.

## Contributor Guidelines

We are actively looking for contributors. Thank you for your interest. We have strict ci checks in place. In case of any questions and support, feel free to raise an issue.

## Branches

- `main` - Points to the  latest **mainnet** release of contracts.
- `testnet` - Points to the latest **testnet** release of contracts.
- `dev` - Points to the latest **dev** branch of the contracts.

### PR

To test locally :-

You need `foundry` to successfully run it.

```bash
cargo test --workspace
```

On Mac, you might get an error similar to:

```bash
Error starting anvil container: Client(PullImage { descriptor: "ghcr.io/foundry-rs/foundry:latest", err: DockerStreamError { error: "no matching manifest for linux/arm64/v8 in the manifest list entries" } })
```

In that case, you'll need to pull the foundry image for the `amd64` platform with:

```bash
docker pull --platform linux/amd64/v8 ghcr.io/foundry-rs/foundry:latest
```

At least 1 `approving` review is required to merge the PR.
  
### lint

```bash
make lint
```

### To run fireblocks tests

Add the following variables to your env

- FIREBLOCKS_API_KEY
- FIREBLOCKS_PRIVATE_KEY_PATH
- FIREBLOCKS_API_URL

 and then run the following command

```bash
make fireblocks-tests

```

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

```sh
open target/llvm-cov/html/index.html
```

## Supported Rust Version

Rolling `MSRV` policy of 6 months. The current `MSRV` is 1.79

## Disclaimer

🚧 EigenSDK-rs is under active development and has not been audited. EigenSDK-rs is rapidly being upgraded, features may be added, removed or otherwise improved or modified and interfaces will have breaking changes. EigenSDK-rs should be used only for testing purposes and not in production. EigenSDK-rs is provided "as is" and Eigen Labs, Inc. does not guarantee its functionality or provide support for its use in production. 🚧

## Credits

- [eigensdk-go](https://github.com/Layr-Labs/eigensdk-go/tree/master)

## Security Bugs

Please report security vulnerabilities to <security@eigenlabs.org>. Do NOT report security bugs via Github Issues.

## Support

Join our [telegram](https://t.me/+0_kYjD7TTCRjMjZh) channel .
