#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
use std::{collections::HashMap, str::FromStr};

pub mod error;
pub mod fake_collector;
use alloy_primitives::{Address, FixedBytes, U256};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::OperatorId;
use error::CollectorMetricError;
use metrics::{describe_gauge, gauge, Key, Label};
/// RegisteredStakes Metrics with logger
#[derive(Debug)]
pub struct Collector {
    logger: SharedLogger,
    operator_addr: Address,
    operator_id: OperatorId,
    el_reader: ELChainReader,
    avs_registry_reader: AvsRegistryChainReader,
    quorum_names: HashMap<u64, String>,
    avs_name: String,
}

impl Collector {
    /// Operator stakes in AVS registry contract.
    /// Most commonly represents a weighted combination of delegated shares in the DelegationManager EigenLayer contract.
    pub fn new(
        logger: SharedLogger,
        operator_addr: Address,
        operator_id: OperatorId,
        el_reader: ELChainReader,
        avs_registry_reader: AvsRegistryChainReader,
        quorum_names: HashMap<u64, String>,
        avs_name: &str,
    ) -> Self {
        describe_gauge!(
            "eigen_registered_stakes",
            "A gauge with weighted delegation of delegated shares in delegation manager contract"
        );
        // for now the namespace if "e" instead of eigen as the docs mention to not use the eigen namespace for this metric
        describe_gauge!(
            format!("{},_slashing_status", "e"),
            "Whether the operator has been slashed"
        );

        Self {
            logger,
            operator_addr,
            el_reader,
            avs_registry_reader,
            operator_id,
            quorum_names,
            avs_name: avs_name.to_owned(),
        }
    }

    pub fn set_stake(&self, quorum_number: &str, quorum_name: &str, avs_name: &str, value: f64) {
        // Create the metric key with dynamic
        let key = Key::from_parts(
            "eigen_registered_stakes",
            vec![
                Label::new("quorum_number", quorum_number.to_string()),
                Label::new("quorum_name", quorum_name.to_string()),
                Label::new("avs_name", avs_name.to_string()),
            ],
        );
        gauge!(key.to_string()).set(value);
        self.logger.debug(
            &format!(
            "set registered stakes , quorum_name: {} , quorum_number: {} , avs_name: {}, value: {}",
            quorum_name, quorum_number, avs_name, value
        ),
            "eigen-metrics-collectors-economic.set_stake",
        );
    }

    pub async fn collect(&mut self) -> Result<(), CollectorMetricError> {
        let operator_is_frozen = self
            .el_reader
            .operator_is_frozen(self.operator_addr)
            .await?;

        let mut operator_is_frozen_float = 0.0;
        if operator_is_frozen {
            operator_is_frozen_float = 1.0;
        }

        gauge!("e_slashing_status").set(operator_is_frozen_float);

        self.init_operator_id().await.inspect_err(|e| {
            self.logger.warn(
                &format!("Failed to fetch and cache operator id. Skipping collection of registeredStake metric. , err {}", e),
                "eigen-metrics-collectors-economic.collect"
            );
        })?;
        let quorum_stake_map = self
            .avs_registry_reader
            .get_operator_stake_in_quorums_of_operator_at_current_block(self.operator_id)
            .await?;
        for (quorum_num, stake) in quorum_stake_map {
            let quorum_name_for_quorum_num = self.quorum_names.get(&(quorum_num as u64));
            let key;
            if let Some(quorum_name) = quorum_name_for_quorum_num {
                key = Key::from_parts(
                    "eigen_registered_stakes",
                    vec![
                        Label::new("quorum_number", quorum_num.to_string()),
                        Label::new("quorum_name", quorum_name.to_string()),
                        Label::new("avs_name", self.avs_name.to_string()),
                    ],
                );
            } else {
                key = Key::from_parts(
                    "eigen_registered_stakes",
                    vec![
                        Label::new("quorum_number", quorum_num.to_string()),
                        Label::new("quorum_name", "".to_string()),
                        Label::new("avs_name", self.avs_name.to_string()),
                    ],
                );
            }
            let u256_intermediate = U256::from_str(&stake.to_string())?;
            gauge!(key.to_string()).set(f64::from(u256_intermediate));
        }

        Ok(())
    }

    pub async fn init_operator_id(&mut self) -> Result<(), CollectorMetricError> {
        if self.operator_id.eq(&FixedBytes::default()) {
            let operator_id = self
                .avs_registry_reader
                .get_operator_id(self.operator_addr)
                .await?;

            if operator_id.eq(&FixedBytes::default()) {
                return Err(CollectorMetricError::OperatorNotRegistered);
            }
            self.operator_id = operator_id;
            return Ok(());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use alloy_primitives::{Address, FixedBytes};
    use eigen_client_avsregistry::reader::AvsRegistryChainReader;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{get_operator_state_retriever_address, get_registry_coordinator_address},
    };

    use crate::fake_collector::FakeCollector;

    #[tokio::test]
    async fn test_init_operator_id() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;

        let operator_addr = Address::ZERO;
        let operator_id = FixedBytes::<32>::default();
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            get_registry_coordinator_address(http_endpoint.clone()).await,
            get_operator_state_retriever_address(http_endpoint.clone()).await,
            http_endpoint,
        )
        .await
        .unwrap();

        let quorums_names = HashMap::new();
        let avs_name = "eigensdk-rs";
        let mut collector = FakeCollector::new(
            get_test_logger(),
            operator_addr,
            operator_id,
            avs_registry_reader,
            quorums_names,
            avs_name,
        );

        assert!(collector.init_operator_id().await.is_err());
    }
}
