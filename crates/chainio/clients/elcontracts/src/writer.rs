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

        let binding = {
            let contract_call = contract_delegation_manager
                .registerAsOperator(op_details, operator.has_metadata_url().unwrap_or_default());
            contract_call.gas(300000)
        };

        let binding_tx = binding
            .send()
            .await
            .map_err(|e| ElContractsError::AlloyContractError(e))?;

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

        let modify_operator_tx_result = contract_call_modify_operator_details.send().await;

        match modify_operator_tx_result {
            Ok(modify_operator_tx) => {
                info!(tx_hash = %modify_operator_tx.tx_hash(), operator = %operator.has_address(), "updated operator details tx");

                let contract_call_update_metadata_uri = contract_delegation_manager
                    .updateOperatorMetadataURI(operator.has_metadata_url().unwrap_or_default());

                let metadata_tx = contract_call_update_metadata_uri.send().await?;

                Ok(*metadata_tx.tx_hash())
            }
            Err(e) => Err(ElContractsError::AlloyContractError(e)),
        }
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
    use alloy_primitives::Address;
    use alloy_provider::Provider;
    use alloy_signer_local::PrivateKeySigner;
    use anvil_constants::{ANVIL_RPC_URL, CONTRACTS_REGISTRY};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants;
    use eigen_testing_utils::anvil_constants::{
        get_delegation_manager_address, get_service_manager_address, get_strategy_manager_address,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::binding::DelegationManager;
    use eigen_utils::binding::{
        mockAvsServiceManager,
        ContractsRegistry::{self, get_test_valuesReturn},
    };
    use std::str::FromStr;
    //get_erc20_mock_strategy, get_operator_state_retriever_address, get_registry_coordinator_address,

    /// Returns a new instance of ELChainWriter and the address of the delegation manager contract
    ///
    /// # Returns
    ///
    /// A tuple containing an instance of ELChainWriter and the address of the delegation manager contract
    async fn setup_el_chain_reader() -> (ELChainReader, Address) {
        let delegation_manager_address = get_delegation_manager_address().await;
        let delegation_manager_contract = DelegationManager::new(
            delegation_manager_address,
            anvil_constants::ANVIL_RPC_URL.clone(),
        );
        let slasher_address_return = delegation_manager_contract.slasher().call().await.unwrap();
        let DelegationManager::slasherReturn {
            _0: slasher_address,
        } = slasher_address_return;

        let service_manager_address = get_service_manager_address().await;
        let service_manager_contract = mockAvsServiceManager::new(
            service_manager_address,
            anvil_constants::ANVIL_RPC_URL.clone(),
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
                "http://localhost:8545".to_string(),
            ),
            delegation_manager_address,
        )
    }

    #[tokio::test]
    async fn test_register_operator() {
        let (el_chain_reader, _delegation_manager_address) = setup_el_chain_reader().await;

        let operator_pvt_key = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
        let operator: PrivateKeySigner = (operator_pvt_key)
            .parse()
            .expect("failed to generate wallet");

        let contract_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, ANVIL_RPC_URL.clone());
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
        let (el_chain_reader, _) = setup_el_chain_reader().await;
        let http_endpoint = "http://localhost:8545".to_string();
        let operator_addr = Address::from_str("90F79bf6EB2c4f870365E785982E1f101E93b906").unwrap();
        let operator_private_key =
            "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6".to_string();
        let strategy_manager = get_strategy_manager_address().await;

        let el_chain_writer = ELChainWriter::new(
            operator_addr,
            strategy_manager,
            el_chain_reader,
            http_endpoint,
            operator_private_key,
        );

        // define an operator
        let wallet = PrivateKeySigner::from_str(
            "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8",
        )
        .expect("no key");

        let operator = Operator::new(
            wallet.address(),
            wallet.address(),
            wallet.address(),
            3,
            Some("eigensdk-rs".to_string()),
        );

        let tx_hash = el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        // this sleep is needed so that we wait for the tx to be processed
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let receipt = ANVIL_RPC_URL
            .get_transaction_receipt(tx_hash)
            .await
            .unwrap();
        assert!(receipt.unwrap().status());
    }
}
