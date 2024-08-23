use std::future::Future;

use alloy_primitives::Address;
use eigen_types::operator::OperatorPubKeys;

pub trait OperatorInfoService {
    fn get_operator_info(&self, address: Address) -> impl Future<Output = Option<OperatorPubKeys>>;
}
