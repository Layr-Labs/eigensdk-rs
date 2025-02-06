# #!/usr/bin/env sh

# # Exit if any command fails
# set +e

# # Print each command executed (useful for debugging)
# # set -o xtrace

# generate_flags() {
#     for contract in $@; do
#         echo "$acc --select ^$contract\$"
#     done
# }

# # cd to the directory of this script so that this can be run from anywhere
# parent_path=$(
#     cd "$(dirname "$0")"
#     pwd -P
# )
# repo_root=$parent_path/..
# cd $repo_root

# __BINDINGS__: ##

# ### SDK bindings ###
# SDK_CONTRACTS:="MockAvsServiceManager ContractsRegistry MockERC20"
# SDK_CONTRACTS_LOCATION:=crates/operator_sets_contracts/
# SDK_BINDINGS_PATH:=crates/utils/src/slashing/sdk
# # The echo is to remove quotes, and the patsubst to make the regex match the full text only
# SDK_CONTRACTS_ARGS:=$(generate_flags $SDK_CONTRACTS)


# ### Middleware bindings ###
# MIDDLEWARE_CONTRACTS:="IndexRegistry OperatorStateRetriever StakeRegistry BLSApkRegistry IBLSSignatureChecker ServiceManagerBase IERC20 SlashingRegistryCoordinator ISlashingRegistryCoordinator RegistryCoordinator"
# MIDDLEWARE_CONTRACTS_LOCATION:=$(SDK_CONTRACTS_LOCATION)/lib/eigenlayer-middleware
# MIDDLEWARE_BINDINGS_PATH:=crates/utils/src/slashing/middleware
# # The echo is to remove quotes, and the patsubst to make the regex match the full text only
# MIDDLEWARE_CONTRACTS_ARGS:=$(generate_flags $MIDDLEWARE_CONTRACTS)


# ### Core bindings ###
# CORE_CONTRACTS:="DelegationManager IRewardsCoordinator StrategyManager IEigenPod EigenPod IEigenPodManager EigenPodManager IStrategy AVSDirectory AllocationManager PermissionController"
# CORE_CONTRACTS_LOCATION:=$(MIDDLEWARE_CONTRACTS_LOCATION)/lib/eigenlayer-contracts
# CORE_BINDINGS_PATH:=crates/utils/src/slashing/core
# # The echo is to remove quotes, and the patsubst to make the regex match the full text only
# CORE_CONTRACTS_ARGS:=$(generate_flags $CORE_CONTRACTS)


# ### RewardsV2 Core bindings ###
# REWARDS_V2_SDK_CONTRACTS:="MockAvsServiceManager ContractsRegistry MockERC20"
# REWARDS_V2_SDK_CONTRACTS_LOCATION:=crates/m2_contracts/
# REWARDS_V2_SDK_BINDINGS_PATH:=crates/utils/src/rewardsv2/sdk
# REWARDS_V2_SDK_CONTRACTS_ARGS:=$(generate_flags $REWARDS_V2_SDK_CONTRACTS)

# ### RewardsV2 Middleware bindings ###
# REWARDS_V2_MIDDLEWARE_CONTRACTS:="RegistryCoordinator IndexRegistry OperatorStateRetriever StakeRegistry BLSApkRegistry IBLSSignatureChecker ServiceManagerBase IERC20"
# REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION:=$(REWARDS_V2_SDK_CONTRACTS_LOCATION)/lib/eigenlayer-middleware
# REWARDS_V2_MIDDLEWARE_BINDINGS_PATH:=crates/utils/src/rewardsv2/middleware
# # The echo is to remove quotes, and the patsubst to make the regex match the full text only
# REWARDS_V2_MIDDLEWARE_CONTRACTS_ARGS:=$(generate_flags $REWARDS_V2_MIDDLEWARE_CONTRACTS)

# # The echo is to remove quotes, and the patsubst to make the regex match the full text only
# REWARDS_V2_CORE_CONTRACTS:="DelegationManager IRewardsCoordinator RewardsCoordinator  StrategyManager IEigenPod EigenPod IEigenPodManager EigenPodManager IStrategy AVSDirectory AllocationManager PermissionController ERC20 Slasher ISlasher"
# REWARDS_V2_CORE_CONTRACTS_LOCATION:=$(REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION)/lib/eigenlayer-contracts
# REWARDS_V2_CORE_BINDINGS_PATH:=crates/utils/src/rewardsv2/core
# # The echo is to remove quotes, and the patsubst to make the regex match the full text only
# REWARDS_V2_CORE_CONTRACTS_ARGS:=$(generate_flags $REWARDS_V2_CORE_CONTRACTS)



# .PHONY: slashing-bindings
# slashing-bindings:
# 	@echo "Generating bindings..."
# 	# Fetch submoduless
# 	cd $(SDK_CONTRACTS_LOCATION) && forge install

# 	# Generate SDK bindings
# 	cd $(SDK_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
# 	forge bind --alloy --skip-build --bindings-path $(SDK_BINDINGS_PATH) --overwrite \
# 		--root $(SDK_CONTRACTS_LOCATION) --module \
# 		$(SDK_CONTRACTS_ARGS)

# 	# Generate middleware bindings
# 	cd $(MIDDLEWARE_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
# 	forge bind --alloy --skip-build --bindings-path $(MIDDLEWARE_BINDINGS_PATH) --overwrite \
# 		--root $(MIDDLEWARE_CONTRACTS_LOCATION) --module \
# 		$(MIDDLEWARE_CONTRACTS_ARGS)

# 	# Generate core bindings
# 	cd $(CORE_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
# 	forge bind --alloy --skip-build --bindings-path $(CORE_BINDINGS_PATH) --overwrite \
# 		--root $(CORE_CONTRACTS_LOCATION) --module \
# 		$(CORE_CONTRACTS_ARGS)

# 	@echo "Bindings generated"

# .PHONY: rewardsv2-bindings
# rewardsv2-bindings:
# 	@echo "Generating bindings..."
# 	# Fetch submoduless
# 	cd $(REWARDS_V2_SDK_CONTRACTS_LOCATION) && forge install

# 	# Generate SDK bindings
# 	cd $(REWARDS_V2_SDK_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
# 	forge bind --alloy --skip-build --bindings-path $(REWARDS_V2_SDK_BINDINGS_PATH) --overwrite \
# 		--root $(REWARDS_V2_SDK_CONTRACTS_LOCATION) --module \
# 		$(REWARDS_V2_SDK_CONTRACTS_ARGS)

# 	# Generate middleware bindings
# 	cd $(REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
# 	forge bind --alloy --skip-build --bindings-path $(REWARDS_V2_MIDDLEWARE_BINDINGS_PATH) --overwrite \
# 		--root $(REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION) --module \
# 		$(REWARDS_V2_MIDDLEWARE_CONTRACTS_ARGS)

# 	# Generate core bindings
# 	cd $(REWARDS_V2_CORE_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
# 	forge bind --alloy --skip-build --bindings-path $(REWARDS_V2_CORE_BINDINGS_PATH) --overwrite \
# 		--root $(REWARDS_V2_CORE_CONTRACTS_LOCATION) --module \
# 		$(REWARDS_V2_CORE_CONTRACTS_ARGS)

# 	@echo "Bindings generated"






#!/usr/bin/env sh

# Exit if any command fails
set +e

# Print each command executed (useful for debugging)
# set -o xtrace

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

### SDK bindings ###
REWARDS_V2_SDK_CONTRACTS="MockAvsServiceManager ContractsRegistry MockERC20"
REWARDS_V2_SDK_CONTRACTS_LOCATION=crates/m2_contracts
REWARDS_V2_SDK_BINDINGS_PATH=crates/utils/src/rewardsv2/sdk
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
REWARDS_V2_SDK_CONTRACTS_ARGS=$(generate_flags $REWARDS_V2_SDK_CONTRACTS)


### Middleware bindings ###
REWARDS_V2_MIDDLEWARE_CONTRACTS="RegistryCoordinator IndexRegistry OperatorStateRetriever StakeRegistry BLSApkRegistry IBLSSignatureChecker ServiceManagerBase IERC20 ECDSAStakeRegistry ECDSAServiceManagerBase"
REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION=$REWARDS_V2_SDK_CONTRACTS_LOCATION/lib/eigenlayer-middleware
REWARDS_V2_MIDDLEWARE_BINDINGS_PATH=crates/utils/src/rewardsv2/middleware
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
REWARDS_V2_MIDDLEWARE_CONTRACTS_ARGS=$(generate_flags $REWARDS_V2_MIDDLEWARE_CONTRACTS)


### Core bindings ###
REWARDS_V2_CORE_CONTRACTS="DelegationManager IRewardsCoordinator RewardsCoordinator  StrategyManager IEigenPod EigenPod IEigenPodManager EigenPodManager IStrategy AVSDirectory AllocationManager PermissionController ERC20 Slasher ISlasher"
REWARDS_V2_CORE_CONTRACTS_LOCATION=$REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION/lib/eigenlayer-contracts
REWARDS_V2_CORE_BINDINGS_PATH=crates/utils/src/rewardsv2/core
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
REWARDS_V2_CORE_CONTRACTS_ARGS=$(generate_flags $REWARDS_V2_CORE_CONTRACTS)

# Fetch submodules
cd $REWARDS_V2_SDK_CONTRACTS_LOCATION && forge install
cd $repo_root

# Empty the directories before generating the bindings, ignore any errors
rm $REWARDS_V2_SDK_BINDINGS_PATH/* || true
rm $REWARDS_V2_MIDDLEWARE_BINDINGS_PATH/* || true
rm $REWARDS_V2_CORE_BINDINGS_PATH/* || true

# Compile all contracts
cd $repo_root/$REWARDS_V2_SDK_CONTRACTS_LOCATION && forge build --force --skip test --skip script
cd $repo_root/$REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION && forge build --force --skip test --skip script
cd $repo_root/$REWARDS_V2_CORE_CONTRACTS_LOCATION && forge build --force --skip test --skip script

# Move back to repo root
cd $repo_root

# Generate SDK bindings
forge bind --alloy --skip-build --bindings-path $REWARDS_V2_SDK_BINDINGS_PATH --overwrite \
    --root $REWARDS_V2_SDK_CONTRACTS_LOCATION --module \
    $REWARDS_V2_SDK_CONTRACTS_ARGS

# Generate middleware bindings
forge bind --alloy --skip-build --bindings-path $REWARDS_V2_MIDDLEWARE_BINDINGS_PATH --overwrite \
    --root $REWARDS_V2_MIDDLEWARE_CONTRACTS_LOCATION --module \
    $REWARDS_V2_MIDDLEWARE_CONTRACTS_ARGS

# Generate core bindings
forge bind --alloy --skip-build --bindings-path $REWARDS_V2_CORE_BINDINGS_PATH --overwrite \
    --root $REWARDS_V2_CORE_CONTRACTS_LOCATION --module \
    $REWARDS_V2_CORE_CONTRACTS_ARGS


