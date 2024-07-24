// use crate::client::{AssetID, Client};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct Asset {
//     id: AssetID,
//     total: String,
//     balance: String,
//     available: String,
// }

// #[allow(unused)]
// #[derive(Debug, Serialize, Deserialize)]
// struct VaultAccount {
//     id: String,
//     name: String,
//     assets: Vec<Asset>,
// }

// #[derive(Debug,Serialize,Deserialize)]
// pub struct Paging{
//     before: String,
//     after: String

// }

// pub trait ListVaultAccounts {
//     async fn list_vault_accounts(&self) -> Vec<VaultAccount>;
// }

// impl ListVaultAccounts for Client {
//     async fn list_vault_accounts(&self) -> Vec<VaultAccount> {

//         // let mut accounts = Vec::new();
//         // let mut paging = Paging{};

//         // let res = self
//         //     .get_request(&format!("/v1/vault/accounts_paged"))
//         //     .await
//         //     .unwrap();

//         // let contracts: Vec<VaultAccount> = serde_json::from_str(&res).unwrap();
//         // contracts
//     }
// }
