#!/bin/bash

# Enable the script to exit immediately if a command exits with a non-zero status
set -o errexit -o nounset -o pipefail

# cd to the directory of this script so that this can be run from anywhere
anvil_dir=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd $anvil_dir

# Start anvil
./dump-state.sh >/dev/null &
anvil_pid=$!
sleep 3

# Deploy contracts
./deploy-eigenlayer.sh
./deploy-avs.sh

# Stop anvil
kill $anvil_pid
