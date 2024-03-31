use crate::{error::ElContractsError, reader::ELChainReader};
use eigensdk_contract_bindings::{
    DelegationManager::delegation_manager, DelegationManager::OperatorDetails,
    StrategyManager::strategy_manager, IERC20::ierc20,
};
use eigensdk_txmgr::SimpleTxManager;
use eigensdk_types::operator::Operator;
use ethers::middleware::SignerMiddleware;
use ethers_core::types::{Address, TxHash, U256};
use ethers_providers::{Http, Provider};
use std::sync::Arc;
use tracing::info;
trait ELWriter {}

pub struct ELChainWriter {
    delegation_manager: Address,
    strategy_manager: Address,
    el_chain_reader: ELChainReader,
    client: Provider<Http>,
    tx_mgr: SimpleTxManager,
}

impl ELChainWriter {
    fn new(
        delegation_manager: Address,
        strategy_manager: Address,
        el_chain_reader: ELChainReader,
        client: Provider<Http>,
        tx_mgr: SimpleTxManager,
    ) -> Self {
        Self {
            delegation_manager,
            strategy_manager,
            el_chain_reader,
            client,
            tx_mgr,
        }
    }

    pub async fn register_as_operator(
        &self,
        operator: Operator,
    ) -> Result<TxHash, ElContractsError> {
        info!(
            "registering operator {:?} to EigenLayer",
            operator.has_address()
        );
        let op_details = OperatorDetails {
            earnings_receiver: operator.has_earnings_receiver_address(),
            delegation_approver: operator.has_delegation_approver_address(),
            staker_opt_out_window_blocks: operator.has_staker_opt_out_window_blocks(),
        };

        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, signer.into());

        let contract_call = contract_delegation_manager
            .register_as_operator(op_details, operator.has_metadata_url());

        let tx_result = contract_call.send().await;

        match tx_result {
            Ok(tx) => {
                info!(tx_hash = %tx.tx_hash(), "tx successfully included");
                Ok(tx.tx_hash())
            }
            Err(_) => return Err(ElContractsError::RegisterAsOperator),
        }
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
            earnings_receiver: operator.has_earnings_receiver_address(),
            delegation_approver: operator.has_delegation_approver_address(),
            staker_opt_out_window_blocks: operator.has_staker_opt_out_window_blocks(),
        };

        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(self.delegation_manager, signer.into());

        let contract_call_modify_operator_details =
            contract_delegation_manager.modify_operator_details(operator_details);

        let tx_result = contract_call_modify_operator_details.send().await;

        match tx_result {
            Ok(tx) => {
                info!(tx_hash = %tx.tx_hash(), operator = %operator.has_address(), "succesfully updated operator details");

                let contract_call_update_metadata_uri = contract_delegation_manager
                    .update_operator_metadata_uri(operator.has_metadata_url());

                let metadata_tx_result = contract_call_update_metadata_uri.send().await;

                match metadata_tx_result {
                    Ok(metadata_tx) => Ok(metadata_tx.tx_hash()),
                    Err(_) => return Err(ElContractsError::UpdateMetadataUri),
                }
            }
            Err(_) => return Err(ElContractsError::ModifyOperatorDetails),
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
        let result_value = self
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(strategy_addr)
            .await;

        match result_value {
            Ok((_, underlying_token_contract, underlying_token)) => {
                let provider = Arc::new(&self.client);
                let wallet = self.tx_mgr.wallet.signer.clone();
                let signer = SignerMiddleware::new(provider.clone(), wallet);

                let contract_underlying_token =
                    ierc20::IERC20::new(underlying_token_contract, signer.clone().into());

                let contract_call =
                    contract_underlying_token.approve(self.strategy_manager, amount);

                let _approve_tx_result = contract_call.send().await;

                match _approve_tx_result {
                    Ok(_approve_tx) => {
                        let contract_strategy_manager = strategy_manager::StrategyManager::new(
                            self.strategy_manager,
                            signer.into(),
                        );

                        let deposit_contract_call = contract_strategy_manager
                            .deposit_into_strategy(strategy_addr, underlying_token, amount);

                        let tx_result = deposit_contract_call.send().await;

                        match tx_result {
                            Ok(tx) => {
                                info!(
                                    "deposited {:?} tokens into strategy {:?}",
                                    amount, strategy_addr
                                );
                                Ok(tx.tx_hash())
                            }
                            Err(_) => {
                                return Err(ElContractsError::GetStrategyAndUnderlyingERC20Token)
                            }
                        }
                    }
                    Err(_) => return Err(ElContractsError::DepositIntoStrategy),
                }
            }
            Err(_) => return Err(ElContractsError::ApproveCallToUnderlyingToken),
        }
    }
}
