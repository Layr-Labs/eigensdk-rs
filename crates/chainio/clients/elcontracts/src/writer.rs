use crate::reader::ELChainReader;
use alloy_primitives::{Address, TxHash, U256};
pub use eigen_types::operator::Operator;
use eigen_utils::{binding::{DelegationManager, StrategyManager, IERC20}, get_signer};
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
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        info!(
            "registering operator {:?} to EigenLayer",
            operator.has_address()
        );

        let op_details = OperatorDetails {
            earningsReceiver: operator.has_earnings_receiver_address(),
            delegationApprover: operator.has_delegation_approver_address(),
            stakerOptOutWindowBlocks: operator.has_staker_opt_out_window_blocks(),
        };
        let provider = get_signer(self.signer.clone(),&self.provider);


        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        match operator.has_metadata_url() {
            Some(metadata) => {
                let contract_call =
                    contract_delegation_manager.registerAsOperator(op_details, metadata);
                let binding = contract_call.gas(130000);
                let tx = binding.send().await?;

                info!(tx_hash = %tx.tx_hash(), "tx successfully included");
                Ok(*tx.tx_hash())
            }
            None => {
                let contract_call =
                    contract_delegation_manager.registerAsOperator(op_details, "".to_string());
                let binding = contract_call.gas(130000);
                let tx = binding.send().await?;

                info!(tx_hash = %tx.tx_hash(), "tx successfully included");
                Ok(*tx.tx_hash())
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
        let provider = get_signer(self.signer.clone(),&self.provider);

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
        let provider = get_signer(self.signer.clone(),&self.provider);

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
