use ark_ff::BigInteger256;
use eigensdk_contracts_bindings::{
    AVSDirectory::{self, avs_directory},
    DelegationManager::{self, delegation_manager, OperatorDetails},
    ISlasher::{self, i_slasher},
    IStrategy::{self, i_strategy},
    StrategyManager::{self, strategy_manager},
    IERC20::{self, ierc20},
};
use eigensdk_logging::logger::Logger;
use eigensdk_types::operator::Operator;
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

    pub async fn get_operator_shares_in_strategy(
        &self,
        operator_addr: Address,
        strategy_addr: Address,
    ) {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let operator_shares_in_strategy = contract_delegation_manager
            .operator_shares(operator_addr, strategy_addr)
            .call()
            .await
            .unwrap();
    }

    pub async fn operator_is_frozen(&self, operator_addr: Address) -> Result<bool, String> {
        let provider = Arc::new(self.client.clone());

        let contract_slasher = i_slasher::ISlasher::new(self.slasher, provider);

        let operator_is_frozen = contract_slasher
            .is_frozen(operator_addr)
            .call()
            .await
            .unwrap();

        Ok(operator_is_frozen)
    }

    pub async fn service_manager_can_slash_operator_until_block(
        &self,
        operator_addr: Address,
        service_manager_addr: Address,
    ) {
        let provider = Arc::new(self.client.clone());

        let contract_slasher = i_slasher::ISlasher::new(self.slasher, provider);

        let service_manager_can_slash_operator_until_block = contract_slasher
            .contract_can_slash_operator_until_block(operator_addr, service_manager_addr)
            .call()
            .await
            .unwrap();
    }

    pub async fn get_strategy_and_underlying_erc20_token(
        &self,
        strategy_addr: Address,
    ) -> Result<(Address, Address, Address), String> {
        let provider = Arc::new(self.client.clone());
        let contract_strategy = i_strategy::IStrategy::new(strategy_addr, provider.clone());
        let underlying_token_addr = contract_strategy.underlying_token().call().await.unwrap();

        let contract_ierc20 = ierc20::IERC20::new(underlying_token_addr, provider);

        return Ok((
            strategy_addr,
            underlying_token_addr,
            contract_ierc20.address(),
        ));
    }

    pub async fn get_operator_details(&self, operator: Address) -> Result<Operator, String> {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let operator_details: OperatorDetails = contract_delegation_manager
            .operator_details(operator)
            .call()
            .await
            .unwrap();

        return Ok(Operator::new(
            operator,
            operator_details.earnings_receiver,
            operator_details.delegation_approver,
            operator_details.staker_opt_out_window_blocks,
            None,
        ));
    }

    pub async fn is_operator_registered(&self, operator: Address) -> Result<bool, String> {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let is_operator = contract_delegation_manager
            .is_operator(operator)
            .call()
            .await
            .unwrap();

        Ok(is_operator)
    }
}
