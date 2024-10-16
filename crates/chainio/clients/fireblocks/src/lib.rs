#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod client;
pub mod contract_call;
pub mod error;
pub mod get_asset_addresses;
mod get_transaction;
mod list_contracts;
pub mod list_external_accounts;
mod list_vault_accounts;
pub mod status;
pub mod transaction;
use std::collections::HashMap;
use std::str::FromStr;

use alloy::providers::Provider;
use alloy::rpc::types::transaction::TransactionReceipt;
use alloy_primitives::Address;
use alloy_primitives::U64;
use client::{Client, ASSET_ID_BY_CHAIN};
use eigen_utils::get_provider;
use error::FireBlockError;
use get_transaction::GetTransaction;
use list_contracts::ListContracts;
use list_contracts::WhitelistedContract;
use list_external_accounts::ListExternalAccounts;
use list_external_accounts::WhitelistedAccount;
use list_vault_accounts::{ListVaultAccounts, VaultAccount};
use status::Status;

/// Fireblocks wallet
#[derive(Debug)]
pub struct FireblocksWallet {
    fireblocks_client: Client,
    //
    vault_account_name: String,
    provider: String,
    chain_id: u64,
    vault_account: Option<VaultAccount>,
    whitelisted_accounts: HashMap<Address, WhitelistedAccount>,
    whitelisted_contracts: HashMap<Address, WhitelistedContract>,
    tx_id_to_nonce: HashMap<String, U64>,
}

impl FireblocksWallet {
    pub async fn new(
        fireblocks_client: Client,
        provider: String,
        vault_account_name: String,
    ) -> Result<FireblocksWallet, error::FireBlockError> {
        let provider_ = get_provider(&provider);
        let chain_id = provider_.get_chain_id().await.map_err(|e| {
            FireBlockError::AlloyContractError(alloy::contract::Error::TransportError(e))
        })?;

        Ok(Self {
            fireblocks_client,
            vault_account_name,
            chain_id,
            provider,
            vault_account: None,
            whitelisted_accounts: HashMap::new(),
            whitelisted_contracts: HashMap::new(),
            tx_id_to_nonce: HashMap::new(),
        })
    }

    /// Get Vault Accounts
    pub async fn get_account(&mut self) -> Result<Option<VaultAccount>, FireBlockError> {
        match &self.vault_account {
            None => {
                let accounts = self.fireblocks_client.list_vault_accounts().await?;

                for account in accounts.vault_accounts().iter() {
                    if account.name().eq(&self.vault_account_name) {
                        self.vault_account = Some(account.clone());
                        break;
                    }
                }
                Ok(self.vault_account.clone())
            }
            Some(account) => Ok(Some(account.clone())),
        }
    }

    /// Get whitelisted account for the particular address
    pub async fn get_whitelisted_account(
        &mut self,
        address: Address,
    ) -> Result<WhitelistedAccount, FireBlockError> {
        match ASSET_ID_BY_CHAIN.contains_key(&self.chain_id) {
            true => {
                let contains_account = self.whitelisted_accounts.contains_key(&address);

                match contains_account {
                    true => {
                        if self.whitelisted_accounts.get(&address).unwrap().id().eq("") {
                            Err(FireBlockError::AccountNotFoundError(address.to_string()))
                        } else {
                            Ok(self.whitelisted_accounts.get(&address).unwrap().clone())
                        }
                    }
                    false => {
                        let whitelisted_accounts;
                        let accounts = self.fireblocks_client.list_external_accounts().await?;

                        for account in accounts.iter() {
                            for asset in account.assets.iter() {
                                if asset.address.as_ref().unwrap().eq(&address.to_string())
                                    && asset.status.as_ref().unwrap().as_str() == "APPROVED"
                                    && *asset.id.as_ref().unwrap()
                                        == *ASSET_ID_BY_CHAIN.get(&self.chain_id).unwrap()
                                {
                                    self.whitelisted_accounts.insert(address, account.clone());
                                    whitelisted_accounts = account;
                                    return Ok(whitelisted_accounts.clone());
                                }
                            }
                        }

                        Ok(WhitelistedAccount::default())
                    }
                }
            }
            false => Err(FireBlockError::AssetIDError(self.chain_id.to_string())),
        }
    }

    /// Get whitelisted contract
    pub async fn get_whitelisted_contract(
        &mut self,
        address: Address,
    ) -> Result<WhitelistedContract, FireBlockError> {
        match ASSET_ID_BY_CHAIN.contains_key(&self.chain_id) {
            true => {
                let contains_contract = self.whitelisted_contracts.contains_key(&address);

                match contains_contract {
                    true => {
                        if self
                            .whitelisted_contracts
                            .get(&address)
                            .unwrap()
                            .id()
                            .eq("")
                        {
                            Err(FireBlockError::ContractNotFound(address.to_string()))
                        } else {
                            Ok(self.whitelisted_contracts.get(&address).unwrap().clone())
                        }
                    }

                    false => {
                        let contract;
                        let contracts_response = self.fireblocks_client.list_contracts().await?;
                        let contracts = contracts_response.contracts();
                        for c in contracts.iter() {
                            for a in c.assets().iter() {
                                if a.address.as_ref().unwrap().eq(&address.to_string())
                                    && a.status.as_ref().unwrap().as_str() == "APPROVED"
                                    && *a.id.as_ref().unwrap()
                                        == *ASSET_ID_BY_CHAIN.get(&self.chain_id).unwrap()
                                {
                                    self.whitelisted_contracts.insert(address, c.clone());
                                    contract = c;
                                    return Ok(contract.clone());
                                }
                            }
                        }

                        Ok(WhitelistedContract::default())
                    }
                }
            }
            false => Err(FireBlockError::AssetIDError(self.chain_id.to_string())),
        }
    }

    /// Get transaction receipt for the tx_id derived from fireblocks
    pub async fn get_transaction_receipt(
        &mut self,
        tx_id: String,
    ) -> Result<TransactionReceipt, FireBlockError> {
        let fireblocks_tx = self
            .fireblocks_client
            .get_transaction(tx_id.clone())
            .await?;

        match fireblocks_tx.status() {
            Status::Completed => {
                let provider = get_provider(&self.provider);
                let hash = alloy_primitives::FixedBytes::<32>::from_str(&fireblocks_tx.tx_hash())
                    .map_err(|e| FireBlockError::OtherError(e.to_string()))?;

                let tx_hash = provider.get_transaction_receipt(hash).await.map_err(|e| {
                    FireBlockError::AlloyContractError(alloy::contract::Error::TransportError(e))
                })?;

                let tx =
                    tx_hash.ok_or(FireBlockError::TransactionReceiptNotFound(tx_id.clone()))?;

                if self.tx_id_to_nonce.contains_key(&tx_id) {
                    self.tx_id_to_nonce.remove(&tx_id);
                }
                Ok(tx)
            }
            Status::Failed | Status::Rejected | Status::Cancelled | Status::Blocked => {
                Err(FireBlockError::TransactionFailed(
                    fireblocks_tx.status().as_str().to_string(),
                    tx_id,
                ))
            }
            Status::Submitted
            | Status::PendingAuthorization
            | Status::PendingScreening
            | Status::Queued
            | Status::PendingSignature
            | Status::PendingEmailApproval
            | Status::Pending3rdParity
            | Status::Broadcasting => Err(FireBlockError::NotBroadcasted(
                fireblocks_tx.status().as_str().to_string(),
                tx_id,
            )),
            _ => Err(FireBlockError::ReceiptNotYetAvailable(
                fireblocks_tx.status().as_str().to_string(),
                tx_id,
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "fireblock-tests")]
    use crate::client::Client;
    #[cfg(feature = "fireblock-tests")]
    use crate::FireblocksWallet;
    #[cfg(feature = "fireblock-tests")]
    use alloy_primitives::address;
    #[cfg(feature = "fireblock-tests")]
    use std::env;

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_get_account() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");

        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );
        let mut fireblocks_wallet = FireblocksWallet::new(
            client,
            "https://ethereum-holesky-rpc.publicnode.com".to_string(),
            "vault-name".to_string(),
        )
        .await
        .unwrap();

        let _ = fireblocks_wallet.get_account().await.unwrap();
    }

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_get_whitelisted_account() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");

        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );
        let mut fireblocks_wallet = FireblocksWallet::new(
            client,
            "https://ethereum-holesky-rpc.publicnode.com".to_string(),
            "vault-name".to_string(),
        )
        .await
        .unwrap();
        let address = address!("AE64660EfB3223445c55cd76654e89101FB23830");
        let _ = fireblocks_wallet
            .get_whitelisted_account(address)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_get_whitelisted_contract() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");

        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );
        let mut fireblocks_wallet = FireblocksWallet::new(
            client,
            "https://ethereum-holesky-rpc.publicnode.com".to_string(),
            "vault-name".to_string(),
        )
        .await
        .unwrap();
        let address = address!("AE64660EfB3223445c55cd76654e89101FB23830");
        let _ = fireblocks_wallet
            .get_whitelisted_contract(address)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_get_transaction_receipt() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");

        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );
        let mut fireblocks_wallet = FireblocksWallet::new(
            client,
            "https://holesky.drpc.org".to_string(),
            "vault-name".to_string(),
        )
        .await
        .unwrap();
        let tx_id = "39155aaa-cae7-45d8-824b-b74aef68edc0";
        let _ = fireblocks_wallet
            .get_transaction_receipt(tx_id.to_string())
            .await
            .unwrap();
    }
}
