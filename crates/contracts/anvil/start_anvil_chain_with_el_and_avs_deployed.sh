#!/bin/bash

set -o errexit -o nounset -o pipefail

# cd to the directory of this script so that this can be run from anywhere
anvil_dir=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
root_dir=$(realpath $anvil_dir/../..)

set -a
source $anvil_dir/utils.sh
set +a

# start an anvil instance in the background that has eigenlayer contracts deployed
# we start anvil in the background so that we can run the below script
start_anvil_docker $anvil_dir/contracts-deployed-anvil-state.json ""

cd $root_dir/contracts
# we need to restart the anvil chain at the correct block, otherwise the indexRegistry has a quorumUpdate at the block number
# at which it was deployed (aka quorum was created/updated), but when we start anvil by loading state file it starts at block number 0
# so calling getOperatorListAtBlockNumber reverts because it thinks there are no quorums registered at block 0
# advancing chain manually like this is a current hack until https://github.com/foundry-rs/foundry/issues/6679 is merged
cast rpc anvil_mine 200 --rpc-url http://localhost:8545 > /dev/null
echo "Anvil is ready. Advanced chain to block-number:" $(cast block-number)


docker attach anvil