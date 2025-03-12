use crate::error::AvsRegistryError;
use alloy::primitives::aliases::U96;
use alloy::primitives::{Address, Bytes, FixedBytes, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::Filter;
use ark_ff::Zero;
use async_trait::async_trait;
use eigen_common::{get_provider, get_ws_provider, NEW_PUBKEY_REGISTRATION_EVENT};
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, alloy_registry_g2_point_to_g2_affine, BlsG1Point,
    BlsG2Point,
};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::{
    bitmap_to_quorum_ids, bitmap_to_quorum_ids_from_u192, OperatorPubKeys, QuorumNum,
};
use eigen_utils::slashing::middleware::blsapkregistry::BLSApkRegistry;
use eigen_utils::slashing::middleware::operatorstateretriever::OperatorStateRetriever;
use eigen_utils::slashing::middleware::registrycoordinator::RegistryCoordinator;
use eigen_utils::slashing::middleware::servicemanagerbase::ServiceManagerBase;
use eigen_utils::slashing::middleware::stakeregistry::IStakeRegistryTypes::{
    StakeUpdate, StrategyParams,
};
use eigen_utils::slashing::middleware::stakeregistry::StakeRegistry;
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
        block_number: u64,
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
        reference_block_number: u64,
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
        block_number: u64,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_operator_state_retriever =
            OperatorStateRetriever::new(self.operator_state_retriever, provider);

        let block_number_u32 =
            u32::try_from(block_number).map_err(|_e| AvsRegistryError::BlockNumberOverflow)?;
        let operator_state = contract_operator_state_retriever
            .getOperatorState_0(
                self.registry_coordinator_addr,
                quorum_numbers,
                block_number_u32,
            )
            .call()
            .await
            .map_err(|_| AvsRegistryError::GetOperatorState)?;

        let OperatorStateRetriever::getOperatorState_0Return { _0: quorum } = operator_state;
        Ok(quorum)
    }

    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u64,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<OperatorStateRetriever::CheckSignaturesIndices, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_operator_state_retriever =
            OperatorStateRetriever::new(self.operator_state_retriever, provider);

        let reference_block_number_u32 = u32::try_from(reference_block_number)
            .map_err(|_e| AvsRegistryError::BlockNumberOverflow)?;
        let check_signature_indices = contract_operator_state_retriever
            .getCheckSignaturesIndices(
                self.registry_coordinator_addr,
                reference_block_number_u32,
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
        block_number: u64,
        operator_id: B256,
    ) -> Result<(U256, Vec<Vec<OperatorStateRetriever::Operator>>), AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_operator_state_retriever =
            OperatorStateRetriever::new(self.operator_state_retriever, provider);

        let block_number_u32 =
            u32::try_from(block_number).map_err(|_e| AvsRegistryError::BlockNumberOverflow)?;
        let operator_state_with_registry_coordinator_and_operator_id =
            contract_operator_state_retriever
                .getOperatorState_1(
                    self.registry_coordinator_addr,
                    operator_id,
                    block_number_u32,
                )
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

        self.get_operators_stake_in_quorums_at_block(current_block_number, quorum_numbers)
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
        block_number: u64,
    ) -> Result<(Vec<u8>, Vec<Vec<OperatorStateRetriever::Operator>>), AvsRegistryError> {
        let (quorum_bitmaps, operator_stakes) = self
            .get_operators_stake_in_quorums_at_block_operator_id(block_number, operator_id)
            .await
            .map_err(|_| AvsRegistryError::GetOperatorStakeInQuorumAtBlockOperatorId)?;

        let quorums = bitmap_to_quorum_ids(quorum_bitmaps);
        let s = (quorums, operator_stakes);
        Ok(s)
    }

    /// Get a list of strategies that the AVS supports for restaking
    ///
    /// # Returns
    ///
    /// A vector of addresses representing the strategies that the AVS supports for restaking
    pub async fn get_restakeable_strategies(&self) -> Result<Vec<Address>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, &provider);

        let service_manager = contract_registry_coordinator
            .serviceManager()
            .call()
            .await?
            ._0;

        let strategies = ServiceManagerBase::new(service_manager, &provider)
            .getRestakeableStrategies()
            .call()
            .await?
            ._0;

        Ok(strategies)
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

        self.get_operators_stake_in_quorums_of_operator_at_block(operator_id, current_block_number)
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

    /// Query registration detail
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator address.
    ///
    /// # Returns
    ///
    /// A vector of booleans, where each boolean represents if the operator
    /// is registered for a quorum.
    ///
    /// # Errors
    ///
    /// Returns an error if the operator id cannot be fetched or if the quorum bitmap
    pub async fn query_registration_detail(
        &self,
        operator_address: Address,
    ) -> Result<[bool; 64], AvsRegistryError> {
        let operator_id = self.get_operator_id(operator_address).await?;

        let provider = get_provider(&self.provider);
        let registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, &provider);
        let quorum_bitmap = registry_coordinator
            .getCurrentQuorumBitmap(operator_id)
            .call()
            .await?;

        let inner_value = quorum_bitmap._0.into_limbs()[0];
        let mut quorums: [bool; 64] = [false; 64];
        for i in 0..64_u64 {
            if let Some(value) = quorums.get_mut(i as usize) {
                *value = inner_value & (1 << i) != 0;
            }
        }
        Ok(quorums)
    }

    /// Get a list of strategies that an operator has potentially restaked on the AVS.
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The operator address.
    ///
    /// # Returns
    ///
    /// A vector with strategies addresses
    pub async fn get_operator_restaked_strategies(
        &self,
        operator_address: Address,
    ) -> Result<Vec<Address>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, &provider);

        let service_manager = contract_registry_coordinator
            .serviceManager()
            .call()
            .await?
            ._0;

        let strategies = ServiceManagerBase::new(service_manager, &provider)
            .getOperatorRestakedStrategies(operator_address)
            .call()
            .await?
            ._0;

        Ok(strategies)
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

    /// Computes the total weight of operator with the given quorum number.
    ///
    /// The quorum number must exist, or else the tx will fail.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The respective quorum number.
    /// * `operator_address` - The operator's address.
    ///
    /// # Returns
    ///
    /// * [`U96`] - The total weight.
    pub async fn weight_of_operator_for_quorum(
        &self,
        quorum_number: QuorumNum,
        operator_address: Address,
    ) -> Result<U96, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let stake = contract_stake_registry
            .weightOfOperatorForQuorum(quorum_number, operator_address)
            .call()
            .await?
            ._0;

        Ok(stake)
    }

    /// Returns the length of the strategy parameters stored for a given quorum.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    ///
    /// # Returns
    ///
    /// * [`U256`] - The length of the strategy parameters array.
    pub async fn strategy_params_length(
        &self,
        quorum_number: QuorumNum,
    ) -> Result<U256, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let len = contract_stake_registry
            .strategyParamsLength(quorum_number)
            .call()
            .await?
            ._0;

        Ok(len)
    }

    /// Returns the strategy parameters by index for a given quorum.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    /// * `index` - The index in the strategy parameter array.
    ///
    /// # Returns
    ///
    /// * [`StrategyParams`] - The strategy parameters at the specified index.
    pub async fn strategy_params_by_index(
        &self,
        quorum_number: QuorumNum,
        index: U256,
    ) -> Result<StrategyParams, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let strategy_params = contract_stake_registry
            .strategyParamsByIndex(quorum_number, index)
            .call()
            .await?
            ._0;

        Ok(strategy_params)
    }

    /// Returns the stake history length for a given operator and quorum.
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator's identifier.
    /// * `quorum_number` - The quorum number.
    ///
    /// # Returns
    ///
    /// * [`U256`] - The length of the stake history.
    pub async fn get_stake_history_length(
        &self,
        operator_id: B256,
        quorum_number: QuorumNum,
    ) -> Result<U256, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let len = contract_stake_registry
            .getStakeHistoryLength(operator_id, quorum_number)
            .call()
            .await?
            ._0;

        Ok(len)
    }

    /// Returns the entire stake history for a given operator and quorum.
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator's identifier.
    /// * `quorum_number` - The quorum number.
    ///
    /// # Returns
    ///
    /// * `Vec<StakeUpdate>` - A vector containing all stake updates.
    pub async fn get_stake_history(
        &self,
        operator_id: B256,
        quorum_number: QuorumNum,
    ) -> Result<Vec<StakeUpdate>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake_update_vec = contract_stake_registry
            .getStakeHistory(operator_id, quorum_number)
            .call()
            .await?
            ._0;

        Ok(stake_update_vec)
    }

    /// Returns the most recent stake update for a given operator and quorum.
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator's identifier.
    /// * `quorum_number` - The quorum number.
    ///
    /// # Returns
    ///
    /// * [`StakeUpdate`] - The latest stake update.
    pub async fn get_latest_stake_update(
        &self,
        operator_id: B256,
        quorum_number: QuorumNum,
    ) -> Result<StakeUpdate, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake_update = contract_stake_registry
            .getLatestStakeUpdate(operator_id, quorum_number)
            .call()
            .await?
            ._0;

        Ok(stake_update)
    }

    /// Returns the stake update at a specific index for a given operator and quorum.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    /// * `operator_id` - The operator's identifier.
    /// * `index` - The index in the stake history array.
    ///
    /// # Returns
    ///
    /// * [`StakeUpdate`] - The stake update at the specified index.
    pub async fn get_stake_update_at_index(
        &self,
        quorum_number: QuorumNum,
        operator_id: B256,
        index: U256,
    ) -> Result<StakeUpdate, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake_update = contract_stake_registry
            .getStakeUpdateAtIndex(quorum_number, operator_id, index)
            .call()
            .await?
            ._0;

        Ok(stake_update)
    }

    /// Returns the stake of an operator for a given quorum at a specific block number.
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator's identifier.
    /// * `quorum_number` - The quorum number.
    /// * `block_number` - The block number at which to retrieve the stake.
    ///
    /// # Returns
    ///
    /// * [`U96`] - The stake at the specified block number.
    pub async fn get_stake_at_block_number(
        &self,
        operator_id: B256,
        quorum_number: QuorumNum,
        block_number: u32,
    ) -> Result<U96, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake = contract_stake_registry
            .getStakeAtBlockNumber(operator_id, quorum_number, block_number)
            .call()
            .await?
            ._0;

        Ok(stake)
    }

    /// Returns the index of the stake update for an operator at a given block number.
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator's identifier.
    /// * `quorum_number` - The quorum number.
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// * `u32` - The index of the stake update.
    pub async fn get_stake_update_index_at_block_number(
        &self,
        operator_id: B256,
        quorum_number: QuorumNum,
        block_number: u32,
    ) -> Result<u32, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let index = contract_stake_registry
            .getStakeUpdateIndexAtBlockNumber(operator_id, quorum_number, block_number)
            .call()
            .await?
            ._0;

        Ok(index)
    }

    /// Returns the stake of an operator for a given quorum at a specific block number and index.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    /// * `block_number` - The block number at which to retrieve the stake.
    /// * `operator_id` - The operator's identifier.
    /// * `index` - The index in the stake history array.
    ///
    /// # Returns
    ///
    /// * [`U96`] - The stake weight at the specified block number and index.
    pub async fn get_stake_at_block_number_and_index(
        &self,
        quorum_number: QuorumNum,
        block_number: u32,
        operator_id: B256,
        index: U256,
    ) -> Result<U96, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake_weight = contract_stake_registry
            .getStakeAtBlockNumberAndIndex(quorum_number, block_number, operator_id, index)
            .call()
            .await?
            ._0;

        Ok(stake_weight)
    }

    /// Returns the length of the total stake history for a given quorum.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    ///
    /// # Returns
    ///
    /// * [`U256`] - The total stake history length.
    pub async fn get_total_stake_history_length(
        &self,
        quorum_number: QuorumNum,
    ) -> Result<U256, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let length = contract_stake_registry
            .getTotalStakeHistoryLength(quorum_number)
            .call()
            .await?
            ._0;

        Ok(length)
    }

    /// Returns the current total stake weight for a given quorum.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    ///
    /// # Returns
    ///
    /// * [`U96`] - The current total stake weight.
    pub async fn get_current_total_stake(
        &self,
        quorum_number: QuorumNum,
    ) -> Result<U96, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake_weight = contract_stake_registry
            .getCurrentTotalStake(quorum_number)
            .call()
            .await?
            ._0;

        Ok(stake_weight)
    }

    /// Returns the stake update at a specific index for a given quorum.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    /// * `index` - The index in the total stake history.
    ///
    /// # Returns
    ///
    /// * [`StakeUpdate`] - The stake update at the specified index.
    pub async fn get_total_stake_update_at_index(
        &self,
        quorum_number: QuorumNum,
        index: U256,
    ) -> Result<StakeUpdate, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let stake_update = contract_stake_registry
            .getTotalStakeUpdateAtIndex(quorum_number, index)
            .call()
            .await?
            ._0;

        Ok(stake_update)
    }

    /// Returns the total stake at a given block number and index.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number.
    /// * `block_number` - The block number.
    /// * `index` - The index in the total stake history.
    ///
    /// # Returns
    ///
    /// * [`U96`] - The total stake at the specified block number and index.
    pub async fn get_total_stake_at_block_number_from_index(
        &self,
        quorum_number: QuorumNum,
        block_number: u32,
        index: U256,
    ) -> Result<U96, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let total_stake = contract_stake_registry
            .getTotalStakeAtBlockNumberFromIndex(quorum_number, block_number, index)
            .call()
            .await?
            ._0;

        Ok(total_stake)
    }

    /// Returns the total stake indices for the given quorum at a specific block number.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number.
    /// * `quorum_number` - The quorum number in bytes format.
    ///
    /// # Returns
    ///
    /// * `Vec<u32>` - A vector of stake indices at the specified block number.
    pub async fn get_total_stake_indices_at_block_number(
        &self,
        block_number: u32,
        quorum_number: Bytes,
    ) -> Result<Vec<u32>, AvsRegistryError> {
        let provider = get_provider(&self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let total_stakes = contract_stake_registry
            .getTotalStakeIndicesAtBlockNumber(block_number, quorum_number)
            .call()
            .await?
            ._0;

        Ok(total_stakes)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_utils::{
        build_avs_registry_chain_reader, build_avs_registry_chain_writer, test_register_operator,
    };
    use alloy::primitives::address;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_testing_utils::{
        anvil::{start_anvil_container, start_m2_anvil_container},
        anvil_constants::{
            FIFTH_ADDRESS, FIFTH_PRIVATE_KEY, FIRST_ADDRESS, FIRST_PRIVATE_KEY, OPERATOR_BLS_KEY,
        },
        transaction::wait_transaction,
    };

    #[tokio::test]
    async fn test_get_quorum_count() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let _ = avs_reader.get_quorum_count().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_at_block() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let bls_key_pair = BlsKeyPair::new(
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string(),
        )
        .unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let quorum_nums = Bytes::from([0]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let quorum_number = Bytes::from([0]);

        avs_reader
            .get_operators_stake_in_quorums_at_block(1245063, quorum_number)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_at_block_operator_id() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let bls_key_pair = BlsKeyPair::new(
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string(),
        )
        .unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let quorum_nums = Bytes::from([0]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let operator_address = address!("0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc");
        let operator_id = avs_reader.get_operator_id(operator_address).await.unwrap();

        avs_reader
            .get_operators_stake_in_quorums_at_block_operator_id(1245842, operator_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_at_current_block() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let bls_key_pair = BlsKeyPair::new(
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string(),
        )
        .unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let quorum_nums = Bytes::from([0]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let quorum_number = Bytes::from([0]);
        avs_reader
            .get_operators_stake_in_quorums_at_current_block(quorum_number)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_of_operator_at_current_block() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let bls_key_pair = BlsKeyPair::new(
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string(),
        )
        .unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let quorum_nums = Bytes::from([0]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let operator_address = address!("0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc");
        let operator_id = avs_reader.get_operator_id(operator_address).await.unwrap();

        avs_reader
            .get_operators_stake_in_quorums_of_operator_at_current_block(operator_id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operator_stake_in_quorums_of_operator_at_current_block() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
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
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let is_registered = avs_reader
            .is_operator_registered(avs_reader.registry_coordinator_addr)
            .await
            .unwrap();
        assert!(!is_registered);
    }

    #[tokio::test]
    async fn test_get_operators_stake_in_quorums_of_operator_at_block() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let bls_key_pair = BlsKeyPair::new(
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string(),
        )
        .unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let quorum_nums = Bytes::from([0]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let operator_address = address!("0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc");
        let operator_id = avs_reader.get_operator_id(operator_address).await.unwrap();

        avs_reader
            .get_operators_stake_in_quorums_of_operator_at_block(operator_id, 1246078)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_operator_restaked_strategies() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), FIRST_PRIVATE_KEY.to_string())
                .await;

        let strategies = avs_reader
            .get_operator_restaked_strategies(FIRST_ADDRESS)
            .await
            .unwrap();

        assert!(strategies.is_empty());

        let bls_key = OPERATOR_BLS_KEY.to_string();
        let quorum_nums = Bytes::from([0]);
        let bls_key_pair = BlsKeyPair::new(bls_key).unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums,
                "socket".to_string(),
            )
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let strategies = avs_reader
            .get_operator_restaked_strategies(FIRST_ADDRESS)
            .await
            .unwrap();

        assert!(!strategies.is_empty());
    }

    #[tokio::test]
    async fn test_query_existing_registered_operator_sockets() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let _ = avs_reader
            .query_existing_registered_operator_sockets(0, 1000)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_restakeable_strategies() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let strategies = avs_reader.get_restakeable_strategies().await.unwrap();
        assert!(!strategies.is_empty());
    }

    #[tokio::test]
    async fn test_query_registration_detail() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let operator_id = U256::from_str(
            "35344093966194310405039483339636912150346494903629410125452342281826147822033",
        )
        .unwrap();

        let operator_id_b256: B256 = operator_id.into();

        let operator_address = avs_reader
            .get_operator_from_id(operator_id_b256.into())
            .await
            .unwrap();

        let ret_query_registration_detail = avs_reader
            .query_registration_detail(operator_address)
            .await
            .unwrap();

        println!("{:?}", ret_query_registration_detail);

        // all the value are false
        for ret_value in ret_query_registration_detail.iter() {
            assert!(!ret_value);
        }

        // register an operator
        let is_registered = avs_reader
            .is_operator_registered(operator_address)
            .await
            .unwrap();
        assert!(!is_registered);
    }

    #[tokio::test]
    async fn test_weight_of_operator_for_quorum() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let quorum_number = 0;
        let operator_address = FIRST_ADDRESS;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let weight = avs_reader
            .weight_of_operator_for_quorum(quorum_number, operator_address)
            .await
            .unwrap();
        assert_eq!(weight, "10000000000000000000".parse().unwrap());
    }

    #[tokio::test]
    async fn test_strategy_params_length() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let quorum_number = 0;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let len = avs_reader
            .strategy_params_length(quorum_number)
            .await
            .unwrap();
        assert_eq!(len, "1".parse().unwrap());
    }

    #[tokio::test]
    async fn test_strategy_params_by_index() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let quorum_number = 0;
        let index = U256::from(0);
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let params = avs_reader
            .strategy_params_by_index(quorum_number, index)
            .await
            .unwrap();
        assert_eq!(
            params.multiplier,
            "1000000000000000000".parse::<U96>().unwrap()
        );
    }

    #[tokio::test]
    async fn test_avs_reader_methods() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key = OPERATOR_BLS_KEY.to_string();
        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let operator_addr = FIFTH_ADDRESS;
        let quorum_nums = Bytes::from([0]);

        test_register_operator(
            &avs_writer,
            bls_key,
            quorum_nums.clone(),
            http_endpoint.clone(),
        )
        .await;
        let quorum_number = 0;
        let index = U256::from(0);
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let operator_id = avs_reader.get_operator_id(operator_addr).await.unwrap();

        let stake_update = avs_reader
            .get_stake_update_at_index(quorum_number, operator_id, index)
            .await
            .unwrap();
        assert_eq!(stake_update.stake, "10000000000000000000".parse().unwrap());

        let stake_update = avs_reader
            .get_latest_stake_update(operator_id, quorum_number)
            .await
            .unwrap();
        assert_eq!(stake_update.stake, "10000000000000000000".parse().unwrap());

        let stake_update_vec = avs_reader
            .get_stake_history(operator_id, quorum_number)
            .await
            .unwrap();
        assert_eq!(
            stake_update_vec.first().unwrap().stake,
            "10000000000000000000".parse().unwrap()
        );

        let len = avs_reader
            .get_stake_history_length(operator_id, quorum_number)
            .await
            .unwrap();
        assert_eq!(len, "1".parse().unwrap());

        let stake_update_history_vec = avs_reader
            .get_stake_history(operator_id, quorum_number)
            .await
            .unwrap();

        assert_eq!(
            stake_update_history_vec.first().unwrap().stake,
            "10000000000000000000".parse().unwrap()
        );

        let latest_stake_update = avs_reader
            .get_latest_stake_update(operator_id, quorum_number)
            .await
            .unwrap();
        assert_eq!(
            latest_stake_update.stake,
            "10000000000000000000".parse().unwrap()
        );

        let stake_update = avs_reader
            .get_stake_update_at_index(quorum_number, operator_id, index)
            .await
            .unwrap();
        assert_eq!(stake_update.stake, "10000000000000000000".parse().unwrap());
        let block_number = get_provider(&http_endpoint)
            .get_block_number()
            .await
            .unwrap();
        let stake_update_at_index = avs_reader
            .get_stake_at_block_number(operator_id, quorum_number, (block_number) as u32)
            .await
            .unwrap();
        assert_eq!(
            "10000000000000000000".parse::<U96>().unwrap(),
            stake_update_at_index
        );

        let stake_update_at_index_at_block_number = avs_reader
            .get_stake_update_index_at_block_number(operator_id, quorum_number, block_number as u32)
            .await
            .unwrap();
        assert_eq!(stake_update_at_index_at_block_number, 0);

        let stake_at_index_at_block_number = avs_reader
            .get_stake_at_block_number_and_index(
                quorum_number,
                block_number as u32,
                operator_id,
                index,
            )
            .await
            .unwrap();
        assert_eq!(
            "10000000000000000000".parse::<U96>().unwrap(),
            stake_at_index_at_block_number
        );

        let total_stake_history_length = avs_reader
            .get_total_stake_history_length(quorum_number)
            .await
            .unwrap();
        assert_eq!(total_stake_history_length, "2".parse().unwrap());

        let current_total_stake = avs_reader
            .get_current_total_stake(quorum_number)
            .await
            .unwrap();
        dbg!(current_total_stake);
        assert_eq!(
            "10000000000000000000".parse::<U96>().unwrap(),
            current_total_stake
        );

        let total_stake_update_at_index = avs_reader
            .get_total_stake_update_at_index(quorum_number, index)
            .await
            .unwrap();
        assert_eq!(
            "0".parse::<U96>().unwrap(),
            total_stake_update_at_index.stake
        );

        let total_stake_at_block_number_from_index = avs_reader
            .get_total_stake_at_block_number_from_index(
                quorum_number,
                (block_number - 1) as u32,
                index,
            )
            .await
            .unwrap();
        assert_eq!(
            "0".parse::<U96>().unwrap(),
            total_stake_at_block_number_from_index
        );

        let total_stake_indices_at_block_number = avs_reader
            .get_total_stake_indices_at_block_number(block_number as u32, quorum_nums)
            .await
            .unwrap();
        assert_eq!(total_stake_indices_at_block_number.first(), Some(&1u32));
    }

    #[tokio::test]
    async fn test_query_existing_registered_operator_pub_keys() {
        let (_container, http_endpoint, ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIFTH_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let quorum_nums = Bytes::from([0]);
        let signature_expiry = U256::MAX;

        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair.clone(),
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let operators = avs_reader
            .query_existing_registered_operator_pub_keys(0, 1000, ws_endpoint)
            .await
            .unwrap();

        assert_eq!(*operators.0.first().unwrap(), FIFTH_ADDRESS);
        assert_eq!(
            operators.1.first().unwrap().g1_pub_key,
            BlsG1Point::new(bls_key_pair.public_key().g1()),
        );
    }
}
