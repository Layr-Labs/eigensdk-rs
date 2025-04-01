## Zeus Templates

This repo sets up template contracts for [Zeus](https://github.com/Layr-Labs/zeus), the staged web3 deployer + metadata management CLI.

## Usage

The `templates` directory contains all relevant abstract contracts for scripts to inherit.

An example import is `import {EOADeployer} from "zeus-templates/templates/EOADeployer.sol";`

### EOADeployer

A generic script with a `deploy(string memory)` entry function for setting up any number of consecutive deploys.

### MultisigBuilder

A generic script with an `execute(string memory)` entry function for setting up any number of arbitrary calls from a multisig. Uses the `MultiSendCallOnly` contract for batching multiple calls into one transaction.

### OpsTimelockBuilder

A specialized script for the Operations Multisig, allowing for calls to be written for queueing into the Timelock before being sent to the Executor Multisig. See [the multisig governance documentation](https://docs.eigenlayer.xyz/eigenlayer/security/multisig-governance) for more details on EigenLayer's onchain role structure.

## Install via Foundry

```shell
forge install Layr-labs/zeus-templates
```

Run `forge install` inside a relevant repo that Zeus is using as a $ZEUS_CONTRACTS repo (e.g. [eigenlayer-contracts](https://github.com/Layr-Labs/eigenlayer-contracts)).

