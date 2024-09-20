use crate::error::ElContractsError;
use crate::reader::ELChainReader;
use alloy_primitives::{Address, FixedBytes, TxHash, U256};
pub use eigen_types::operator::Operator;
use eigen_utils::{
    binding::{DelegationManager, StrategyManager, IERC20},
    get_signer,
};

use tracing::info;
use DelegationManager::OperatorDetails;

/// Gas limit for registerAsOperator in [`DelegationManager`]
pub const GAS_LIMIT_REGISTER_AS_OPERATOR_DELEGATION_MANAGER: u128 = 300000;

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
    ) -> Result<FixedBytes<32>, ElContractsError> {
        info!("registering operator {:?} to EigenLayer", operator.address);
        let op_details = OperatorDetails {
            earningsReceiver: operator.earnings_receiver_address,
            delegationApprover: operator.delegation_approver_address,
            stakerOptOutWindowBlocks: operator.staker_opt_out_window_blocks,
        };
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let binding = {
            let contract_call = contract_delegation_manager
                .registerAsOperator(op_details, operator.metadata_url.unwrap_or_default());
            contract_call.gas(300000)
        };

        let binding_tx = binding
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let receipt = binding_tx.get_receipt().await.map_err(|e| {
            ElContractsError::AlloyContractError(alloy_contract::Error::TransportError(e))
        })?;

        let tx_status = receipt.status();
        let hash = receipt.transaction_hash;
        if tx_status {
            info!(tx_hash = %receipt.transaction_hash, "tx successfully included");
        } else {
            info!(tx_hash = %receipt.transaction_hash, "failed to register operator");
        };
        Ok(hash)
    }

    pub async fn update_operator_details(
        &self,
        operator: Operator,
    ) -> Result<TxHash, ElContractsError> {
        info!(
            "updating operator detils of operator {:?} to EigenLayer",
            operator.address
        );
        let operator_details = OperatorDetails {
            earningsReceiver: operator.earnings_receiver_address,
            delegationApprover: operator.delegation_approver_address,
            stakerOptOutWindowBlocks: operator.staker_opt_out_window_blocks,
        };
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let contract_call_modify_operator_details =
            contract_delegation_manager.modifyOperatorDetails(operator_details);

        let modify_operator_tx = contract_call_modify_operator_details
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        info!(tx_hash = %modify_operator_tx.tx_hash(), operator = %operator.address, "updated operator details tx");

        let contract_call_update_metadata_uri = contract_delegation_manager
            .updateOperatorMetadataURI(operator.metadata_url.unwrap_or_default());

        let metadata_tx = contract_call_update_metadata_uri.send().await?;

        Ok(*metadata_tx.tx_hash())
    }

    pub async fn deposit_erc20_into_strategy(
        &self,
        strategy_addr: Address,
        amount: U256,
    ) -> Result<TxHash, ElContractsError> {
        info!(
            "depositing {:?} tokens into strategy {:?}",
            amount, strategy_addr
        );
        let tokens_result = self
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(strategy_addr)
            .await;
        match tokens_result {
            Ok(tokens) => {
                let (_, underlying_token_contract, underlying_token) = tokens;
                let provider = get_signer(self.signer.clone(), &self.provider);

                let contract_underlying_token = IERC20::new(underlying_token_contract, &provider);

                let contract_call =
                    contract_underlying_token.approve(self.strategy_manager, amount);

                let _approve = contract_call.send().await?;

                let contract_strategy_manager =
                    StrategyManager::new(self.strategy_manager, &provider);

                let deposit_contract_call = contract_strategy_manager.depositIntoStrategy(
                    strategy_addr,
                    underlying_token,
                    amount,
                );

                let tx = deposit_contract_call.send().await?;

                info!(
                    "deposited {:?} tokens into strategy {:?}",
                    amount, strategy_addr
                );
                Ok(*tx.tx_hash())
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ELChainWriter;
    use crate::reader::ELChainReader;
    use alloy_primitives::{Address, U256};
    use alloy_provider::Provider;
    use alloy_signer_local::PrivateKeySigner;
    use anvil_constants::CONTRACTS_REGISTRY;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{
            self, get_delegation_manager_address, get_erc20_mock_strategy,
            get_service_manager_address, get_strategy_manager_address,
        },
    };
    use eigen_types::operator::Operator;
    use eigen_utils::{
        binding::{
            mockAvsServiceManager,
            ContractsRegistry::{self, get_test_valuesReturn},
            DelegationManager,
        },
        get_provider,
    };
    use std::str::FromStr;

    /// Returns a new instance of ELChainWriter and the address of the delegation manager contract
    ///
    /// # Returns
    ///
    /// A tuple containing an instance of ELChainWriter and the address of the delegation manager contract
    async fn setup_el_chain_reader(http_endpoint: String) -> (ELChainReader, Address) {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let delegation_manager_contract = DelegationManager::new(
            delegation_manager_address,
            get_provider(http_endpoint.as_str()),
        );
        let slasher_address_return = delegation_manager_contract.slasher().call().await.unwrap();
        let DelegationManager::slasherReturn {
            _0: slasher_address,
        } = slasher_address_return;

        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let service_manager_contract = mockAvsServiceManager::new(
            service_manager_address,
            get_provider(http_endpoint.as_str()),
        );
        let avs_directory_address_return = service_manager_contract
            .avsDirectory()
            .call()
            .await
            .unwrap();

        let mockAvsServiceManager::avsDirectoryReturn {
            _0: avs_directory_address,
        } = avs_directory_address_return;

        (
            ELChainReader::new(
                get_test_logger().clone(),
                slasher_address,
                delegation_manager_address,
                avs_directory_address,
                http_endpoint,
            ),
            delegation_manager_address,
        )
    }

    #[tokio::test]
    async fn test_register_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        let (el_chain_reader, _delegation_manager_address) =
            setup_el_chain_reader(http_endpoint).await;

        let operator_pvt_key = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
        let operator: PrivateKeySigner = (operator_pvt_key)
            .parse()
            .expect("failed to generate wallet");

        let contract_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, provider);
        // Use these value in tests when needed
        let operator_index = "1".parse().unwrap();
        let get_test_values_return = contract_registry
            .get_test_values("test_register_operator".to_string(), operator_index)
            .call()
            .await
            .unwrap();
        let get_test_valuesReturn {
            _0: _timestamp,
            _1: _blocknumber,
            _2: _index,
        } = get_test_values_return;

        // operator who registered at index 1
        let operator_address = operator.address();
        assert!(el_chain_reader
            .is_operator_registered(operator_address)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_chain_writer() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        let (el_chain_reader, _) = setup_el_chain_reader(http_endpoint.clone()).await;
        let operator_addr = Address::from_str("90F79bf6EB2c4f870365E785982E1f101E93b906").unwrap();
        let operator_private_key =
            "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6".to_string();
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;

        let el_chain_writer = ELChainWriter::new(
            operator_addr,
            strategy_manager,
            el_chain_reader,
            http_endpoint.clone(),
            operator_private_key,
        );

        // define an operator
        let wallet = PrivateKeySigner::from_str(
            "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8",
        )
        .expect("no key");

        let operator = Operator {
            address: wallet.address(),
            earnings_receiver_address: wallet.address(),
            delegation_approver_address: wallet.address(),
            staker_opt_out_window_blocks: 3,
            metadata_url: Some("eigensdk-rs".to_string()),
        };

        // First test: register as an operator
        let tx_hash = el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        // this sleep is needed so that we wait for the tx to be processed
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();
        assert!(receipt.unwrap().status());

        let wallet_modified = PrivateKeySigner::from_str(
            "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
        )
        .expect("no key");

        let operator_modified = Operator {
            address: wallet_modified.address(),
            earnings_receiver_address: wallet_modified.address(),
            delegation_approver_address: wallet_modified.address(),
            staker_opt_out_window_blocks: 3,
            metadata_url: Some("eigensdk-rs".to_string()),
        };

        // Second test: update operator details
        let tx_hash = el_chain_writer
            .update_operator_details(operator_modified)
            .await
            .unwrap();

        // this sleep is needed so that we wait for the tx to be processed
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();
        assert!(receipt.unwrap().status());

        // Third test: deposit_erc20_into_strategy
        let amount = U256::from_str("100").unwrap();
        let strategy_addr = get_erc20_mock_strategy(http_endpoint).await;
        let tx_hash = el_chain_writer
            .deposit_erc20_into_strategy(strategy_addr, amount)
            .await
            .unwrap();

        // this sleep is needed so that we wait for the tx to be processed
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();
        assert!(receipt.unwrap().status());
    }
}
