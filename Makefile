__CONTRACTS__: ##

.PHONY: start-anvil-chain-with-contracts-deployed
start-anvil-chain-with-contracts-deployed: ##
	./crates/contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh

.PHONY: deploy-eigenlayer
deploy-eigenlayer:
	./crates/contracts/anvil/deploy-eigenlayer.sh

.PHONY: deploy-avs
deploy-avs:
	./crates/contracts/anvil/deploy-avs.sh

.PHONY: deploy
deploy: deploy-eigenlayer deploy-avs ##

.PHONY: dump-state
dump-state:
	./crates/contracts/anvil/dump-state.sh

.PHONY: deploy-contracts-to-anvil-and-save-state
deploy-contracts-to-anvil-and-save-state: ##
	./crates/contracts/anvil/deploy-contracts-save-anvil-state.sh

__TESTING__: ##

.PHONY: start-anvil
start-anvil: reset-anvil ##
	$(MAKE) start-anvil-chain-with-contracts-deployed

.PHONY: reset-anvil
reset-anvil:
	-docker stop anvil
	-docker rm anvil

.PHONY: coverage
coverage:
	cargo llvm-cov --lcov --output-path lcov.info --workspace --features fireblock-tests
	cargo llvm-cov report --html

.PHONY: deps
deps:
	@if ! command -v cargo-llvm-cov &> /dev/null; then \
		cargo install cargo-llvm-cov; \
	fi

.PHONY: fireblocks-tests
fireblocks-tests:
	cargo test --package eigen-client-fireblocks --features fireblock-tests

.PHONY: lint
lint:
	cargo fmt --all -- --check \
		&& cargo clippy --workspace --all-features --benches --examples --tests -- -D warnings

__BINDINGS__: ##

.PHONY: bindings_host
bindings_host:
	@echo "Generating bindings..."
	./scripts/generate_bindings.sh
	cargo fmt --all
	# Apply a fix for any compile issues
	git apply --allow-empty scripts/bindings.patch
	@echo "Bindings generated"

.PHONY: bindings
bindings:
	@echo "Starting Docker container..."
	@docker run --rm -v "$(PWD):/sdk" -w "/sdk" \
		ghcr.io/foundry-rs/foundry:v0.3.0 \
		-c scripts/generate_bindings.sh
	cargo fmt --all
	# Apply a fix for any compile issues
	git apply --allow-empty scripts/bindings.patch
