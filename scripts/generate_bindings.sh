#!/usr/bin/env sh

# Exit if any command fails
set +e

# Print each command executed (useful for debugging)
# set -o xtrace

# Turns the contract names into flags for the `forge bind` command
generate_flags() {
    for contract in $@; do
        echo "$acc --select ^$contract\$"
    done
}

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "$0")"
    pwd -P
)
repo_root=$parent_path/..
cd $repo_root

# Load variables from the bindings_vars.sh file
source scripts/bindings_vars.sh

SDK_CONTRACTS_ARGS=$(generate_flags $SDK_CONTRACTS)
MIDDLEWARE_CONTRACTS_ARGS=$(generate_flags $MIDDLEWARE_CONTRACTS)
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
