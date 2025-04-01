use crate::{operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceError};
use alloy::primitives::Address;
use async_trait::async_trait;
use eigen_common::get_provider;
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, alloy_registry_g2_point_to_g2_affine, BlsG1Point,
    BlsG2Point,
};
use eigen_types::operator::OperatorPubKeys;
use eigen_utils::slashing::middleware::{blsapkregistry, socketregistry};

/// Retrieves operator's [`OperatorPubKeys`] directly from middleware.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperatorInfoOnChain {
    http_url: String,
    bls_apk_registry: Address,
    socket_registry: Address,
}

#[async_trait]
impl OperatorInfoService for OperatorInfoOnChain {
    async fn get_operator_info(
        &self,
        address: Address,
    ) -> Result<Option<OperatorPubKeys>, OperatorInfoServiceError> {
        let contract_bls_apk_registry = blsapkregistry::BLSApkRegistry::new(
            self.bls_apk_registry,
            get_provider(&self.http_url),
        );
        let g1_point = contract_bls_apk_registry
            .getRegisteredPubkey(address)
            .call()
            .await?
            ._0;
        let g2_point = contract_bls_apk_registry
            .getOperatorPubkeyG2(address)
            .call()
            .await?
            ._0;
        let bls_g1_point = BlsG1Point::new(alloy_registry_g1_point_to_g1_affine(g1_point));
        let bls_g2_point = BlsG2Point::new(alloy_registry_g2_point_to_g2_affine(g2_point));
        Ok(Some(OperatorPubKeys {
            g1_pub_key: bls_g1_point,
            g2_pub_key: bls_g2_point,
        }))
    }

    async fn get_operator_socket(
        &self,
        address: Address,
    ) -> Result<Option<String>, OperatorInfoServiceError> {
        // todo!()
        let contract_socket_registry =
            socketregistry::SocketRegistry::new(self.socket_registry, get_provider(&self.http_url));
        let contract_bls_apk_registry = blsapkregistry::BLSApkRegistry::new(
            self.bls_apk_registry,
            get_provider(&self.http_url),
        );

        let operator_id = contract_bls_apk_registry
            .getOperatorId(address)
            .call()
            .await?
            ._0;
        let socket = contract_socket_registry
            .getOperatorSocket(operator_id)
            .call()
            .await?
            ._0;
        if socket == "" {
            Ok(None)
        } else {
            Ok(Some(socket))
        }
    }
}

impl OperatorInfoOnChain {
    pub fn new(http_url: &str, bls_apk_registry: Address, socket_registry: Address) -> Self {
        Self {
            http_url: http_url.to_string(),
            bls_apk_registry,
            socket_registry,
        }
    }
}

#[cfg(test)]
mod tests {
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{
            get_bls_apk_registry_address, get_service_manager_address, get_socket_registry_address,
            SECOND_ADDRESS, SECOND_PRIVATE_KEY,
        },
        chain_clients::{build_el_chain_reader, create_operator_set, new_test_writer},
        transaction::wait_transaction,
    };
    use eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet;

    use crate::{operator_info::OperatorInfoService, operatorsinfo_onchain::OperatorInfoOnChain};

    async fn register_operator(http_endpoint: String) {
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let operator_set_id = 0;
        create_operator_set(http_endpoint.as_str(), avs_address).await;

        let operator_addr = SECOND_ADDRESS;
        let operator_private_key = SECOND_PRIVATE_KEY;
        let el_chain_writer =
            new_test_writer(http_endpoint.clone(), operator_private_key.to_string()).await;
        let bls_key = BlsKeyPair::new("1".to_string()).unwrap();

        let tx_hash = el_chain_writer
            .register_for_operator_sets(
                operator_addr,
                avs_address,
                vec![operator_set_id],
                bls_key,
                "socket",
            )
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let operator_set = OperatorSet {
            avs: avs_address,
            id: operator_set_id,
        };
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;

        let is_registered = el_chain_reader
            .is_operator_registered_with_operator_set(operator_addr, operator_set.clone())
            .await
            .unwrap();
        assert!(is_registered);

        let tx_hash = el_chain_writer
            .deregister_from_operator_sets(operator_addr, avs_address, vec![operator_set_id])
            .await
            .unwrap();
        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        let is_registered = el_chain_reader
            .is_operator_registered_with_operator_set(operator_addr, operator_set.clone())
            .await
            .unwrap();
        assert!(!is_registered);
    }

    #[tokio::test]
    async fn test_get_operator_info() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let bls_apk_registry_address = get_bls_apk_registry_address(http_endpoint.clone()).await;
        let socket_registry_address = get_socket_registry_address(http_endpoint.clone()).await;
        register_operator(http_endpoint.clone()).await;
        let operator_info_on_chain = OperatorInfoOnChain::new(
            &http_endpoint,
            bls_apk_registry_address,
            socket_registry_address,
        );
        assert!(operator_info_on_chain
            .get_operator_info(SECOND_ADDRESS)
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn test_get_operator_socket() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let bls_apk_registry_address = get_bls_apk_registry_address(http_endpoint.clone()).await;
        let socket_registry_address = get_socket_registry_address(http_endpoint.clone()).await;
        register_operator(http_endpoint.clone()).await;
        let operator_info_on_chain = OperatorInfoOnChain::new(
            &http_endpoint,
            bls_apk_registry_address,
            socket_registry_address,
        );
        let socket = operator_info_on_chain
            .get_operator_socket(SECOND_ADDRESS)
            .await
            .unwrap()
            .unwrap();
        let expected_socket = "socket";
        assert_eq!(socket, expected_socket);
    }
}
