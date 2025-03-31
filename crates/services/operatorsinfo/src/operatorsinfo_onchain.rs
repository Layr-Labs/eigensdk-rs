use crate::{operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceError};
use alloy::primitives::Address;
use async_trait::async_trait;
use eigen_common::get_provider;
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, alloy_registry_g2_point_to_g2_affine, BlsG1Point,
    BlsG2Point,
};
use eigen_types::operator::OperatorPubKeys;
use eigen_utils::slashing::middleware::blsapkregistry;

/// Retrieves operator's [`OperatorPubKeys`] directly from middleware.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperatorInfoOnChain {
    http_url: String,
    bls_apk_registry: Address,
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
        _address: Address,
    ) -> Result<Option<String>, OperatorInfoServiceError> {
        Err(OperatorInfoServiceError::NotSupported(
            "get_operator_socket is not supported for OperatorInfoOnChain".to_string(),
        ))
    }
}
