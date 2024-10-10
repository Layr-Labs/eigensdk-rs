use crate::error::AvsRegistryError;
use alloy::primitives::{Address, Bytes, FixedBytes, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::Filter;
use ark_ff::Zero;
use async_trait::async_trait;
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, alloy_registry_g2_point_to_g2_affine, BlsG1Point,
    BlsG2Point,
};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::{
    bitmap_to_quorum_ids, bitmap_to_quorum_ids_from_u192, OperatorPubKeys,
};
use eigen_utils::{
    get_provider, get_ws_provider, NEW_PUBKEY_REGISTRATION_EVENT,
    {
        blsapkregistry::BLSApkRegistry, operatorstateretriever::OperatorStateRetriever,
        registrycoordinator::RegistryCoordinator, stakeregistry::StakeRegistry,
    },
};
use num_bigint::BigInt;
use std::fmt::Debug;
use std::{collections::HashMap, str::FromStr};

/// Avs Registry chainreader
#[derive(Debug, Clone)]
pub struct AvsRegistryChainReader {
    logger: SharedLogger,
    bls_apk_registry_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever: Address,
    stake_registry_addr: Address,
    provider: String,
}

#[async_trait]
/// Common methods for AvsRegistryChainReader
pub trait AvsRegistryReader {
    /// Get operators stake in quorums at a particular block
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number.
    /// * `quorum_numbers` - The list of quorum numbers.
    ///
    /// # Returns
    ///
    /// A vector of operators, containing each operator's address, id and stake.
    async fn get_operators_stake_in_quorums_at_block(
        &self,
        block_number: u32,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError>;

    /// Get signature indices
    ///
    /// # Arguments
    ///
    /// * `reference_block_number` - The block number.
    /// * `quorum_numbers` - The list of quorum numbers.
    /// * `non_signer_operator_ids` -  The list of non-signer operator ids.
    ///
    /// # Returns
    ///
    /// A struct containing the indices of the quorum members that signed,
    /// and the ones that didn't
    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<OperatorStateRetriever::CheckSignaturesIndices, AvsRegistryError>;

    /// Get operator from operator id
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id.
    ///
    /// # Returns
    ///
    /// The operator address.
    async fn get_operator_from_id(
        &self,
        operator_id: [u8; 32],
    ) -> Result<Address, AvsRegistryError>;
}

#[async_trait]
impl AvsRegistryReader for AvsRegistryChainReader {
    async fn get_operators_stake_in_quorums_at_block(
        &self,
        block_number: u32,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_operator_state_retriever =
            OperatorStateRetriever::new(self.operator_state_retriever, provider);
        let operator_state = contract_operator_state_retriever
            .getOperatorState_0(self.registry_coordinator_addr, quorum_numbers, block_number)
            .call()
            .await
            .map_err(|_| AvsRegistryError::GetOperatorState)?;

        let OperatorStateRetriever::getOperatorState_0Return { _0: quorum } = operator_state;
        Ok(quorum)
    }

    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<OperatorStateRetriever::CheckSignaturesIndices, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_operator_state_retriever =
            OperatorStateRetriever::new(self.operator_state_retriever, provider);

        let check_signature_indices = contract_operator_state_retriever
            .getCheckSignaturesIndices(
                self.registry_coordinator_addr,
                reference_block_number,
                quorum_numbers.into(),
                non_signer_operator_ids,
            )
            .call()
            .await?;
        let OperatorStateRetriever::getCheckSignaturesIndicesReturn { _0: indices } =
            check_signature_indices;
        Ok(indices)
    }

    async fn get_operator_from_id(
        &self,
        operator_id: [u8; 32],
    ) -> Result<Address, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, &provider);

        let operator_address_return = contract_registry_coordinator
            .getOperatorFromId(operator_id.into())
            .call()
            .await?;
        let RegistryCoordinator::getOperatorFromIdReturn {
            _0: operator_address,
        } = operator_address_return;

        Ok(operator_address)
    }
}

impl AvsRegistryChainReader {
    /// Create a new instance of the AvsRegistryChainReader
    ///
    /// # Arguments
    ///
    /// * `logger` - A reference to the logger.
    /// * `registry_coordinator_addr` - The address of the RegistryCoordinator contract.
    /// * `operator_state_retriever_addr` - The address of the OperatorStateRetriever contract.
    /// * `http_provider_url` - The http provider url.
    pub async fn new(
        logger: SharedLogger,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        http_provider_url: String,
    ) -> Result<AvsRegistryChainReader, AvsRegistryError> {
        let provider = get_provider(&http_provider_url);

        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, &provider);
        let bls_apk_registry_return = contract_registry_coordinator
            .blsApkRegistry()
            .call()
            .await
            .map_err(|_| AvsRegistryError::GetBlsApkRegistry)?;

        let RegistryCoordinator::blsApkRegistryReturn {
            _0: bls_apk_registry_addr,
        } = bls_apk_registry_return;

        let stake_registry_return = contract_registry_coordinator
            .stakeRegistry()
            .call()
            .await
            .map_err(|_| AvsRegistryError::GetStakeRegistry)?;

        let RegistryCoordinator::stakeRegistryReturn {
            _0: stake_registry_addr,
        } = stake_registry_return;

        Ok(AvsRegistryChainReader {
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            operator_state_retriever: operator_state_retriever_addr,
            stake_registry_addr,
            provider: http_provider_url.clone(),
        })
    }

    /// Get quorum count
    ///
    /// # Returns
    ///
    /// The total quorum count read from the RegistryCoordinator.
    pub async fn get_quorum_count(&self) -> Result<u8, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let quorum_count = contract_registry_coordinator
            .quorumCount()
            .call()
            .await
            .map_err(|_| AvsRegistryError::GetQuorumCount)?;

        let RegistryCoordinator::quorumCountReturn { _0: quorum } = quorum_count;
        Ok(quorum)
    }

    /// Get operators stake in quorums at block operator id
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number.
    /// * `operator_id` - The operator id.
    ///
    /// # Returns
    ///
    /// - a bitmap of the quorums the operator was registered for at `block_number`.
    /// - for each of the quorums mentioned above, a vector of the operators registered for
    ///   that quorum at `block_number`, containing each operator's `operatorId` and `stake`.
    pub async fn get_operators_stake_in_quorums_at_block_operator_id(
        &self,
        block_number: u32,
        operator_id: B256,
    ) -> Result<(U256, Vec<Vec<OperatorStateRetriever::Operator>>), AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_operator_state_retriever =
            OperatorStateRetriever::new(self.operator_state_retriever, provider);

        let operator_state_with_registry_coordinator_and_operator_id =
            contract_operator_state_retriever
                .getOperatorState_1(self.registry_coordinator_addr, operator_id, block_number)
                .call()
                .await
                .map_err(|_| {
                    AvsRegistryError::GetOperatorStateWithRegistryCoordinatorAndOperatorId
                })?;

        let OperatorStateRetriever::getOperatorState_1Return {
            _0: stake,
            _1: operator_state,
        } = operator_state_with_registry_coordinator_and_operator_id;
        Ok((stake, operator_state))
    }

    /// Get operators stake in quorums at current block
    ///
    /// # Arguments
    ///
    /// * `quorum_numbers` - The list of quorum numbers.
    ///
    /// # Returns
    ///
    /// For each quorum in `quorum_numbers`, a vector of the operators registered for
    /// that quorum at the current block, containing each operator's `operatorId` and `stake`.
    pub async fn get_operators_stake_in_quorums_at_current_block(
        &self,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let current_block_number = provider
            .get_block_number()
            .await
            .map_err(|_| AvsRegistryError::GetBlockNumber)?;

        if current_block_number > u32::MAX.into() {
            return Err(AvsRegistryError::BlockNumberOverflow);
        }

        self.get_operators_stake_in_quorums_at_block(current_block_number as u32, quorum_numbers)
            .await
            .map_err(|_| AvsRegistryError::GetOperatorStakeInQuorumAtBlockNumber)
    }

    /// Get operators stake in quorums of operator at block
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id.
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// - a vector of the quorum numbers the operator was registered for at `block_number`.
    /// - for each of the quorums mentioned above, a vector of the operators registered for
    ///   that quorum at `block_number`, containing each operator's `operatorId` and `stake`.

    pub async fn get_operators_stake_in_quorums_of_operator_at_block(
        &self,
        operator_id: B256,
        block_number: u32,
    ) -> Result<(Vec<u8>, Vec<Vec<OperatorStateRetriever::Operator>>), AvsRegistryError> {
        let (quorum_bitmaps, operator_stakes) = self
            .get_operators_stake_in_quorums_at_block_operator_id(block_number, operator_id)
            .await
            .map_err(|_| AvsRegistryError::GetOperatorStakeInQuorumAtBlockOperatorId)?;

        let quorums = bitmap_to_quorum_ids(quorum_bitmaps);
        let s = (quorums, operator_stakes);
        Ok(s)
    }

    /// Get operators stake in quorums of operator at current block
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id.
    ///
    /// # Returns
    ///
    /// - a vector of the quorum numbers the operator was registered for at the current block.
    /// - for each of the quorums mentioned above, a vector of the operators registered for
    ///   that quorum at the current block, containing each operator's `operatorId` and `stake`.
    pub async fn get_operators_stake_in_quorums_of_operator_at_current_block(
        &self,
        operator_id: B256,
    ) -> Result<(Vec<u8>, Vec<Vec<OperatorStateRetriever::Operator>>), AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let current_block_number = provider.get_block_number().await.map_err(|e| {
            AvsRegistryError::AlloyContractError(alloy::contract::Error::TransportError(e))
        })?;

        if current_block_number > u32::MAX.into() {
            return Err(AvsRegistryError::BlockNumberOverflow);
        }

        self.get_operators_stake_in_quorums_of_operator_at_block(
            operator_id,
            current_block_number as u32,
        )
        .await
    }

    /// Get operator's stake in quorums at current block
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id.
    ///
    /// # Returns
    ///
    /// A hashmap containing the quorum numbers that the operator is registered for,
    /// and the amount staked on each quorum.
    pub async fn get_operator_stake_in_quorums_of_operator_at_current_block(
        &self,
        operator_id: B256,
    ) -> Result<HashMap<u8, BigInt>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, &provider);

        let quorum_bitmap = registry_coordinator
            .getCurrentQuorumBitmap(operator_id)
            .call()
            .await?;

        let RegistryCoordinator::getCurrentQuorumBitmapReturn { _0: quo } = quorum_bitmap;

        let quorums = bitmap_to_quorum_ids_from_u192(quo);

        let mut quorum_stakes: HashMap<u8, BigInt> = HashMap::new();

        let stake_registry = StakeRegistry::new(self.stake_registry_addr, &provider);
        for quorum in quorums.iter() {
            let stakes_result = stake_registry
                .getCurrentStake(operator_id, *quorum)
                .call()
                .await?;

            let StakeRegistry::getCurrentStakeReturn { _0: c_stake } = stakes_result;
            quorum_stakes.insert(
                *quorum,
                BigInt::from_str(&U256::from(c_stake).to_string())
                    .map_err(|_| AvsRegistryError::ParseBigIntError)?,
            );
        }
        Ok(quorum_stakes)
    }

    /// Get operator id
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator address.
    ///
    /// # Returns
    ///
    /// The operator id.
    pub async fn get_operator_id(
        &self,
        operator_address: Address,
    ) -> Result<FixedBytes<32>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let operator_id_return = contract_registry_coordinator
            .getOperatorId(operator_address)
            .call()
            .await?;
        let RegistryCoordinator::getOperatorIdReturn { _0: operator_id } = operator_id_return;
        Ok(operator_id)
    }

    /// Check if operator is registered
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator address.
    ///
    /// # Returns
    ///
    /// True if the operator is registered, false otherwise.
    pub async fn is_operator_registered(
        &self,
        operator_address: Address,
    ) -> Result<bool, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let operator_status_return = contract_registry_coordinator
            .getOperatorStatus(operator_address)
            .call()
            .await?;

        let RegistryCoordinator::getOperatorStatusReturn {
            _0: operator_status,
        } = operator_status_return;

        Ok(operator_status == 1)
    }

    /// Queries existing operators from for a particular block range.
    ///
    /// # Arguments
    ///
    /// * `start_block` - The block number to start querying from.
    /// * `stop_block` - The block number to stop querying at.
    /// * `ws_url` - The websocket url to use for querying.
    ///
    /// # Returns
    ///
    /// * (`Vec<Address>`, `Vec<OperatorPubKeys>`) - A vector of operator addresses and its
    ///   corresponding operator pub keys.
    pub async fn query_existing_registered_operator_pub_keys(
        &self,
        start_block: u64,
        mut stop_block: u64,
        ws_url: String,
    ) -> Result<(Vec<Address>, Vec<OperatorPubKeys>), AvsRegistryError> {
        let provider = get_ws_provider(&ws_url).await.map_err(|e| {
            AvsRegistryError::AlloyContractError(alloy::contract::Error::TransportError(e))
        })?;

        let query_block_range = 1024;
        let current_block_number = provider.get_block_number().await.map_err(|e| {
            AvsRegistryError::AlloyContractError(alloy::contract::Error::TransportError(e))
        })?;
        if stop_block.is_zero() {
            stop_block = current_block_number;
        }
        let mut i = start_block;
        let mut operator_addresses: Vec<Address> = vec![];
        let mut operator_pub_keys: Vec<OperatorPubKeys> = vec![];

        while i <= stop_block {
            let to_block = std::cmp::min(i + (query_block_range - 1), stop_block);
            let filter = Filter::new()
                .from_block(i)
                .to_block(to_block)
                .event(NEW_PUBKEY_REGISTRATION_EVENT)
                .address(self.bls_apk_registry_addr);

            let logs = provider.get_logs(&filter).await.map_err(|e| {
                AvsRegistryError::AlloyContractError(alloy::contract::Error::TransportError(e))
            })?;

            let len = logs.len();
            self.logger.debug(
                &format!("numTransactionLogs: {len}, fromBlock: {i}, toBlock: {to_block}",),
                "eigen-client-avsregistry.reader.query_existing_registered_operator_pub_keys",
            );

            for pub_key_reg in logs
                .iter()
                .map(|v| v.log_decode::<BLSApkRegistry::NewPubkeyRegistration>())
                .filter_map(Result::ok)
            {
                let data = pub_key_reg.data();
                let operator_addr = data.operator;
                operator_addresses.push(operator_addr);
                let g1_pub_key = data.pubkeyG1.clone();
                let g2_pub_key = data.pubkeyG2.clone();
                let operator_pub_key = OperatorPubKeys {
                    g1_pub_key: BlsG1Point::new(alloy_registry_g1_point_to_g1_affine(g1_pub_key)),
                    g2_pub_key: BlsG2Point::new(alloy_registry_g2_point_to_g2_affine(g2_pub_key)),
                };
                operator_pub_keys.push(operator_pub_key);
            }
            i += query_block_range;
        }

        Ok((operator_addresses, operator_pub_keys))
    }

    /// TODO: Update bindings and then update this function
    /// Query existing operator sockets
    ///
    /// # Arguments
    ///
    /// * `start_block` - Start block number
    /// * `stop_block` - Stop block number. If zero is passed, then the current block number is fetched and used.
    ///
    /// # Returns
    ///
    /// * `HashMap<FixedBytes<32>, String>` - Operator Id to socket mapping containing all the operator
    ///   sockets registered in the given block range
    pub async fn query_existing_registered_operator_sockets(
        &self,
        start_block: u64,
        stop_block: u64,
    ) -> Result<HashMap<FixedBytes<32>, String>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let mut operator_id_to_socket = HashMap::new();

        let query_block_range = 10000;

        let stop_block = if stop_block == 0 {
            provider.get_block_number().await.map_err(|e| {
                AvsRegistryError::AlloyContractError(alloy::contract::Error::TransportError(e))
            })?
        } else {
            stop_block
        };

        for from_block in (start_block..=stop_block).step_by(query_block_range as usize) {
            let to_block = (from_block + query_block_range - 1).min(stop_block);

            let filter = Filter::new()
                .from_block(from_block)
                .to_block(to_block)
                .event("OperatorSocketUpdate(bytes32,string)")
                .address(self.registry_coordinator_addr);

            let logs = provider.get_logs(&filter).await.map_err(|e| {
                AvsRegistryError::AlloyContractError(alloy::contract::Error::TransportError(e))
            })?;

            for v_log in logs.iter() {
                let socket_update_filter_option = v_log
                    .log_decode::<RegistryCoordinator::OperatorSocketUpdate>()
                    .ok();
                if let Some(socket_update_filter) = socket_update_filter_option {
                    let data = socket_update_filter.data();
                    let operator_id = data.operatorId;
                    let socket = &data.socket;
                    operator_id_to_socket.insert(operator_id, socket.clone());
                }
            }
            let len = logs.len();
            self.logger.debug(
                &format!("num_transaction_logs : {len} , from_block: {from_block} , to_block: {to_block}"),
                "eigen-client-avsregistry.reader.query_existing_registered_operator_sockets",
            );
        }
        Ok(operator_id_to_socket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eigen_logging::get_test_logger;
    use hex::FromHex;
    use std::str::FromStr;
    const HOLESKY_REGISTRY_COORDINATOR: &str = "0x53012C69A189cfA2D9d29eb6F19B32e0A2EA3490";
    const HOLESKY_OPERATOR_STATE_RETRIEVER: &str = "0xB4baAfee917fb4449f5ec64804217bccE9f46C67";

    async fn build_avs_registry_chain_reader() -> AvsRegistryChainReader {
        let holesky_registry_coordinator =
            Address::from_str(HOLESKY_REGISTRY_COORDINATOR).expect("failed to parse address");
        let holesky_operator_state_retriever =
            Address::from_str(HOLESKY_OPERATOR_STATE_RETRIEVER).expect("failed to parse address");

        let holesky_provider = "https://ethereum-holesky.blockpi.network/v1/rpc/public";
        AvsRegistryChainReader::new(
            get_test_logger(),
            holesky_registry_coordinator,
            holesky_operator_state_retriever,
            holesky_provider.to_string(),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_get_quorum_count() {
        let avs_reader = build_avs_registry_chain_reader().await;

        let _ = avs_reader.get_quorum_count().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_at_block() {
        let avs_reader = build_avs_registry_chain_reader().await;

        let quorum_number = Bytes::from_hex("0x00").expect("bytes parse");
        let _ = avs_reader
            .get_operators_stake_in_quorums_at_block(1245063, quorum_number)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_at_block_operator_id() {
        let avs_reader = build_avs_registry_chain_reader().await;

        let operator_id = U256::from_str(
            "35344093966194310405039483339636912150346494903629410125452342281826147822033",
        )
        .unwrap();

        let _ = avs_reader
            .get_operators_stake_in_quorums_at_block_operator_id(1245842, operator_id.into())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_at_current_block() {
        let avs_reader = build_avs_registry_chain_reader().await;
        let quorum_number = Bytes::from_hex("0x00").expect("bytes parse");

        let _ = avs_reader
            .get_operators_stake_in_quorums_at_current_block(quorum_number)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_of_operator_at_current_block() {
        let avs_reader = build_avs_registry_chain_reader().await;
        let operator_id = U256::from_str(
            "35344093966194310405039483339636912150346494903629410125452342281826147822033",
        )
        .unwrap();

        let (quorums, operators) = avs_reader
            .get_operators_stake_in_quorums_of_operator_at_current_block(operator_id.into())
            .await
            .unwrap();
        assert_eq!(quorums.len(), 0);
        assert_eq!(operators.len(), 0);
    }

    #[tokio::test]
    async fn test_get_operator_stake_in_quorums_of_operator_at_current_block() {
        let avs_reader = build_avs_registry_chain_reader().await;
        let operator_id = U256::from_str(
            "35344093966194310405039483339636912150346494903629410125452342281826147822033",
        )
        .unwrap();

        let stakes = avs_reader
            .get_operator_stake_in_quorums_of_operator_at_current_block(operator_id.into())
            .await
            .unwrap();
        assert_eq!(stakes.len(), 0);
    }

    #[tokio::test]
    async fn test_is_operator_registered() {
        let avs_reader = build_avs_registry_chain_reader().await;
        let address = Address::from_str(HOLESKY_REGISTRY_COORDINATOR).unwrap();

        let is_registered = avs_reader.is_operator_registered(address).await.unwrap();
        assert!(!is_registered);
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_of_operator_at_block() {
        let avs_reader = build_avs_registry_chain_reader().await;

        let operator_id = U256::from_str(
            "35344093966194310405039483339636912150346494903629410125452342281826147822033",
        )
        .unwrap();

        let _ = avs_reader
            .get_operators_stake_in_quorums_of_operator_at_block((operator_id).into(), 1246078)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_query_existing_registered_operator_sockets() {
        let avs_reader = build_avs_registry_chain_reader().await;

        let _ = avs_reader
            .query_existing_registered_operator_sockets(0, 1000)
            .await
            .unwrap();
    }
}
