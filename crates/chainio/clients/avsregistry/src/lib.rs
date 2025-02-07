//! AvsRegistry methods for reading, writing and subscribing purposes.

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// Reader module
pub mod reader;

#[allow(dead_code)]
/// Writer module
pub mod writer;

/// Avs registry error message
pub mod error;

#[allow(dead_code)]
/// Fake avs registry module
pub mod fake_reader;

#[cfg(test)]
pub(crate) mod test_utils {
    use alloy::primitives::{address, Address};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address,
    };

    use crate::reader::AvsRegistryChainReader;

    pub(crate) const ANVIL_FIRST_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    pub(crate) const ANVIL_SECOND_ADDRESS: Address =
        address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");

    pub(crate) async fn build_avs_registry_chain_reader(
        http_endpoint: String,
    ) -> AvsRegistryChainReader {
        let registry_coordinator_addr =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;

        AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_addr,
            operator_state_retriever_address,
            http_endpoint.to_string(),
        )
        .await
        .unwrap()
    }
}
