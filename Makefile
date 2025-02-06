__CONTRACTS__: ##

.PHONY: deploy-eigenlayer
deploy-eigenlayer:
	./crates/operator_sets_contracts/anvil/deploy-eigenlayer.sh

.PHONY: deploy-avs
deploy-avs:
	./crates/operator_sets_contracts/anvil/deploy-avs.sh

deploy-m2-eigenlayer:
	./crates/m2_contracts/anvil/deploy-eigenlayer.sh

deploy-m2-avs:
	./crates/m2_contracts/anvil/deploy-avs.sh

.PHONY: dump-m2-state
dump-m2-state:
	./crates/m2_contracts/anvil/dump-state.sh

.PHONY: dump-slashing-state
dump-slashing-state:
	./crates/operator_sets_contracts/anvil/dump-state.sh

.PHONY: dump-state
dump-state: dump-m2-state dump-slashing-state

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

.PHONY: bindings_rewardsv2_host
bindings_rewardsv2_host:
	@echo "Generating bindings..."
	./scripts/generate_rewardsv2_bindings.sh
	cargo fmt --all
	@echo "Bindings generated"

.PHONY: bindings_slashing_host
bindings_slashing_host:
	@echo "Generating bindings..."
	./scripts/generate_slashing_bindings.sh
	cargo fmt --all
	# Apply a fix for any compile issues
	git apply --allow-empty scripts/bindings.patch
	@echo "Bindings generated"

.PHONY: bindings_host
bindings_host: bindings_rewardsv2_host bindings_slashing_host
.PHONY: rewardsv2-bindings
rewardsv2-bindings:
	@echo "Starting Docker container..."
	@docker run --rm -v "$(PWD):/sdk" -w "/sdk" \
		ghcr.io/foundry-rs/foundry:v0.3.0 \
		-c scripts/generate_rewardsv2_bindings.sh
	cargo fmt --all

.PHONY: slashing-bindings
slashing-bindings:
	@echo "Starting Docker container..."
	@docker run --rm -v "$(PWD):/sdk" -w "/sdk" \
		ghcr.io/foundry-rs/foundry:v0.3.0 \
		-c scripts/generate_slashing_bindings.sh
	cargo fmt --all
	# Apply a fix for any compile issues
	git apply --allow-empty scripts/bindings.patch

.PHONY: bindings
bindings: rewardsv2-bindings slashing-bindings