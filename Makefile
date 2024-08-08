__CONTRACTS__: ##

start-anvil-chain-with-contracts-deployed: ##
	./crates/contracts/anvil/start-anvil-chain-with-el-and-avs-deployed.sh

deploy-contracts-to-anvil-and-save-state: ## 
	./crates/contracts/anvil/deploy-contracts-save-anvil-state.sh

start-anvil: ##
	./crates/contracts/anvil/start-anvil.sh

stop-anvil: ##
	./crates/contracts/anvil/stop-anvil.sh

__TESTING__: ##

pr: ## 
	$(MAKE) start-anvil-chain-with-contracts-deployed
	$(MAKE) start-anvil
	cargo test --workspace
	cargo clippy --workspace --lib --examples --tests --benches --all-features
	cargo +nightly fmt -- --check
	$(MAKE) stop-anvil

fireblocks-tests:
	$(MAKE) start-anvil-chain-with-contracts-deployed
	$(MAKE) start-anvil
	cargo test --workspace --features fireblock-tests