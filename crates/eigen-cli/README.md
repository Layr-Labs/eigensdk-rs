# Eigen Cli

The `eigen-cli` crate provides the following utilities.

## egnaddrs
This tool is used to help debug and test eigenlayer/avs deployments and contract setups.

### Usage

Currently egnaddrs only supports deriving contract addresses starting from a registry-coordinator or service-manager address.
To test it using a local anvil instance, run:
```bash
$ make start-anvil
```

Then run the eigen-cli:
```bash
$ cargo run --package eigen-cli -- egnaddrs --registry-coordinator 0x9E545E3C0baAB3E08CdfD552C960A1050f373042
```

It then prints the following datastructure:
```json
{
  "avs": {
    "bls-pubkey-compendium (shared)": "0x322813Fd9A801c5507c9de605d63CEA4f2CE6c44",
    "bls-pubkey-registry": "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9",
    "index-registry": "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
    "registry-coordinator": "0x9E545E3C0baAB3E08CdfD552C960A1050f373042",
    "service-manager": "0xc3e53F4d16Ae77Db1c982e75a937B9f60FE63690",
    "stake-registry": "0x851356ae760d987E095750cCeb3bC6014560891C"
  },
  "eigenlayer": {
    "delegation-manager": "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9",
    "slasher": "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707",
    "strategy-manager": "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9"
  },
  "network": {
    "chain-id": "31337",
    "rpc-url": "http://localhost:8545"
  }
}
```

## egnkey
This tool is used to manage keys for AVS development purpose

Features:
- [Generate ecdsa or bls key in batches](#generate-ecdsa-or-bls-key-in-batches)

### Generate ecdsa or bls key in batches

To create in a random folder:
```bash
cargo run --package eigen-cli -- egnkey generate --key-type ecdsa --num-keys <num_key>
```

To create in specific folder:
```bash
cargo run --package eigen-cli -- egnkey generate --key-type ecdsa --num-keys <num_key> --output-dir <path_to_folder>
```

To create `ECDSA` and `BLS` keys in a random folder:
```bash
cargo run --package eigen-cli -- egnkey generate --key-type ecdsa --num-keys <num_key>
```
