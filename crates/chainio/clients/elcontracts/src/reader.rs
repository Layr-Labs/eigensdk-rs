use ark_ff::BigInteger256;
use eigensdk_contracts_bindings::{
    AVSDirectory::{self, avs_directory},
    DelegationManager::{self, delegation_manager},
    ISlasher, StrategyManager,
};
use eigensdk_logging::logger::Logger;
use ethers_core::types::{Address, U256};
use ethers_providers::{Http, Provider};
use std::sync::Arc;
pub struct ELChainReader {
    logger: Logger,
    slasher: Address,
    delegation_manager: Address,
    strategy_manager: Address,
    avs_directory: Address,
    client: Provider<Http>,
}

pub struct ELReader {}

impl ELChainReader {
    fn new(
        slasher: Address,
        delegation_manager: Address,
        strategy_manager: Address,
        avs_directory: Address,
        logger: Logger,
        client: Provider<Http>,
    ) -> Self {
        ELChainReader {
            slasher,
            delegation_manager,
            strategy_manager,
            avs_directory,
            logger,
            client,
        }
    }

    pub async fn build(
        delegation_manager: Address,
        avs_directory: Address,
        logger: Logger,
        client: Provider<Http>,
    ) -> Self {
        let provider = Arc::new(client.clone());
        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(delegation_manager, provider);
        let slasher_addr = contract_delegation_manager.slasher().call().await.unwrap();

        let strategy_manager_addr = contract_delegation_manager
            .strategy_manager()
            .call()
            .await
            .unwrap();

        Self {
            avs_directory,
            slasher: slasher_addr,
            delegation_manager,
            strategy_manager: strategy_manager_addr,
            logger,
            client,
        }
    }

    pub async fn calculate_delegation_approval_digest_hash(
        &self,
        staker: Address,
        operator: Address,
        delegation_approver: Address,
        approve_salt: [u8; 32],
        expiry: U256,
    ) -> [u8; 32] {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        return contract_delegation_manager
            .calculate_delegation_approval_digest_hash(
                staker,
                operator,
                delegation_approver,
                approve_salt,
                expiry,
            )
            .call()
            .await
            .unwrap();
    }

    pub async fn calculate_operator_avs_registration_digest_hash(
        &self,
        operator: Address,
        avs: Address,
        salt: [u8; 32],
        expiry: U256,
    ) -> Result<[u8; 32], String> {
        let provider = Arc::new(self.client.clone());
        let contract_avs_directory = avs_directory::AVSDirectory::new(self.avs_directory, provider);

        return Ok(contract_avs_directory
            .calculate_operator_avs_registration_digest_hash(operator, avs, salt, expiry)
            .call()
            .await
            .unwrap());
    }
}
