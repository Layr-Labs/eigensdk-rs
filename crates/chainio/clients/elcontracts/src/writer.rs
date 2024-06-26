use crate::reader::ELChainReader;
use alloy_network::ReceiptResponse;
use alloy_primitives::FixedBytes;
use alloy_primitives::{Address, TxHash, I256, U256, U32};
pub use eigen_types::operator::Operator;
use eigen_utils::{
    binding::{
        DelegationManager::{self, isOperatorReturn},
        StrategyManager, IERC20,
    },
    get_signer,
};

use tracing::info;
use DelegationManager::OperatorDetails;

#[derive(Debug, Clone)]
pub struct ELChainWriter {
    delegation_manager: Address,
    strategy_manager: Address,
    el_chain_reader: ELChainReader,
    provider: String,
    signer: String,
}

impl ELChainWriter {
    pub fn new(
        delegation_manager: Address,
        strategy_manager: Address,
        el_chain_reader: ELChainReader,
        provider: String,
        signer: String,
    ) -> Self {
        Self {
            delegation_manager,
            strategy_manager,
            el_chain_reader,
            provider,
            signer,
        }
    }

    pub async fn register_as_operator(
        &self,
        operator: Operator,
    ) -> Result<FixedBytes<32>, Box<dyn std::error::Error>> {
        info!(
            "registering operator {:?} to EigenLayer",
            operator.has_address()
        );
        let op_details = OperatorDetails {
            earningsReceiver: operator.has_earnings_receiver_address(),
            delegationApprover: operator.has_delegation_approver_address(),
            stakerOptOutWindowBlocks: operator.has_staker_opt_out_window_blocks(),
        };
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let already_operator_return = contract_delegation_manager
            .isOperator(operator.has_earnings_receiver_address())
            .call()
            .await?;
        let isOperatorReturn { _0: isoperator } = already_operator_return;
        let binding = match operator.has_metadata_url() {
            Some(metadata) => {
                let contract_call =
                    contract_delegation_manager.registerAsOperator(op_details, metadata);
                let binding = contract_call.gas(130000);
                binding
            }
            None => {
                let contract_call =
                    contract_delegation_manager.registerAsOperator(op_details, "".to_string());
                let binding = contract_call.gas(130000);
                binding
            }
        };
        let tx = binding.send().await?;
        let receipt = tx.get_receipt().await?;
        let tx_status = receipt.status();
        let hash = receipt.transaction_hash;
        match tx_status {
            true => {
                info!(tx_hash = %receipt.transaction_hash, "tx successfully included");
                Ok(hash)
            }
            false => {
                info!(tx_hash = %receipt.transaction_hash, "tx failed");
                Err("Failed to register operator".into())
            }
        }
    }

    pub async fn update_operator_details(
        &self,
        operator: Operator,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        info!(
            "updating operator detils of operator {:?} to EigenLayer",
            operator.has_address()
        );
        let operator_details = OperatorDetails {
            earningsReceiver: operator.has_earnings_receiver_address(),
            delegationApprover: operator.has_delegation_approver_address(),
            stakerOptOutWindowBlocks: operator.has_staker_opt_out_window_blocks(),
        };
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let contract_call_modify_operator_details =
            contract_delegation_manager.modifyOperatorDetails(operator_details);

        let tx = contract_call_modify_operator_details.send().await?;

        info!(tx_hash = %tx.tx_hash(), operator = %operator.has_address(), "succesfully updated operator details");

        let contract_call_update_metadata_uri = contract_delegation_manager
            .updateOperatorMetadataURI(operator.has_metadata_url().unwrap_or_default());

        let metadata_tx = contract_call_update_metadata_uri.send().await?;

        Ok(*metadata_tx.tx_hash())
    }

    pub async fn deposit_erc20_into_strategy(
        &self,
        strategy_addr: Address,
        amount: U256,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        info!(
            "depositing {:?} tokens into strategy {:?}",
            amount, strategy_addr
        );
        let tokens = self
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(strategy_addr)
            .await?;
        let (_, underlying_token_contract, underlying_token) = tokens;
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_underlying_token = IERC20::new(underlying_token_contract, &provider);

        let contract_call = contract_underlying_token.approve(self.strategy_manager, amount);

        let _approve = contract_call.send().await?;

        let contract_strategy_manager = StrategyManager::new(self.strategy_manager, &provider);

        let deposit_contract_call =
            contract_strategy_manager.depositIntoStrategy(strategy_addr, underlying_token, amount);

        let tx = deposit_contract_call.send().await?;

        info!(
            "deposited {:?} tokens into strategy {:?}",
            amount, strategy_addr
        );
        Ok(*tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy_signer_local::PrivateKeySigner;
    use eigen_testing_utils::anvil_constants::{self, ANVIL_RPC_URL};
    use eigen_utils::binding::{
        mockAvsServiceManager,
        ContractsRegistry::{self, get_test_valuesReturn},
    };
    use std::str::FromStr;

    #[tokio::test]
    async fn test_register_operator() {
        let delegation_manager_address = anvil_constants::get_delegation_manager_address().await;
        let strategy_manager_address = anvil_constants::get_strategy_manager_address().await;
        let delegation_manager_contract = DelegationManager::new(
            delegation_manager_address,
            anvil_constants::ANVIL_RPC_URL.clone(),
        );
        let slasher_address_return = delegation_manager_contract.slasher().call().await.unwrap();
        let DelegationManager::slasherReturn {
            _0: slasher_address,
        } = slasher_address_return;
        let service_manager_address = anvil_constants::get_service_manager_address().await;
        let service_manager_contract = mockAvsServiceManager::new(
            service_manager_address,
            anvil_constants::ANVIL_RPC_URL.clone(),
        );
        let operator_pvt_key = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
        let operator: PrivateKeySigner = (operator_pvt_key)
            .parse()
            .expect("failed to generate wallet");
        let avs_directory_address_return = service_manager_contract
            .avsDirectory()
            .call()
            .await
            .unwrap();
        let mockAvsServiceManager::avsDirectoryReturn {
            _0: avs_directory_address,
        } = avs_directory_address_return;
        let el_chain_reader = ELChainReader::new(
            slasher_address,
            delegation_manager_address,
            avs_directory_address,
            "http://localhost:8545".to_string(),
        );
        let el_chain_writer = ELChainWriter::new(
            delegation_manager_address,
            strategy_manager_address,
            el_chain_reader.clone(),
            "http://localhost:8545".to_string(),
            operator_pvt_key.to_string(),
        );

        let contract_registry = ContractsRegistry::new(
            anvil_constants::CONTRACTS_REGISTRY,
            anvil_constants::ANVIL_RPC_URL.clone(),
        );
        /// Use these value in tests when needed
        let operator_index = "1".parse().unwrap();
        let get_test_values_return = contract_registry
            .get_test_values("test_register_operator".to_string(), operator_index)
            .call()
            .await
            .unwrap();
        let get_test_valuesReturn {
            _0: timestamp,
            _1: blocknumber,
            _2: index,
        } = get_test_values_return;

        // operator who registered at index 1
        let operator_address = operator.address();
        let operator_details = Operator::new(
            operator_address,
            operator_address,
            Address::ZERO,
            "0".parse().unwrap(),
            Some("https://coolstuff.com/operator/".to_string()),
        );
        assert!(el_chain_reader
            .is_operator_registered(operator_address)
            .await
            .unwrap());
        assert_eq!(
            el_chain_writer
                .register_as_operator(operator_details)
                .await
                .unwrap_err()
                .to_string(),
            "Failed to register operator"
        );
    }
}
