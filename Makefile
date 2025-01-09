PHONY: reset-anvil

__CONTRACTS__: ##

start-anvil-chain-with-contracts-deployed: ##
	./crates/contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh

deploy-contracts-to-anvil-and-save-state: ##
	./crates/contracts/anvil/deploy-contracts-save-anvil-state.sh

__TESTING__: ##

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

bindings:
	@echo "Generating bindings..."
	cd crates/contracts
	forge build
	cd ../..
	forge bind --alloy --bindings-path crates/utils --overwrite --root crates/contracts
	# Restore the Cargo.toml file
	git restore crates/utils/Cargo.toml
	@echo "Bindings generated"

start-anvil: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed
