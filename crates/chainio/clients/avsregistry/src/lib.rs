//! AvsRegistry methods for reading, writing and subscribing purposes.

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// Reader module
pub mod reader;

/// Writer module
pub mod writer;

/// Avs registry error message
pub mod error;

/// Fake avs registry module
pub mod fake_reader;

#[cfg(test)]
pub(crate) mod test_utils {
    use alloy::{
        primitives::{aliases::U96, Address},
        providers::WalletProvider,
        sol_types::SolCall,
    };
    use eigen_common::get_signer;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_erc20_mock_strategy,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_service_manager_address, FIRST_PRIVATE_KEY,
    };
    use eigen_utils::slashing::{
        core::allocationmanager::AllocationManager,
        middleware::registrycoordinator::{
            ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            IStakeRegistryTypes::StrategyParams, RegistryCoordinator,
        },
        sdk::mockavsservicemanager::MockAvsServiceManager,
    };

    use crate::reader::AvsRegistryChainReader;

    pub(crate) const BLS_KEY: &str =
        "1371012690269088913462269866874713266643928125698382731338806296762673180359922";

    pub(crate) async fn create_operator_set(http_endpoint: &str, avs_address: Address) {
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
