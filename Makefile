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
	cargo +nightly fmt -- --check
	docker stop anvil

fireblocks-tests:
	$(MAKE) start-anvil-chain-with-contracts-deployed > /dev/null &
	cargo test --workspace --features fireblock-tests

clippy:
	cargo clippy --workspace --all-features  --tests --examples -- -D warnings

start-anvil: reset-anvil ##
			 $(MAKE) start-anvil-chain-with-contracts-deployed
			 docker start anvil
