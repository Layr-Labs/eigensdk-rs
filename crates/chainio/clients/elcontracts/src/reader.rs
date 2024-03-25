use eigensdk_contracts_bindings::{
    AVSDirectory::avs_directory, DelegationManager::delegation_manager, ISlasher::i_slasher,
    IStrategy::i_strategy, IERC20::ierc20,
};
use eigensdk_types::operator::Operator;
use ethers_core::types::{Address, U256};
use ethers_providers::{Http, Provider};
use std::sync::Arc;

use crate::error::ElContractsError;

pub struct ELChainReader {
    slasher: Address,
    delegation_manager: Address,
    strategy_manager: Address,
    avs_directory: Address,
    client: Provider<Http>,
}

impl ELChainReader {
    fn new(
        slasher: Address,
        delegation_manager: Address,
        strategy_manager: Address,
        avs_directory: Address,
        client: Provider<Http>,
    ) -> Self {
        ELChainReader {
            slasher,
            delegation_manager,
            strategy_manager,
            avs_directory,
            client,
        }
    }

    pub async fn build(
        delegation_manager: Address,
        avs_directory: Address,
        client: Provider<Http>,
    ) -> Result<Self, ElContractsError> {
        let provider = Arc::new(client.clone());
        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(delegation_manager, provider);
        let slasher_addr_result = contract_delegation_manager.slasher().call().await;

        match slasher_addr_result {
            Ok(slasher_addr) => {
                let strategy_manager_addr_result =
                    contract_delegation_manager.strategy_manager().call().await;

                match strategy_manager_addr_result {
                    Ok(strategy_manager_addr) => Ok(Self {
                        avs_directory,
                        slasher: slasher_addr,
                        delegation_manager,
                        strategy_manager: strategy_manager_addr,
                        client,
                    }),
                    Err(_) => return Err(ElContractsError::GetStrategyManager),
                }
            }
            Err(_) => return Err(ElContractsError::GetSlasher),
        }
    }

    pub async fn calculate_delegation_approval_digest_hash(
        &self,
        staker: Address,
        operator: Address,
        delegation_approver: Address,
        approve_salt: [u8; 32],
        expiry: U256,
    ) -> Result<[u8; 32], ElContractsError> {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let delegation_approval_digest_hash_result = contract_delegation_manager
            .calculate_delegation_approval_digest_hash(
                staker,
                operator,
                delegation_approver,
                approve_salt,
                expiry,
            )
            .call()
            .await;

        match delegation_approval_digest_hash_result {
            Ok(delegation_approval_digest_hash) => Ok(delegation_approval_digest_hash),
            Err(_) => return Err(ElContractsError::GetDelegationApprovalDigestHash),
        }
    }

    pub async fn calculate_operator_avs_registration_digest_hash(
        &self,
        operator: Address,
        avs: Address,
        salt: [u8; 32],
        expiry: U256,
    ) -> Result<[u8; 32], ElContractsError> {
        let provider = Arc::new(self.client.clone());
        let contract_avs_directory = avs_directory::AVSDirectory::new(self.avs_directory, provider);

        let operator_avs_registration_digest_hash_result = contract_avs_directory
            .calculate_operator_avs_registration_digest_hash(operator, avs, salt, expiry)
            .call()
            .await;

        match operator_avs_registration_digest_hash_result {
            Ok(operator_avs_registration_digest_hash) => Ok(operator_avs_registration_digest_hash),
            Err(_) => return Err(ElContractsError::GetOperatorAvsRegistrationDigestHash),
        }
    }

    pub async fn get_operator_shares_in_strategy(
        &self,
        operator_addr: Address,
        strategy_addr: Address,
    ) -> Result<U256, ElContractsError> {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let operator_shares_in_strategy_result = contract_delegation_manager
            .operator_shares(operator_addr, strategy_addr)
            .call()
            .await;

        match operator_shares_in_strategy_result {
            Ok(operator_shares_in_strategy) => Ok(operator_shares_in_strategy),
            Err(_) => return Err(ElContractsError::GetOperatorShares),
        }
    }

    pub async fn operator_is_frozen(
        &self,
        operator_addr: Address,
    ) -> Result<bool, ElContractsError> {
        let provider = Arc::new(self.client.clone());

        let contract_slasher = i_slasher::ISlasher::new(self.slasher, provider);

        let operator_is_frozen_result = contract_slasher.is_frozen(operator_addr).call().await;

        match operator_is_frozen_result {
            Ok(operator_is_frozen) => Ok(operator_is_frozen),
            Err(_) => return Err(ElContractsError::IsFrozen),
        }
    }

    pub async fn service_manager_can_slash_operator_until_block(
        &self,
        operator_addr: Address,
        service_manager_addr: Address,
    ) -> Result<u32, ElContractsError> {
        let provider = Arc::new(self.client.clone());

        let contract_slasher = i_slasher::ISlasher::new(self.slasher, provider);

        let service_manager_can_slash_operator_until_block_result = contract_slasher
            .contract_can_slash_operator_until_block(operator_addr, service_manager_addr)
            .call()
            .await;

        match service_manager_can_slash_operator_until_block_result {
            Ok(service_manager_can_slash_operator_until_block) => {
                Ok(service_manager_can_slash_operator_until_block)
            }
            Err(_) => return Err(ElContractsError::ServiceManagerCanSlashOperatorExpiry),
        }
    }

    pub async fn get_strategy_and_underlying_erc20_token(
        &self,
        strategy_addr: Address,
    ) -> Result<(Address, Address, Address), ElContractsError> {
        let provider = Arc::new(self.client.clone());
        let contract_strategy = i_strategy::IStrategy::new(strategy_addr, provider.clone());

        let underlying_token_addr_result = contract_strategy.underlying_token().call().await;

        match underlying_token_addr_result {
            Ok(underlying_token_addr) => {
                let contract_ierc20 = ierc20::IERC20::new(underlying_token_addr, provider);

                return Ok((
                    strategy_addr,
                    underlying_token_addr,
                    contract_ierc20.address(),
                ));
            }
            Err(_) => return Err(ElContractsError::GetUnderlyingToken),
        }
    }

    pub async fn get_operator_details(
        &self,
        operator: Address,
    ) -> Result<Operator, ElContractsError> {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let operator_details_result = contract_delegation_manager
            .operator_details(operator)
            .call()
            .await;

        match operator_details_result {
            Ok(operator_details) => {
                return Ok(Operator::new(
                    operator,
                    operator_details.earnings_receiver,
                    operator_details.delegation_approver,
                    operator_details.staker_opt_out_window_blocks,
                    None,
                ));
            }
            Err(_) => {
                return Err(ElContractsError::GetOperatorDetails);
            }
        }
    }

    pub async fn is_operator_registered(
        &self,
        operator: Address,
    ) -> Result<bool, ElContractsError> {
        let provider = Arc::new(self.client.clone());

        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, provider);

        let is_operator_result = contract_delegation_manager
            .is_operator(operator)
            .call()
            .await;

        match is_operator_result {
            Ok(is_operator) => Ok(is_operator),
            Err(_) => return Err(ElContractsError::IsOperator),
        }
    }
}
