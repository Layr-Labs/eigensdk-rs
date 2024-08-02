use crate::args::Args;
use crate::EigenAddressCliError;
use alloy_contract::Error as ContractError;
use alloy_primitives::Address;
use alloy_provider::Provider;
use eigen_utils::{
    binding::{DelegationManager, IBLSSignatureChecker, RegistryCoordinator},
    get_provider,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "kebab-case"))]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct EigenAddresses {
    delegation_manager: Address,
    slasher: Address,
    strategy_manager: Address,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EigenAddressesResponse {
    avs: AvsAddresses,
    eigenlayer: EigenAddresses,
    network: NetworkInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "kebab-case"))]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct NetworkInfo {
    chain_id: String,
    rpc_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "kebab-case"))]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct AvsAddresses {
    bls_apk_registry: Address,
    index_registry: Address,
    registry_coordinator: Address,
    service_manager: Address,
    stake_registry: Address,
}

impl EigenAddresses {
    /// Public function to get the Eigenlayer and AVS contract addresses.
    ///
    /// # Arguments
    ///
    /// * `args` - The command line arguments.
    ///
    /// # Returns
    ///
    /// * `EigenAddressesResponse` - The Eigenlayer and AVS contract addresses.
    pub async fn get_addresses(args: Args) -> Result<EigenAddressesResponse, EigenAddressCliError> {
        let rpc_url = args.rpc_url.clone();
        let client = get_provider(&rpc_url);
        let chain_id = client
            .get_chain_id()
            .await
            .map_err(|e| EigenAddressCliError::RpcError(e))?
            .to_string();
        let (registry_coord_addr, service_manager_addr) =
            EigenAddresses::get_registry_coord_and_service_manager_addr(args, client.clone())
                .await?;

        let avs = EigenAddresses::get_avs_contract_addresses(registry_coord_addr, client.clone())
            .await
            .map_err(|e| EigenAddressCliError::ContractError(e))?;

        let eigenlayer =
            EigenAddresses::get_eigenlayer_contract_addresses(service_manager_addr, client)
                .await
                .map_err(|e| EigenAddressCliError::ContractError(e))?;

        let network = NetworkInfo { rpc_url, chain_id };
        Ok(EigenAddressesResponse {
            network,
            eigenlayer,
            avs,
        })
    }

    /// Get the registry coordinator and service manager contract addresses.
    ///
    /// # Arguments
    ///
    /// * `args` - The command line arguments.
    /// * `client` - The provider client.
    ///
    /// # Returns
    ///
    /// * `(Address, Address)` - The registry coordinator and service manager contract addresses,
    /// used to call `get_avs_contract_addresses` and `get_eigenlayer_contract_addresses` functions.
    async fn get_registry_coord_and_service_manager_addr<T, P, N>(
        args: Args,
        client: P,
    ) -> Result<(Address, Address), EigenAddressCliError>
    where
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    {
        match (args.registry_coordinator, args.service_manager) {
            (Some(registry_coord_addr), _) => {
                let registry_coordinator = RegistryCoordinator::new(registry_coord_addr, &client);
                let service_manager_addr = registry_coordinator
                    .serviceManager()
                    .call()
                    .await
                    .map_err(|e| EigenAddressCliError::ContractError(e))?
                    ._0;
                Ok((registry_coord_addr, service_manager_addr))
            }
            (_, Some(service_manager_addr)) => {
                let service_manager = IBLSSignatureChecker::new(service_manager_addr, client);
                let registry_coord_addr = service_manager
                    .registryCoordinator()
                    .call()
                    .await
                    .map_err(|e| EigenAddressCliError::ContractError(e))?
                    ._0;
                Ok((registry_coord_addr, service_manager_addr))
            }
            _ => unreachable!(),
        }
    }

    /// Get the Eigenlayer contract addresses.
    ///
    /// # Arguments
    ///
    /// * `service_manager_addr` - The service manager contract address.
    /// * `client` - The provider client.
    ///
    /// # Returns
    ///
    /// * `EigenAddresses` - The Eigenlayer contract addresses.
    async fn get_eigenlayer_contract_addresses<T, P, N>(
        service_manager_addr: Address,
        client: P,
    ) -> Result<EigenAddresses, ContractError>
    where
        P: alloy_contract::private::Provider<T, N>,
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        N: alloy_contract::private::Network,
    {
        let service_manager = IBLSSignatureChecker::new(service_manager_addr, &client);
        let delegation_manager = service_manager.delegation().call().await?._0;
        let delegation_manager_client = DelegationManager::new(delegation_manager, &client);
        let slasher = delegation_manager_client.slasher().call().await?._0;
        let strategy_manager = delegation_manager_client.strategyManager().call().await?._0;

        Ok(EigenAddresses {
            slasher,
            delegation_manager,
            strategy_manager,
        })
    }

    /// Get the AVS contract addresses.
    ///
    /// # Arguments
    ///
    /// * `registry_coordinator` - The registry coordinator contract address.
    /// * `client` - The provider client.
    ///
    /// # Returns
    ///
    /// * `AvsAddresses` - The AVS contract addresses.
    async fn get_avs_contract_addresses<T, P, N>(
        registry_coordinator: Address,
        client: P,
    ) -> Result<AvsAddresses, ContractError>
    where
        P: alloy_contract::private::Provider<T, N>,
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        N: alloy_contract::private::Network,
    {
        let registry_coordinator_instance = RegistryCoordinator::new(registry_coordinator, &client);
        let service_manager = registry_coordinator_instance
            .serviceManager()
            .call()
            .await?
            ._0;
        let bls_apk_registry = registry_coordinator_instance
            .blsApkRegistry()
            .call()
            .await?
            ._0;
        let index_registry = registry_coordinator_instance
            .indexRegistry()
            .call()
            .await?
            ._0;
        let stake_registry = registry_coordinator_instance
            .stakeRegistry()
            .call()
            .await?
            ._0;

        Ok(AvsAddresses {
            service_manager,
            registry_coordinator,
            bls_apk_registry,
            index_registry,
            stake_registry,
        })
    }
}
