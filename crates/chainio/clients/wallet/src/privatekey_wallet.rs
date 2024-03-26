// use alloy_primitives::TxHash;
use crate::{TxId, WalletTrait};
use eigensdk_signerv2::PrivateKeySigner;
use ethers::{
    middleware::SignerMiddleware,
    types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest},
};
use ethers_core::types::{Address, BlockId, BlockNumber, TxHash, U256};
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{LocalWallet, Wallet};
use std::sync::Arc;

pub struct PrivateKeyWallet {
    client: Provider<Http>,
    address: Address,
    pub signer: LocalWallet,
}

impl WalletTrait for PrivateKeyWallet {
    async fn send_transaction(&self, tx: TypedTransaction) -> Result<TxHash, String> {
        let transaction = self.estimate_gas_and_nonce(tx.clone()).await.unwrap();
        let provider = Arc::new(self.client.clone());
        let current_block_number = provider.clone().get_block_number().await.unwrap();
        // let signer = self.signer.set_private_k
        let sign_client = SignerMiddleware::new(provider, self.signer.clone());
        let tx_receipt = sign_client
            .send_transaction(
                tx,
                Some(BlockId::Number(BlockNumber::Number(current_block_number))),
            )
            .await
            .unwrap();

        return Ok(tx_receipt.tx_hash());
    }
}

impl PrivateKeyWallet {
    pub fn new(
        client: Provider<Http>,
        address: Address,
        pvt_key: &str,
        signer: LocalWallet,
    ) -> Self {
        let wallet: LocalWallet = pvt_key.parse().unwrap();

        return Self {
            client,
            address,
            signer: wallet,
        };
    }

    async fn estimate_gas_and_nonce(
        &self,
        tx: TypedTransaction,
    ) -> Result<TypedTransaction, String> {
        let provider = Arc::new(self.client.clone());
        let current_block_number = provider.clone().get_block_number().await.unwrap();

        let mut priority_fees = Vec::new();
        let mut base_fee: U256;
        let start_block = current_block_number - 3;
        let start_block_num = start_block.as_u64();
        let current_block_num = current_block_number.as_u64();

        for block_number in start_block_num..=current_block_num {
            if let Some(block) = provider.clone().get_block(block_number).await.unwrap() {
                if let Some(base_fee_per_gas) = block.base_fee_per_gas {
                    if block_number == current_block_num {
                        base_fee = (base_fee_per_gas);
                    }
                    if let Some(tx_hash) = block.transactions.get(0) {
                        if let Some(tx_receipt) =
                            provider.get_transaction_receipt(*tx_hash).await.unwrap()
                        {
                            if let Some(effective_priority_fee) = tx_receipt
                                .effective_gas_price
                                .unwrap()
                                .checked_sub(base_fee_per_gas)
                            {
                                priority_fees.push(effective_priority_fee);
                            }
                        }
                    }
                }
            }
        }

        let gas_to_cap = provider
            .clone()
            .estimate_eip1559_fees(Some(custom_estimator))
            .await
            .unwrap();

        let gas_limit = provider
            .estimate_gas(
                &tx,
                Some(BlockId::Number(BlockNumber::Number(current_block_number))),
            )
            .await
            .unwrap();

        let s: Eip1559TransactionRequest = TypedTransaction::Eip1559(tx.into()).into();
        s.clone().max_fee_per_gas(gas_to_cap.0);
        s.clone().max_priority_fee_per_gas(gas_to_cap.1);
        s.clone().gas(gas_limit);

        Ok(TypedTransaction::Eip1559(s.clone().into()))
    }
}

fn custom_estimator(base_fee: U256, priority_fees: Vec<Vec<U256>>) -> (U256, U256) {
    (
        base_fee,
        priority_fees.last().unwrap().last().unwrap().clone(),
    )
}
