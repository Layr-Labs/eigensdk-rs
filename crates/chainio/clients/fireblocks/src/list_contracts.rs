use crate::{client::AssetID, status::Status};
use alloy_primitives::Address;

#[allow(unused)]
#[derive(Debug)]
pub struct Assets {
    id: AssetID,
    status: Status,
    address: Address,
    tag: String,
}

#[allow(unused)]
#[derive(Debug)]
pub struct WhitelistedContract {
    id: String,
    name: String,
    assets: Assets,
}
