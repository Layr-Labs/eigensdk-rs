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

impl OperatorInfoOnChain {
    pub fn new(http_url: &str, bls_apk_registry: Address) -> Self {
        Self {
            http_url: http_url.to_string(),
            bls_apk_registry,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloy::{
        primitives::{aliases::U96, Address},
        providers::WalletProvider,
        sol_types::SolCall,
    };
    use eigen_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
    use eigen_common::{get_provider, get_signer};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{
            get_allocation_manager_address, get_avs_directory_address,
            get_bls_apk_registry_address, get_delegation_manager_address, get_erc20_mock_strategy,
            get_registry_coordinator_address, get_rewards_coordinator_address,
            get_service_manager_address, get_strategy_manager_address, FIRST_PRIVATE_KEY,
            SECOND_ADDRESS, SECOND_PRIVATE_KEY,
        },
        transaction::wait_transaction,
    };
    use eigen_utils::slashing::{
        core::{
            allocationmanager::AllocationManager::{self, OperatorSet},
            delegationmanager::DelegationManager,
        },
        middleware::registrycoordinator::{
            ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            IStakeRegistryTypes::StrategyParams, RegistryCoordinator,
        },
        sdk::mockavsservicemanager::MockAvsServiceManager,
    };

    use crate::{operator_info::OperatorInfoService, operatorsinfo_onchain::OperatorInfoOnChain};

    pub async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;

        ELChainReader::build(
            get_test_logger().clone(),
            delegation_manager_address,
            avs_directory_address,
            rewards_coordinator,
            &http_endpoint,
        )
        .await
        .unwrap()
    }

    pub async fn new_test_writer(http_endpoint: String, private_key: String) -> ELChainWriter {
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;
        let delegation_manager = get_delegation_manager_address(http_endpoint.clone()).await;
        let allocation_manager = get_allocation_manager_address(http_endpoint.clone()).await;
        let contract_delegation_manager =
            DelegationManager::new(delegation_manager, get_provider(&http_endpoint));
        let permission_controller = contract_delegation_manager
            .permissionController()
            .call()
            .await
            .unwrap()
            ._0;
        let registry_coordinator = get_registry_coordinator_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            strategy_manager,
            rewards_coordinator,
            Some(permission_controller),
            Some(allocation_manager),
            registry_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            private_key,
        )
    }

    async fn create_operator_set(http_endpoint: &str, avs_address: Address) {
        let allocation_manager_addr =
            get_allocation_manager_address(http_endpoint.to_string()).await;
        let default_signer = get_signer(FIRST_PRIVATE_KEY, http_endpoint);
        let allocation_manager =
            AllocationManager::new(allocation_manager_addr, default_signer.clone());
        let registry_coordinator_addr =
            get_registry_coordinator_address(http_endpoint.to_string()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.to_string()).await;
        let service_manager =
            MockAvsServiceManager::new(service_manager_address, default_signer.clone());
        service_manager
            .setAppointee(
                default_signer.default_signer_address(),
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::setAVSRegistrarCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        allocation_manager
            .setAVSRegistrar(avs_address, registry_coordinator_addr)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        // Create slashable quorum
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, default_signer.clone());
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 100,
            kickBIPsOfTotalStake: 1000,
        };
        let strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        service_manager
            .setAppointee(
                registry_coordinator_addr,
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::createOperatorSetsCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        let strategy_params = StrategyParams {
            strategy,
            multiplier: U96::from(1),
        };

        contract_registry_coordinator
            .createSlashableStakeQuorum(operator_set_params, U96::from(0), vec![strategy_params], 0)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
    }

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
        register_operator(http_endpoint.clone()).await;
        let operator_info_on_chain =
            OperatorInfoOnChain::new(&http_endpoint, bls_apk_registry_address);
        assert!(operator_info_on_chain
            .get_operator_info(SECOND_ADDRESS)
            .await
            .unwrap()
            .is_some());
    }
}
