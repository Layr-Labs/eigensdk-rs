// use crate::{
//     client::{AssetID, Client},
//     status::Status,
// };
// use alloy_primitives::Address;
// use serde::{Deserialize, Serialize};

// #[allow(unused)]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Assets {
//     id: AssetID,
//     status: Status,
//     address: Address,
//     tag: String,
// }

// #[allow(unused)]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct WhitelistedContract {
//     id: String,
//     name: String,
//     assets: Assets,
// }

// pub trait ListContracts {
//     async fn list_contracts(&self) -> Vec<WhitelistedContract>;
// }

// impl ListContracts for Client {
//     async fn list_contracts(&self) -> Vec<WhitelistedContract> {
//         let res = self.get_request(&format!("/v1/contracts")).await.unwrap();

//         let contracts: Vec<WhitelistedContract> = serde_json::from_str(&res).unwrap();
//         contracts
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[tokio::test]
//     async fn test_list_contracts() {
//         let api_key = "4aff0abf-9e81-4cf4-8bae-c9fc0c8a13af".to_string();
//         let private_key = "0x29962ee54b8107328d033006e33627ecdbc9f5adc8b3e8ab4064e34a1193110c";
//         let api_url = "https://api.fireblocks.io".to_string();
//         let client = Client::new(
//             api_key.to_string(),
//             private_key.to_string(),
//             api_url.clone(),
//         );

//         let s = client.list_contracts().await;
//     }
// }
