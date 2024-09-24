# Eigen Testing Utils

The `eigen-testing-utils` crate provides a set of utilities for testing Ethereum-based contracts and services within the Eigen ecosystem. This crate is designed to simplify interactions with local Anvil instances and Holesky/Mainnet environments by providing constant addresses and helper functions to retrieve important contract addresses.

## Anvil Utilities

### Anvil Constants

Provides utilities for interacting with local Anvil instances. Key components include:

- CONTRACTS_REGISTRY: The address of the local Anvil Contracts Registry.
- ANVIL_RPC_URL: A static instance of the local Anvil RPC URL configured with required fillers.

### Key Functions

- get_service_manager_address(): Retrieves the address of the service manager contract.
- get_registry_coordinator_address(): Retrieves the address of the registry coordinator contract.
- get_operator_state_retriever_address(): Retrieves the address of the operator state retriever contract.
- get_delegation_manager_address(): Retrieves the address of the delegation manager contract.
- get_strategy_manager_address(): Retrieves the address of the strategy manager contract.
- get_erc20_mock_strategy(): Retrieves the address of the ERC20 mock strategy contract.
- get_proxy_admin(): Retrieves the address of the proxy admin contract.

## Holesky Constants

Contains predefined addresses for various contracts on the Holesky test network.

### Holesky Key Addresses

- DELEGATION_MANAGER_ADDRESS: Address of the delegation manager.
- STRATEGY_MANAGER_ADDRESS: Address of the strategy manager.
- EIGENPOD_MANAGER_ADDRESS: Address of the Eigenpod manager.
- AVS_DIRECTORY_ADDRESS: Address of the AVS directory.
- SLASHER_ADDRESS: Address of the slasher.
- REWARDS_COORDINATOR: Address of the rewards coordinator.
And various strategy base addresses.

## Mainnet Constants

Contains predefined addresses for various contracts on the Ethereum mainnet.

### Mainnet Key Addresses

- DELEGATION_MANAGER_ADDRESS: Address of the delegation manager.
- STRATEGY_MANAGER_ADDRESS: Address of the strategy manager.
- EIGENPOD_MANAGER_ADDRESS: Address of the Eigenpod manager.
- AVS_DIRECTORY_ADDRESS: Address of the AVS directory.
- SLASHER_ADDRESS: Address of the slasher.
- StrategyBase_cbETH: Address of the cbETH strategy base.
- StrategyBase_stETH: Address of the stETH strategy base.
And various other strategy base addresses.

## Example

- [get_contracts_from_registry](https://github.com/Layr-Labs/eigensdk-rs/blob/main/examples/anvil-utils/examples/get_contracts_from_registry.rs)
