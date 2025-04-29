#!/bin/bash

SENDER_ADDR=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd "$parent_path"

cd ../
forge script script/SetupDistributions.s.sol:SetupDistributions --rpc-url http://localhost:8545 \
    --broadcast -v --sender $SENDER_ADDR

forge script script/SetupDistributions.s.sol:SetupDistributions --rpc-url http://localhost:8545 \
   --broadcast --sig "executeProcessClaim()" -v --sender $SENDER_ADDR
