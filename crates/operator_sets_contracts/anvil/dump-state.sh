#!/bin/bash

# Enable the script to exit immediately if a command exits with a non-zero status
set -o errexit -o nounset -o pipefail

# Navigate to the script directory
parent_path=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
cd "$parent_path"

root_dir=$(realpath "$parent_path/../..")

set -a
source $parent_path/utils.sh
# we overwrite some variables here because should always deploy to anvil (localhost)
RPC_URL=http://localhost:8545
DEPLOYER_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
set +a


# CHAIN_ID=$(cast chain-id --rpc-url $RPC_URL)

# # start an empty anvil chain in the background and dump its state to a json file upon exit
start_anvil_docker "" $parent_path/operatorset_contracts_deployed_anvil_state

# Deploy Contracts
cd "$root_dir/operator_sets_contracts"
forge create src/ContractsRegistry.sol:ContractsRegistry --rpc-url $RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast
forge script script/DeployEigenLayerCore.s.sol:DeployEigenlayerCore --rpc-url $RPC_URL --broadcast --slow -vvv

cd "$root_dir/operator_sets_contracts"
forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --slow -vvv
forge script script/ContractsRegistry.s.sol --rpc-url $RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --slow
