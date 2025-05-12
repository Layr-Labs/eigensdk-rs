PHONY: deploy-el-and-avs-contracts
PHONY: reset-anvil

deploy-avs:
	./contracts/anvil/deploy-avs.sh

deploy-eigenlayer:
	./contracts/anvil/deploy-eigenlayer.sh

deploy-uam-permissions:
	./contracts/anvil/uam-permissions.sh

deploy-el-and-avs-contracts: deploy-eigenlayer deploy-avs deploy-uam-permissions

__TESTING__: ##

reset_anvil:
	-docker stop anvil
	-docker rm anvil 

start_docker:
	$(MAKE) reset_anvil
	docker run -d --name anvil -p 8545:8545 --entrypoint anvil \
		ghcr.io/foundry-rs/foundry:latest --host 0.0.0.0
	sleep 2

RUST_BINDINGS_PATH:=src/bindings

generate-bindings:
	cd contracts && forge build --force --skip test --skip script
	rm -rf ${RUST_BINDINGS_PATH}
	forge bind --alloy --skip-build --overwrite --module \
		--root contracts/  \
		--bindings-path ${RUST_BINDINGS_PATH} \
		--select '^AwesomeVaultTaskManager$$' \
		--select '^AwesomeVaultServiceManager$$'
