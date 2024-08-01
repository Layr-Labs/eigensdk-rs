use alloy_contract::Error as ContractError;
use alloy_json_rpc::RpcError;
use alloy_transport::TransportErrorKind;
use thiserror::Error;
pub mod args;
pub mod eigen_address;

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

/// Possible errors raised while trying to get contract addresses
#[derive(Error, Debug)]
pub enum EigenAddressCliError {
    #[error("missing arguments error")]
    MissingArgs(String),
    #[error("contract error")]
    ContractError(ContractError),
    #[error("RPC error")]
    RpcError(RpcError<TransportErrorKind>),
}

#[cfg(test)]
mod test {
    use super::ANVIL_RPC_URL;
    use crate::args::Args;
    use crate::eigen_address::{EigenAddresses, EigenAddressesResponse};
    use eigen_testing_utils::anvil_constants::{
        get_registry_coordinator_address, get_service_manager_address,
    };
    use tokio;

    // TODO: start anvil with the right deployed state
    #[tokio::test]
    async fn egnaddrs_with_service_manager_flag() {
        let service_manager_address = get_service_manager_address().await;

        let args = Args {
            registry_coordinator: None,
            service_manager: Some(service_manager_address),
            rpc_url: ANVIL_RPC_URL.into(),
        };
        let expected_addresses: EigenAddressesResponse = serde_json::from_str(
            r#"{
            "avs": {
              "bls-apk-registry": "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
              "index-registry": "0x851356ae760d987E095750cCeb3bC6014560891C",
              "registry-coordinator": "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9",
              "service-manager": "0x84eA74d481Ee0A5332c457a4d796187F6Ba67fEB",
              "stake-registry": "0xf5059a5D33d5853360D16C683c16e67980206f36"
            },
            "eigenlayer": {
              "delegation-manager": "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9",
              "slasher": "0xa513E6E4b8f2a923D98304ec87F64353C4D5C853",
              "strategy-manager": "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707"
            },
            "network": {
              "chain-id": "31337",
              "rpc-url": "http://localhost:8545"
            }
          }"#,
        )
        .unwrap();
        let addresses = EigenAddresses::get_addresses(args).await.unwrap();

        assert_eq!(expected_addresses, addresses);
    }

    #[tokio::test]
    async fn egnaddrs_with_registry_coordinator_flag() {
        let registry_coordinator_address = get_registry_coordinator_address().await;

        let args = Args {
            registry_coordinator: Some(registry_coordinator_address),
            service_manager: None,
            rpc_url: ANVIL_RPC_URL.into(),
        };
        let expected_addresses: EigenAddressesResponse = serde_json::from_str(
            r#"{
            "avs": {
              "bls-apk-registry": "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
              "index-registry": "0x851356ae760d987E095750cCeb3bC6014560891C",
              "registry-coordinator": "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9",
              "service-manager": "0x84eA74d481Ee0A5332c457a4d796187F6Ba67fEB",
              "stake-registry": "0xf5059a5D33d5853360D16C683c16e67980206f36"
            },
            "eigenlayer": {
              "delegation-manager": "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9",
              "slasher": "0xa513E6E4b8f2a923D98304ec87F64353C4D5C853",
              "strategy-manager": "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707"
            },
            "network": {
              "chain-id": "31337",
              "rpc-url": "http://localhost:8545"
            }
          }"#,
        )
        .unwrap();

        let addresses = EigenAddresses::get_addresses(args).await.unwrap();

        assert_eq!(expected_addresses, addresses);
    }
}
