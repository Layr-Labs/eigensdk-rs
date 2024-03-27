use crate::error::PrivateKeyWalletError;
use crate::{TxId, WalletTrait};
use ethers::{
    middleware::SignerMiddleware,
    types::{
        transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, TransactionReceipt,
    },
};
use ethers_core::types::{BlockId, BlockNumber, TxHash, U256};
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::LocalWallet;
use std::str::FromStr;
use std::sync::Arc;

pub struct PrivateKeyWallet {
    client: Provider<Http>,
    pub signer: LocalWallet,
}

impl WalletTrait for PrivateKeyWallet {
    async fn send_transaction(
        &self,
        tx: TypedTransaction,
    ) -> Result<TxHash, PrivateKeyWalletError> {
        let transaction_result = self.estimate_gas_and_nonce(tx.clone()).await;

        match transaction_result {
            Ok(txs) => {
                let provider = Arc::new(self.client.clone());
                let current_block_number_result = provider.clone().get_block_number().await;

                match current_block_number_result {
                    Ok(current_block_number) => {
                        let sign_client = SignerMiddleware::new(provider, self.signer.clone());

                        if let Some(transaction_val) = txs {
                            let tx_receipt_result = sign_client
                                .send_transaction(
                                    transaction_val,
                                    Some(BlockId::Number(BlockNumber::Number(
                                        current_block_number,
                                    ))),
                                )
                                .await;

                            match tx_receipt_result {
                                Ok(tx_receipt) => {
                                    return Ok(tx_receipt.tx_hash());
                                }
                                Err(_) => return Err(PrivateKeyWalletError::SendTransaction),
                            }
                        } else {
                            return Err(PrivateKeyWalletError::EstimateGasAndNonce);
                        }
                    }
                    Err(_) => return Err(PrivateKeyWalletError::GetBlockNumber),
                }
            }
            Err(_) => return Err(PrivateKeyWalletError::EstimateGasAndNonce),
        }
    }
}

impl PrivateKeyWallet {
    pub fn new(client: Provider<Http>, pvt_key: &str) -> Result<Self, PrivateKeyWalletError> {
        let wallet_result = pvt_key.parse();

        match wallet_result {
            Ok(wallet) => {
                return Ok(Self {
                    client,
                    signer: wallet,
                });
            }
            Err(_) => return Err(PrivateKeyWalletError::BuildWallet),
        }
    }

    async fn estimate_gas_and_nonce(
        &self,
        tx: TypedTransaction,
    ) -> Result<Option<TypedTransaction>, PrivateKeyWalletError> {
        let provider = Arc::new(self.client.clone());
        let current_block_number_result = provider.clone().get_block_number().await;

        match current_block_number_result {
            Ok(current_block_number) => {
                let mut base_fee = U256::default();
                let mut priority_fees = Vec::new();
                let start_block = current_block_number - 3;
                let start_block_num = start_block.as_u64();
                let current_block_num = current_block_number.as_u64();

                for block_number in start_block_num..=current_block_num {
                    let block_result = provider.clone().get_block(block_number).await;

                    match block_result {
                        Ok(block_r) => {
                            if let Some(block) = block_r {
                                if let Some(base_fee_per_gas) = block.base_fee_per_gas {
                                    if block_number == current_block_num {
                                        base_fee = base_fee_per_gas;
                                    }
                                    if let Some(tx_hash) = block.transactions.get(0) {
                                        let tx_receipt_result =
                                            provider.get_transaction_receipt(*tx_hash).await;

                                        match tx_receipt_result {
                                            Ok(tx_receipts) => {
                                                if let Some(tx_receipt) = tx_receipts {
                                                    if let Some(effective_priority_fee) = tx_receipt
                                                        .effective_gas_price
                                                        .unwrap()
                                                        .checked_sub(base_fee_per_gas)
                                                    {
                                                        priority_fees.push(effective_priority_fee);
                                                    }
                                                }
                                            }
                                            Err(_) => {
                                                return Err(
                                                    PrivateKeyWalletError::TransactionReceipt,
                                                )
                                            }
                                        }
                                    }
                                }
                            }

                            let gas_to_cap_result = provider
                                .clone()
                                .estimate_eip1559_fees(Some(custom_estimator))
                                .await;

                            match gas_to_cap_result {
                                Ok(gas_to_cap) => {
                                    let gas_limit_result = provider
                                        .estimate_gas(
                                            &tx,
                                            Some(BlockId::Number(BlockNumber::Number(
                                                current_block_number,
                                            ))),
                                        )
                                        .await;

                                    match gas_limit_result {
                                        Ok(gas_limit) => {
                                            let s: Eip1559TransactionRequest =
                                                TypedTransaction::Eip1559(tx.into()).into();
                                            let _ = s.clone().max_fee_per_gas(gas_to_cap.0);
                                            let _ =
                                                s.clone().max_priority_fee_per_gas(gas_to_cap.1);
                                            let _ = s.clone().gas(gas_limit);

                                            return Ok(Some(TypedTransaction::Eip1559(
                                                s.clone().into(),
                                            )));
                                        }
                                        Err(_) => return Err(PrivateKeyWalletError::EstimateGas),
                                    }
                                }
                                Err(_) => return Err(PrivateKeyWalletError::EstimateEip1559Fees),
                            }
                        }
                        Err(_) => return Err(PrivateKeyWalletError::GetBlock),
                    }
                }

                Ok(None)
            }
            Err(_) => return Err(PrivateKeyWalletError::GetBlockNumber),
        }
    }

    async fn get_transaction_receipt(
        &self,
        tx_id: TxId,
    ) -> Result<Option<TransactionReceipt>, PrivateKeyWalletError> {
        let tx_hash_result = TxHash::from_str(&tx_id);

        match (tx_hash_result) {
            Ok(tx_hash) => {
                let receipt_result = self.client.get_transaction_receipt(tx_hash).await;

                match receipt_result {
                    Ok(receipt) => {
                        return Ok(receipt);
                    }
                    Err(_) => return Err(PrivateKeyWalletError::GetTransactionReceipt),
                }
            }
            Err(_) => return Err(PrivateKeyWalletError::TxIdtoTxHash),
        }
    }
}
fn custom_estimator(base_fee: U256, priority_fees: Vec<Vec<U256>>) -> (U256, U256) {
    (
        base_fee,
        priority_fees.last().unwrap().last().unwrap().clone(),
    )
}
