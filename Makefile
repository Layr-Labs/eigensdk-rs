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

pr: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed > /dev/null &
	sleep 4 # needed to wait for anvil setup to finish
	cargo test --workspace
	docker stop anvil

coverage: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed > /dev/null &
	sleep 4 # needed to wait for anvil setup to finish
	cargo llvm-cov --lcov --output-path lcov.info
	docker stop anvil

deps:
	@if ! command -v cargo-llvm-cov &> /dev/null; then \
		cargo install cargo-llvm-cov; \
	fi

fireblocks-tests:
	$(MAKE) start-anvil-chain-with-contracts-deployed > /dev/null &
	cargo test --workspace --features fireblock-tests

lint:
	cargo fmt --all -- --check \
		&& cargo clippy --workspace --all-features --benches --examples --tests -- -D warnings

start-anvil: reset-anvil ##
			 $(MAKE) start-anvil-chain-with-contracts-deployed
			 docker start anvil
