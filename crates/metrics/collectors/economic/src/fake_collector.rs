use std::{collections::HashMap, str::FromStr};

use crate::error::CollectorMetricError;

use alloy_primitives::{Address, FixedBytes, U256};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::OperatorId;
use metrics::{describe_gauge, gauge, Key, Label};
use num_bigint::BigInt;

/// RegisteredStakes Metrics with logger
#[derive(Debug)]
pub struct FakeCollector {
    logger: SharedLogger,
    operator_addr: Address,
    operator_id: OperatorId,
    avs_registry_reader: AvsRegistryChainReader,
    quorum_names: HashMap<u64, String>,
    avs_name: String,
}

impl FakeCollector {
    /// Operator stakes in AVS registry contract.
    /// Most commonly represents a weighted combination of delegated shares in the DelegationManager EigenLayer contract.
    pub fn new(
        logger: SharedLogger,
        operator_addr: Address,
        operator_id: OperatorId,
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

    pub async fn collect(
        &mut self,
        is_operator_frozen: bool,
        quorum_stake_map: HashMap<u8, BigInt>,
    ) -> Result<(), CollectorMetricError> {
        let mut operator_is_frozen_float = 0.0;
        if is_operator_frozen {
            operator_is_frozen_float = 1.0;
        }

        gauge!("e_slashing_status").set(operator_is_frozen_float);

        for (quorum_num, stake) in quorum_stake_map {
            let key = Key::from_parts(
                "eigen_registered_stakes",
                vec![
                    Label::new("quorum_number", quorum_num.to_string()),
                    Label::new(
                        "quorum_name",
                        self.quorum_names[&(quorum_num as u64)].to_string(),
                    ),
                    Label::new("avs_name", self.avs_name.to_string()),
                ],
            );
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
