use crate::{client::AssetID, status::Status};
use alloy_primitives::Address;

#[derive(Debug)]
pub struct Assets {
    id: AssetID,
    status: Status,
    address: Address,
    tag: String,
}

#[derive(Debug)]
pub struct WhitelistedContract {
    id: String,
    name: String,
    assets: Assets,
}
