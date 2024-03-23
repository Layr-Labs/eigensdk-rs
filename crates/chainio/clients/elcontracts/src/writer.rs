use crate::reader::ELChainReader;
use eigensdk_contracts_bindings::{
    DelegationManager::OperatorDetails,
    DelegationManager::{self, delegation_manager},
    StrategyManager::{self, strategy_manager},
    IERC20::{self, ierc20},
};
use eigensdk_txmgr::SimpleTxManager;
use eigensdk_types::operator::Operator;
use ethers::middleware::SignerMiddleware;
use ethers_core::types::{Address, TxHash, U256};
use ethers_providers::{Http, Provider};
use std::sync::Arc;
trait ELWriter {}

pub struct ELChainWriter {
    slasher: Address,
    delegation_manager: Address,
    strategy_manager: Address,
    el_chain_reader: ELChainReader,
    client: Provider<Http>,
    tx_mgr: SimpleTxManager,
}

impl ELChainWriter {
    fn new(
        slasher: Address,
        delegation_manager: Address,
        strategy_manager: Address,
        el_chain_reader: ELChainReader,
        client: Provider<Http>,
        tx_mgr: SimpleTxManager,
    ) -> Self {
        Self {
            slasher,
            delegation_manager,
            strategy_manager,
            el_chain_reader,
            client,
            tx_mgr,
        }
    }

    pub async fn register_as_operator(&self, operator: Operator) -> Result<TxHash, String> {
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

        let tx = contract_call.send().await.unwrap();

        Ok(tx.tx_hash())
    }

    pub async fn update_operator_details(&self, operator: Operator) -> Result<TxHash, String> {
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

        let contract_call = contract_delegation_manager.modify_operator_details(operator_details);

        let tx = contract_call.send().await.unwrap();

        Ok(tx.tx_hash())
    }

    pub async fn deposit_erc20_into_strategy(
        &self,
        strategy_addr: Address,
        amount: U256,
    ) -> Result<TxHash, String> {
        let (_, underlying_token_contract, underlying_token) = self
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(strategy_addr)
            .await
            .unwrap();

        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);

        let contract_underlying_token =
            ierc20::IERC20::new(underlying_token_contract, signer.clone().into());

        let contract_call = contract_underlying_token.approve(self.strategy_manager, amount);

        let _approve_tx = contract_call.send().await.unwrap();

        let contract_strategy_manager =
            strategy_manager::StrategyManager::new(self.strategy_manager, signer.into());

        let deposit_contract_call = contract_strategy_manager.deposit_into_strategy(
            strategy_addr,
            underlying_token,
            amount,
        );

        let tx = deposit_contract_call.send().await.unwrap();

        Ok(tx.tx_hash())
    }
}
