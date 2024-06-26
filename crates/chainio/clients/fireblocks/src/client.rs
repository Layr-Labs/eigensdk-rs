use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssetID {
    ETH,
    GoerliETH,
}

impl std::fmt::Display for AssetID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetID::ETH => "ETH",
                AssetID::GoerliETH => "GoerliETH",
            }
        )
    }
}

// Initialize AssetIDByChain as a HashMap
#[allow(unused)]
fn asset_id_by_chain() -> HashMap<u64, AssetID> {
    let mut map = HashMap::new();
    map.insert(1, AssetID::ETH);
    map.insert(2, AssetID::GoerliETH);
    map
}

#[allow(unused)]
#[allow(non_camel_case_types)]
struct client {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub code: i32,
}

#[test]
fn test_asset_id_by_chain() {
    let asset_id_by_chain = asset_id_by_chain();
    assert_eq!(AssetID::ETH, *asset_id_by_chain.get(&1).unwrap());
}
