__CONTRACTS__: ##

start-anvil-chain-with-contracts-deployed: ##
	./crates/contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh

deploy-contracts-to-anvil-and-save-state: ##
	./crates/contracts/anvil/deploy-contracts-save-anvil-state.sh

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

.PHONY: bindings_host
bindings_host:
	@echo "Generating bindings..."
	./scripts/generate_bindings.sh
	@echo "Bindings generated"

.PHONY: bindings
bindings:
	@echo "Starting Docker container..."
	@docker run --rm -v "$(PWD):$(PWD)" -w "$(PWD)" \
		ghcr.io/foundry-rs/foundry:v0.3.0 \
		-c './scripts/generate_bindings.sh'
