use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum AssetID {
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
fn asset_id_by_chain() -> HashMap<u64, AssetID> {
    let mut map = HashMap::new();
    map.insert(1, AssetID::ETH);
    map.insert(2, AssetID::GoerliETH);
    map
}

#[test]
fn test_asset_id_by_chain() {
    let asset_id_by_chain = asset_id_by_chain();
    assert_eq!(AssetID::ETH, *asset_id_by_chain.get(&1).unwrap());
}
