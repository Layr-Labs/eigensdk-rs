PHONY: reset-anvil

__CONTRACTS__: ##

start-anvil-chain-with-contracts-deployed: ##
	./crates/contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh

deploy-eigenlayer:
	./crates/contracts/anvil/deploy-eigenlayer.sh

deploy-avs:
	chmod +x ./crates/contracts/anvil/deploy-avs.sh
	./crates/contracts/anvil/deploy-avs.sh

dump-state:
	chmod +x ./crates/contracts/anvil/dump-state.sh
	./crates/contracts/anvil/dump-state.sh

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

start-anvil: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed
