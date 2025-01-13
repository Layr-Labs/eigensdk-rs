PHONY: reset-anvil

__CONTRACTS__: ##

start-anvil-chain-with-contracts-deployed: ##
	./crates/contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh

deploy-eigenlayer:
	./crates/contracts/anvil/deploy-eigenlayer.sh

deploy-avs:
	./crates/contracts/anvil/deploy-avs.sh

deploy: deploy-eigenlayer deploy-avs ##

dump-state:
	./crates/contracts/anvil/dump-state.sh

__TESTING__: ##

start-anvil: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed

reset-anvil:
	-docker stop anvil
	-docker rm anvil

coverage:
	cargo llvm-cov --lcov --output-path lcov.info --workspace --features fireblock-tests
	cargo llvm-cov report --html

deps:
	@if ! command -v cargo-llvm-cov &> /dev/null; then \
		cargo install cargo-llvm-cov; \
	fi

fireblocks-tests:
	cargo test --package eigen-client-fireblocks --features fireblock-tests

lint:
	cargo fmt --all -- --check \
		&& cargo clippy --workspace --all-features --benches --examples --tests -- -D warnings

__BINDINGS__: ##

### SDK bindings ###
SDK_CONTRACTS:="MockAvsServiceManager ContractsRegistry"
SDK_CONTRACTS_LOCATION:=crates/contracts
SDK_BINDINGS_PATH:=crates/utils/src/sdk
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
SDK_CONTRACTS_ARGS:=$(patsubst %, --select '^%$$', $(shell echo $(SDK_CONTRACTS)))


### Middleware bindings ###
MIDDLEWARE_CONTRACTS:="RegistryCoordinator IndexRegistry OperatorStateRetriever StakeRegistry BLSApkRegistry IBLSSignatureChecker ServiceManagerBase IERC20"
MIDDLEWARE_CONTRACTS_LOCATION:=$(SDK_CONTRACTS_LOCATION)/lib/eigenlayer-middleware
MIDDLEWARE_BINDINGS_PATH:=crates/utils/src/middleware
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
MIDDLEWARE_CONTRACTS_ARGS:=$(patsubst %, --select '^%$$', $(shell echo $(MIDDLEWARE_CONTRACTS)))


### Core bindings ###
CORE_CONTRACTS:="DelegationManager IRewardsCoordinator StrategyManager IEigenPod EigenPod IEigenPodManager EigenPodManager IStrategy AVSDirectory AllocationManager PermissionController"
CORE_CONTRACTS_LOCATION:=$(MIDDLEWARE_CONTRACTS_LOCATION)/lib/eigenlayer-contracts
CORE_BINDINGS_PATH:=crates/utils/src/core
# The echo is to remove quotes, and the patsubst to make the regex match the full text only
CORE_CONTRACTS_ARGS:=$(patsubst %, --select '^%$$', $(shell echo $(CORE_CONTRACTS)))


.PHONY: bindings
bindings:
	@echo "Generating bindings..."
	# Fetch submoduless
	cd $(SDK_CONTRACTS_LOCATION) && forge install

	# Generate SDK bindings
	cd $(SDK_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
	forge bind --alloy --skip-build --bindings-path $(SDK_BINDINGS_PATH) --overwrite \
		--root $(SDK_CONTRACTS_LOCATION) --module \
		$(SDK_CONTRACTS_ARGS)

	# Generate middleware bindings
	cd $(MIDDLEWARE_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
	forge bind --alloy --skip-build --bindings-path $(MIDDLEWARE_BINDINGS_PATH) --overwrite \
		--root $(MIDDLEWARE_CONTRACTS_LOCATION) --module \
		$(MIDDLEWARE_CONTRACTS_ARGS)

	# Generate core bindings
	cd $(CORE_CONTRACTS_LOCATION) && forge build --force --skip test --skip script
	forge bind --alloy --skip-build --bindings-path $(CORE_BINDINGS_PATH) --overwrite \
		--root $(CORE_CONTRACTS_LOCATION) --module \
		$(CORE_CONTRACTS_ARGS)

	@echo "Bindings generated"
