#!/usr/bin/env bash

set +e

function generate_flags() {
    acc=""
    for contract in $@; do
        acc="$acc --select '^$contract\$'"
    done
    echo $acc
}

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
repo_root=$parent_path/..
cd $repo_root

### SDK bindings ###
SDK_CONTRACTS="MockAvsServiceManager ContractsRegistry MockERC20"
SDK_CONTRACTS_LOCATION=crates/contracts
SDK_BINDINGS_PATH=crates/utils/src/sdk
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
SDK_CONTRACTS_ARGS=$(generate_flags $SDK_CONTRACTS)


### Middleware bindings ###
MIDDLEWARE_CONTRACTS="RegistryCoordinator IndexRegistry OperatorStateRetriever StakeRegistry BLSApkRegistry IBLSSignatureChecker ServiceManagerBase IERC20"
MIDDLEWARE_CONTRACTS_LOCATION=$SDK_CONTRACTS_LOCATION/lib/eigenlayer-middleware
MIDDLEWARE_BINDINGS_PATH=crates/utils/src/middleware
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
MIDDLEWARE_CONTRACTS_ARGS=$(generate_flags $MIDDLEWARE_CONTRACTS)


### Core bindings ###
CORE_CONTRACTS="DelegationManager IRewardsCoordinator RewardsCoordinator  StrategyManager IEigenPod EigenPod IEigenPodManager EigenPodManager IStrategy AVSDirectory AllocationManager PermissionController ERC20 Slasher ISlasher"
CORE_CONTRACTS_LOCATION=$MIDDLEWARE_CONTRACTS_LOCATION/lib/eigenlayer-contracts
CORE_BINDINGS_PATH=crates/utils/src/core
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
CORE_CONTRACTS_ARGS=$(generate_flags $CORE_CONTRACTS)

# Fetch submodules
cd $SDK_CONTRACTS_LOCATION && forge install
cd $repo_root

# Empty the directories before generating the bindings, ignore any errors
rm $SDK_BINDINGS_PATH/* || true
rm $MIDDLEWARE_BINDINGS_PATH/* || true
rm $CORE_BINDINGS_PATH/* || true

# Compile all contracts
cd $repo_root/$SDK_CONTRACTS_LOCATION && forge build --force --skip test --skip script
cd $repo_root/$MIDDLEWARE_CONTRACTS_LOCATION && forge build --force --skip test --skip script
cd $repo_root/$CORE_CONTRACTS_LOCATION && forge build --force --skip test --skip script

# Move back to repo root
cd $repo_root

# Generate SDK bindings
forge bind --alloy --skip-build --bindings-path $SDK_BINDINGS_PATH --overwrite \
    --root $SDK_CONTRACTS_LOCATION --module \
    $SDK_CONTRACTS_ARGS

# Generate middleware bindings
forge bind --alloy --skip-build --bindings-path $MIDDLEWARE_BINDINGS_PATH --overwrite \
    --root $MIDDLEWARE_CONTRACTS_LOCATION --module \
    $MIDDLEWARE_CONTRACTS_ARGS

# Generate core bindings
forge bind --alloy --skip-build --bindings-path $CORE_BINDINGS_PATH --overwrite \
    --root $CORE_CONTRACTS_LOCATION --module \
    $CORE_CONTRACTS_ARGS
