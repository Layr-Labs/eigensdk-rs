#!/bin/bash

export RPC_URL=http://localhost:8546
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# Navigate to the script directory
parent_path=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
cd "$parent_path"

root_dir=$(realpath "$parent_path/../..")

# Deploy Contracts
cd "$root_dir/operator_sets_contracts"
forge create src/ContractsRegistry.sol:ContractsRegistry --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast
forge script script/DeployEigenLayerCore.s.sol:DeployEigenlayerCore --rpc-url $RPC_URL --broadcast --slow -vvv
