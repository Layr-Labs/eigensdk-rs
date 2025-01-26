#!/bin/bash

# Enable the script to exit immediately if a command exits with a non-zero status
set -o errexit -o nounset -o pipefail

# cd to the directory of this script so that this can be run from anywhere
anvil_dir=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
root_dir=$(realpath $anvil_dir/../..)

set -a
source $anvil_dir/utils.sh
# we overwrite some variables here because should always deploy to anvil (localhost)
RPC_URL=http://localhost:8545
DEPLOYER_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
set +a

# # start an empty anvil chain in the background and dump its state to a json file upon exit
start_anvil_docker "" $anvil_dir/contracts_deployed_anvil_state.json
# sleep 1

CHAIN_ID=$(cast chain-id)

# Deploy Contracts
cd "$root_dir/contracts"
forge create src/ContractsRegistry.sol:ContractsRegistry --rpc-url $RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast
forge script script/DeployEigenLayerCore.s.sol:DeployEigenlayerCore --rpc-url $RPC_URL --broadcast --slow

cd "$root_dir/contracts"
forge script script/DeployMockAvs.s.sol --rpc-url $RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --slow -vvv
forge script script/ContractsRegistry.s.sol --rpc-url $RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --slow