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

pr:
	$(MAKE) start-anvil > /dev/null &
	sleep 4 # needed to wait for anvil setup to finish
	cargo test --workspace
	docker stop anvil

run-coverage:
	$(MAKE) start-anvil > /dev/null &
	sleep 4 # needed to wait for anvil setup to finish
	cargo llvm-cov --html --ignore-filename-regex='fireblocks/' --workspace
	docker stop anvil
	cargo llvm-cov report --lcov > info.cov
	open target/llvm-cov/index.html

deps:
	@if ! command -v cargo-llvm-cov &> /dev/null; then \
		cargo install cargo-llvm-cov; \
	fi

fireblocks-tests:
	$(MAKE) start-anvil > /dev/null &
	cargo test --workspace --features fireblock-tests

lint:
	cargo fmt --all -- --check \
		&& cargo clippy --workspace --all-features --benches --examples --tests -- -D warnings

start-anvil: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed
