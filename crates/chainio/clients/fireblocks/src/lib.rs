#![doc(
    html_logo_url = "https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/supernovahs/eigen-rs/issues/"
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

use alloy_primitives::Address;
use alloy_provider::Provider;
use client::{Client, ASSET_ID_BY_CHAIN};
use eigen_utils::get_provider;
use error::FireBlockError;
use list_contracts::ListContracts;
use list_contracts::WhitelistedContract;
use list_external_accounts::ListExternalAccounts;
use list_external_accounts::WhitelistedAccount;
use list_vault_accounts::{ListVaultAccounts, VaultAccount};

/// Fireblocks wallet
#[derive(Debug)]
pub struct FireblocksWallet {
    fireblocks_client: Client,
    vault_account_name: String,
    chain_id: u64,
    vault_account: Option<Vec<VaultAccount>>,
    whitelisted_accounts: HashMap<Address, WhitelistedAccount>,
    whitelisted_contracts: HashMap<Address, WhitelistedContract>,
}

impl FireblocksWallet {
    pub async fn new(
        fireblocks_client: Client,
        provider: String,
        vault_account_name: String,
    ) -> Result<FireblocksWallet, error::FireBlockError> {
        let provider_ = get_provider(&provider);
        let chain_id_result = provider_.get_chain_id().await;

        match chain_id_result {
            Ok(chain_id) => Ok(Self {
                fireblocks_client,
                vault_account_name,
                chain_id,
                vault_account: None,
                whitelisted_accounts: HashMap::new(),
                whitelisted_contracts: HashMap::new(),
            }),
            Err(e) => Err(FireBlockError::AlloyContractError(
                alloy_contract::Error::TransportError(e),
            )),
        }
    }

    /// Get Account
    pub async fn get_account(&self) -> Result<Vec<VaultAccount>, FireBlockError> {
        if self.vault_account.is_none() {
            let accounts_result = self.fireblocks_client.list_vault_accounts().await;

            match accounts_result {
                Ok(accounts) => Ok(accounts.vault_accounts()),
                Err(e) => Err(e),
            }
        } else {
            // Already checking if vault account is available in above if condition, so using expect
            Ok(self
                .vault_account
                .as_ref()
                .expect("Vault account not found")
                .to_vec())
        }
    }

    /// get whitelisted account
    pub async fn get_whitelisted_account(
        &mut self,
        address: Address,
    ) -> Result<WhitelistedAccount, FireBlockError> {
        let asset_id_some = ASSET_ID_BY_CHAIN.get(&self.chain_id);

        match asset_id_some {
            Some(asset_id) => {
                let whitelisted_account_some = self.whitelisted_accounts.get(&address);

                match whitelisted_account_some {
                    Some(mut whitelisted_accounts) => {
                        let accounts_result = self.fireblocks_client.list_external_accounts().await;

                        match accounts_result {
                            Ok(accounts) => {
                                for (_, account) in accounts.iter().enumerate() {
                                    for (_, asset) in account.assets.iter().enumerate() {
                                        if asset.address.as_ref().unwrap().eq(&address.to_string())
                                            && asset.status.as_ref().unwrap().as_str() == "APPROVED"
                                            && *asset.id.as_ref().unwrap() == *asset_id
                                        {
                                            self.whitelisted_accounts
                                                .insert(address, account.clone());
                                            whitelisted_accounts = account;
                                            return Ok(whitelisted_accounts.clone());
                                        }
                                    }
                                }

                                Ok(whitelisted_accounts.clone())
                            }
                            Err(e) => Err(e),
                        }
                    }
                    None => {
                        return Err(FireBlockError::AccountNotFoundError(address.to_string()));
                    }
                }
            }
            None => Err(FireBlockError::AssetIDError(self.chain_id.to_string())),
        }
    }

    /// get whitelisted contract
    pub async fn get_whitelisted_contract(
        &mut self,
        address: Address,
    ) -> Result<WhitelistedContract, FireBlockError> {
        let asset_id_some = ASSET_ID_BY_CHAIN.get(&self.chain_id);
        match asset_id_some {
            Some(asset_id) => {
                let contract_some = self.whitelisted_contracts.get(&address);

                match contract_some {
                    Some(mut contract) => {
                        let contracts_response = self.fireblocks_client.list_contracts().await?;
                        let contracts = contracts_response.contracts();
                        for (_, c) in contracts.iter().enumerate() {
                            for (_, a) in c.assets().iter().enumerate() {
                                if a.address.as_ref().unwrap().eq(&address.to_string())
                                    && a.status.as_ref().unwrap().as_str() == "APPROVED"
                                    && *a.id.as_ref().unwrap() == *asset_id
                                {
                                    self.whitelisted_contracts.insert(address, c.clone());
                                    contract = c;
                                    return Ok(contract.clone());
                                }
                            }
                        }

                        return Ok(contract.clone());
                    }

                    None => Err(FireBlockError::ContractNotFound(address.to_string())),
                }
            }
            None => Err(FireBlockError::AssetIDError(self.chain_id.to_string())),
        }
    }
}
