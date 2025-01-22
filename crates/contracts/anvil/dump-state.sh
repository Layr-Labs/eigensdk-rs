#!/bin/bash

# Enable the script to exit immediately if a command exits with a non-zero status
set -o errexit -o nounset -o pipefail

# Define your cleanup function
clean_up() {
    echo "Executing cleanup function..."
    set +e
    pkill -f anvil

    # Check if the exit status is non-zero
    exit_status=$?
    if [ $exit_status -ne 0 ]; then
        echo "Script exited due to set -e on line $1 with command '$2'. Exit status: $exit_status"
    fi
}
# Use trap to call the clean_up function when the script exits
trap 'clean_up $LINENO "$BASH_COMMAND"' EXIT

# cd to the directory of this script so that this can be run from anywhere
anvil_dir=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
root_dir=$(realpath $anvil_dir/../..)

set -a
source $anvil_dir/utils.sh
# we overwrite some variables here because should always deploy to anvil (localhost)
ETH_HTTP_URL=http://localhost:8545
DEPLOYER_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
set +a

# start an empty anvil chain in the background and dump its state to a json file upon exit
start_anvil_docker "" $anvil_dir/dump_state.json
sleep 1
