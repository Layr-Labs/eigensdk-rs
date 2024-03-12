use crate::client::AssetID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    id: AssetID,
    total: String,
    balance: String,
    available: String,
}

struct VaultAccount {
    id: String,
    name: String,
    assets: [Asset],
}
