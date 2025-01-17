#!/bin/bash

RPC_URL=http://localhost:8545

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd "$parent_path"

set -a
source ./utils.sh
set +a

# start an anvil instance in the background that has eigenlayer contracts deployed
# we start anvil in the background so that we can run the below script
# anvil --load-state avs-and-eigenlayer-deployed-anvil-state.json &
start_anvil_docker $parent_path/contracts_deployed_anvil_state.json ""

cd ../../contracts
# we need to restart the anvil chain at the correct block, otherwise the indexRegistry has a quorumUpdate at the block number
# at which it was deployed (aka quorum was created/updated), but when we start anvil by loading state file it starts at block number 0
# so calling getOperatorListAtBlockNumber reverts because it thinks there are no quorums registered at block 0
# advancing chain manually like this is a current hack until https://github.com/foundry-rs/foundry/issues/6679 is merged
cast rpc anvil_mine 200 --rpc-url http://localhost:8545 > /dev/null
echo "Anvil is ready. Advanced chain to block-number:" $(cast block-number)

docker attach anvil
