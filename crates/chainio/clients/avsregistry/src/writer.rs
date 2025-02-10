use crate::error::AvsRegistryError;
use alloy::primitives::aliases::U96;
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_crypto_bls::{
    alloy_g1_point_to_g1_affine, convert_to_g1_point, convert_to_g2_point, BlsKeyPair,
};
use eigen_logging::logger::SharedLogger;
use eigen_utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigen_utils::slashing::middleware::registrycoordinator::IStakeRegistryTypes::StrategyParams;
use eigen_utils::slashing::middleware::registrycoordinator::{
    IBLSApkRegistryTypes::PubkeyRegistrationParams, ISignatureUtils::SignatureWithSaltAndExpiry,
    RegistryCoordinator,
};
use eigen_utils::slashing::middleware::{
    servicemanagerbase::ServiceManagerBase, stakeregistry::StakeRegistry,
};

use eigen_common::{get_provider, get_signer};
use std::str::FromStr;
use tracing::info;

/// Gas limit for registerOperator in [`RegistryCoordinator`]
pub const GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR: u64 = 2000000;

/// AvsRegistry Writer
#[derive(Debug)]
pub struct AvsRegistryChainWriter {
    service_manager_addr: Address,
    registry_coordinator_addr: Address,
    stake_registry_addr: Address,
    el_reader: ELChainReader,
    provider: String,
    signer: String,
}

impl AvsRegistryChainWriter {
    /// build avs registry chain writer instance
    ///
    /// # Arguments
    ///
    /// * `logger` - SharedLogger used for logging
    /// * `provider` - provider string
    /// * `signer` - signer string
    /// * `registry_coordinator_addr` - registry coordinator address
    /// * `operator_state_retriever_addr` - operator state retriever address
    ///
    /// # Returns
    ///
    /// * `Result<Self, AvsRegistryError>` - a new AvsRegistryChainWriter
    ///
    /// # Errors
    ///
    /// * `AvsRegistryError` - if any error occurs
    pub async fn build_avs_registry_chain_writer(
        logger: SharedLogger,
        provider: String,
        signer: String,
        registry_coordinator_addr: Address,
        _operator_state_retriever_addr: Address,
    ) -> Result<Self, AvsRegistryError> {
        let fill_provider = get_provider(&provider);
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, &fill_provider);
        let service_manager_addr = contract_registry_coordinator
            .serviceManager()
            .call()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        let RegistryCoordinator::serviceManagerReturn {
            _0: service_manager,
        } = service_manager_addr;
        let contract_service_manager_base =
            ServiceManagerBase::new(service_manager, &fill_provider);
        let stake_registry_addr = contract_registry_coordinator.stakeRegistry().call().await?;
        let RegistryCoordinator::stakeRegistryReturn { _0: stake_registry } = stake_registry_addr;
        let contract_stake_registry = StakeRegistry::new(stake_registry, &fill_provider);
        let delegation_manager_return = contract_stake_registry.delegation().call().await?;
        let StakeRegistry::delegationReturn {
            _0: delegation_manager_addr,
        } = delegation_manager_return;
        let avs_directory_addr = contract_service_manager_base.avsDirectory().call().await?;
        let ServiceManagerBase::avsDirectoryReturn { _0: avs_directory } = avs_directory_addr;

        // We set rewards coordinator address as zero because we are not going to use it on any writer operation
        let rewards_coordinator_addr = Address::ZERO;

        let el_reader = ELChainReader::build(
            logger.clone(),
            delegation_manager_addr,
            avs_directory,
            rewards_coordinator_addr,
            &provider,
        )
        .await
        .map_err(|e| AvsRegistryError::ElContractsError(e.to_string()))?;

        Ok(AvsRegistryChainWriter {
            service_manager_addr: service_manager,
            registry_coordinator_addr,
            stake_registry_addr: stake_registry,
            el_reader,
            provider: provider.clone(),
            signer: signer.clone(),
        })
    }

    /// Register operator in quorum with avs registry coordinator
    ///
    /// # Arguments
    ///
    /// * `bls_key_pair` - bls key pair of the operator
    /// * `operator_to_avs_registration_sig_salt` - salt for the signature
    /// * `operator_to_avs_registration_sig_expiry` - expiry for the signature
    /// * `quorum_numbers` - quorum numbers
    /// * `socket` - socket used for calling the contract with `registerOperator` function
    ///
    /// # Returns
    ///
    /// * `Result<TxHash, AvsRegistryError>` - transaction hash of the register operator transaction
    pub async fn register_operator_in_quorum_with_avs_registry_coordinator(
        &self,
        bls_key_pair: BlsKeyPair,
        operator_to_avs_registration_sig_salt: FixedBytes<32>,
        operator_to_avs_registration_sig_expiry: U256,
        quorum_numbers: Bytes,
        socket: String,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = get_signer(&self.signer.clone(), &self.provider);
        let wallet = PrivateKeySigner::from_str(&self.signer)
            .map_err(|_| AvsRegistryError::InvalidPrivateKey)?;
        // tracing info
        info!(avs_service_manager = %self.service_manager_addr, operator= %wallet.address(),quorum_numbers = ?quorum_numbers,"quorum_numbers,registering operator with the AVS's registry coordinator");
        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let g1_hashed_msg_to_sign = contract_registry_coordinator
            .pubkeyRegistrationMessageHash(wallet.address())
            .call()
            .await
            .map_err(|_| AvsRegistryError::PubKeyRegistrationMessageHash)?
            ._0;
        let sig = bls_key_pair
            .sign_hashed_to_curve_message(alloy_g1_point_to_g1_affine(g1_hashed_msg_to_sign))
            .g1_point();
        let alloy_g1_point_signed_msg = convert_to_g1_point(sig.g1())?;
        let g1_pub_key_bn254 = convert_to_g1_point(bls_key_pair.public_key().g1())?;
        let g2_pub_key_bn254 = convert_to_g2_point(bls_key_pair.public_key_g2().g2())?;

        let pub_key_reg_params = PubkeyRegistrationParams {
            pubkeyRegistrationSignature: alloy_g1_point_signed_msg,
            pubkeyG1: g1_pub_key_bn254,
            pubkeyG2: g2_pub_key_bn254,
        };

        let msg_to_sign = self
            .el_reader
            .calculate_operator_avs_registration_digest_hash(
                wallet.address(),
                self.service_manager_addr,
                operator_to_avs_registration_sig_salt,
                operator_to_avs_registration_sig_expiry,
            )
            .await?;

        let operator_signature = wallet
            .sign_hash(&msg_to_sign)
            .await
            .map_err(|_| AvsRegistryError::InvalidSignature)?;

        let bytes = operator_signature.as_bytes().into();

        let operator_signature_with_salt_and_expiry = SignatureWithSaltAndExpiry {
            signature: bytes,
            salt: operator_to_avs_registration_sig_salt,
            expiry: operator_to_avs_registration_sig_expiry,
        };
        let contract_call = contract_registry_coordinator.registerOperator_0(
            quorum_numbers.clone(),
            socket,
            pub_key_reg_params,
            operator_signature_with_salt_and_expiry,
        );

        let tx_call = contract_call.gas(GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR);
        let tx = tx_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        info!(tx_hash = ?tx.tx_hash(),"Sent transaction to register operator in the AVS's registry coordinator" );
        Ok(*tx.tx_hash())
    }

    /// Updates the stake of their entire operator set
    ///
    /// Is used by avs teams running https://github.com/Layr-Labs/avs-sync to updates
    /// the stake of their entire operator set.
    /// Because of high gas costs of this operation, it typically needs to be called
    /// for every quorum, or perhaps for a small grouping of quorums
    /// (highly dependent on number of operators per quorum).
    ///
    /// # Arguments
    ///
    /// * `operators_per_quorum` - A vector of vectors of addresses, where each inner vector represents the operators in a quorum
    /// * `quorum_number` - The quorum number to update the stakes for
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the transaction that was sent to update the stakes
    pub async fn update_stakes_of_entire_operator_set_for_quorums(
        &self,
        operators_per_quorum: Vec<Vec<Address>>,
        quorum_number: Bytes,
    ) -> Result<TxHash, AvsRegistryError> {
        info!(quorum_numbers = %quorum_number, "updating stakes for entire operator set");
        let provider = get_signer(&self.signer.clone(), &self.provider);
        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);
        let contract_call = contract_registry_coordinator
            .updateOperatorsForQuorum(operators_per_quorum, quorum_number.clone());

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        info!(tx_hash = ?tx, quorum_numbers = %quorum_number," update stakes for entire operator set tx" );
        Ok(*tx.tx_hash())
    }

    /// Update stakes of operator subset for all quorums
    ///
    /// This function is used to update the stakes of a subset of operators for all quorums.
    ///
    /// # Arguments
    ///
    /// * `operators` - The list of operators to update the stakes for.
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the update stakes of operator subset for all quorums transaction.
    pub async fn update_stakes_of_operator_subset_for_all_quorums(
        &self,
        operators: Vec<Address>,
    ) -> Result<TxHash, AvsRegistryError> {
        info!(operators = ?operators, "updating stakes of operator subset for all quorums");

        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call = contract_registry_coordinator.updateOperators(operators);

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        info!(tx_hash = ?tx,"successfully updated stakes of operator subset for all quorums" );
        Ok(*tx.tx_hash())
    }

    /// Deregister operator
    ///
    /// This function is used to deregister an operator from the AVS's registry coordinator.
    ///
    /// # Arguments
    ///
    /// * `quorum_numbers` - The quorum numbers of the operator to be deregistered.
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the deregister operator transaction.
    pub async fn deregister_operator(
        &self,
        quorum_numbers: Bytes,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("deregistering operator with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call = contract_registry_coordinator.deregisterOperator_1(quorum_numbers);

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx,"successfully deregistered operator with the AVS's registry coordinator" );
        Ok(*tx.tx_hash())
    }

    /// Sets the look-ahead time for checking operator shares for a specific quorum
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The quorum number to set the look-ahead period for
    /// * `lookahead` - The number of blocks to look ahead when checking shares
    ///
    /// # Returns
    /// * `TxHash` - The transaction hash of the set slashable stake lookahead transaction
    pub async fn set_slashable_stake_lookahead(
        &self,
        quorum_number: u8,
        lookahead: u32,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("setting slashable stake lookahead");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);

        let contract_call =
            contract_stake_registry.setSlashableStakeLookahead(quorum_number, lookahead);

        contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx,"successfully set slashable stake lookahead" ))
            .map(|tx_hash| *tx_hash.tx_hash())
    }

    /// Set a new address as the rewards initiator
    ///
    /// # Arguments
    /// * `rewards_initiator` - The new address to set as the rewards initiator
    ///
    /// # Returns
    /// * `TxHash` - The transaction hash of the set rewards initiator transaction
    pub async fn set_rewards_initiator(
        &self,
        rewards_initiator: Address,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("setting a new address as the rewards initiator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_service_manager_base =
            ServiceManagerBase::new(self.service_manager_addr, provider);
        let contract_call = contract_service_manager_base.setRewardsInitiator(rewards_initiator);

        contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully set a new address as the rewards initiator"))
            .map(|tx| *tx.tx_hash())
    }

    /// Update socket
    ///
    /// This function is used to update the socket of the sender (if it is a registered operator).
    ///
    /// # Arguments
    ///
    /// * `socket` - The address of the socket to be assigned to the operator.
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the deregister operator transaction.
    pub async fn update_socket(&self, socket: String) -> Result<TxHash, AvsRegistryError> {
        info!("updating socket with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call = contract_registry_coordinator.updateSocket(socket);

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx,"successfully updated the socket with the AVS's registry coordinator" );
        Ok(*tx.tx_hash())
    }

    /// Update the configuration of an existing quorum.
    ///
    /// # Arguments
    /// * `quorum_number` - the quorum number to update
    /// * `operator_set_params` - the new config with [`OperatorSetParam`]
    ///
    /// # Returns
    /// * `TxHash` - The transaction hash of the set operator set param transaction
    pub async fn set_operator_set_param(
        &self,
        quorum_number: u8,
        operator_set_params: OperatorSetParam,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("setting operator set param with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call =
            contract_registry_coordinator.setOperatorSetParams(quorum_number, operator_set_params);

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx,"successfully set operator set param with the AVS's registry coordinator" );
        Ok(*tx.tx_hash())
    }

    /// Create a new quorum that tracks slashable stake for operators
    ///
    /// # Arguments
    ///
    /// * `operator_set_param` - Configures the quorum's max operator count and churn parameters
    /// * `minimum_stake` - Sets the minimum stake required for an operator to register or remain registered
    /// * `strategy_params` - A list of strategies and multipliers used by the StakeRegistry to calculate an operator's stake weight for the quorum
    /// * `look_ahead_period` - The number of blocks to look ahead when calculating slashable stake
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the create slashable stake quorum transactions
    pub async fn create_slashable_stake_quorum(
        &self,
        operator_set_param: OperatorSetParam,
        minimum_stake: U96,
        strategy_params: Vec<StrategyParams>,
        look_ahead_period: u32,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("creating slashable stake quorum with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        contract_registry_coordinator.createSlashableStakeQuorum(
            operator_set_param,
            minimum_stake,
            strategy_params,
            look_ahead_period,
            ).send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx,"successfully created slashable stake quorum with the AVS's registry coordinator" ))
            .map(|tx| *tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {

    use super::AvsRegistryChainWriter;
    use crate::test_utils::create_operator_set;
    use alloy::primitives::aliases::U96;
    use alloy::primitives::{Address, Bytes, FixedBytes, U256};
    use alloy::sol_types::SolCall;
    use eigen_common::get_signer;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil::{start_anvil_container, start_m2_anvil_container};
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_erc20_mock_strategy,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_service_manager_address,
    };
    use eigen_testing_utils::anvil_constants::{FIRST_PRIVATE_KEY, SECOND_ADDRESS};
    use eigen_testing_utils::transaction::wait_transaction;
    use eigen_utils::slashing::core::allocationmanager::AllocationManager;
    use eigen_utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
    use eigen_utils::slashing::middleware::registrycoordinator::IStakeRegistryTypes::StrategyParams;
    use eigen_utils::slashing::middleware::registrycoordinator::RegistryCoordinator;
    use eigen_utils::slashing::middleware::servicemanagerbase::ServiceManagerBase;
    use eigen_utils::slashing::middleware::stakeregistry::StakeRegistry;
    use futures_util::StreamExt;
    use std::str::FromStr;

    async fn build_avs_registry_chain_writer(
        http_endpoint: String,
        private_key: String,
    ) -> AvsRegistryChainWriter {
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint,
            private_key,
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_avs_writer_methods() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key =
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string();
        let private_key =
            "8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba".to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let operator_addr = Address::from_str("9965507D1a55bcC2695C58ba16FB37d819B0A4dc").unwrap();
        let quorum_nums = Bytes::from([0]);

        test_register_operator(
            &avs_writer,
            bls_key,
            quorum_nums.clone(),
            http_endpoint.clone(),
        )
        .await;
        test_update_stake_of_operator_subset(&avs_writer, operator_addr, http_endpoint.clone())
            .await;
        test_update_stake_of_entire_operator_set(
            &avs_writer,
            operator_addr,
            quorum_nums.clone(),
            http_endpoint.clone(),
        )
        .await;
        test_deregister_operator(&avs_writer, quorum_nums, http_endpoint.clone()).await;
    }

    #[tokio::test]
    async fn test_set_slashable_stake_lookahead() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        create_operator_set(http_endpoint.as_str(), avs_address).await;

        // Set up event poller to listen to `LookAheadPeriodChanged` events
        let provider = get_signer(&avs_writer.signer.clone(), &avs_writer.provider);
        let contract_stake_registry = StakeRegistry::new(avs_writer.stake_registry_addr, provider);
        let event = contract_stake_registry.LookAheadPeriodChanged_filter();
        let poller = event.watch().await.unwrap();

        // Set the slashable stake lookahead period. Old period is 0.
        let quorum_number = 0_u8;
        let lookahead = 10_u32;
        let tx_hash = avs_writer
            .set_slashable_stake_lookahead(quorum_number, lookahead)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        // Assert that event `LookAheadPeriodChanged` is the same as `new_rewards_init_address`
        let mut stream = poller.into_stream();
        let (stream_event, _) = stream.next().await.unwrap().unwrap();
        assert_eq!(stream_event.newLookAheadBlocks, lookahead);
    }

    #[tokio::test]
    async fn test_set_rewards_initiator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        // Set up event poller to listen to `RewardsInitiatorUpdated` events
        let provider = get_signer(&avs_writer.signer.clone(), &avs_writer.provider);
        let contract_registry_coordinator =
            ServiceManagerBase::new(avs_writer.service_manager_addr, provider);
        let event = contract_registry_coordinator.RewardsInitiatorUpdated_filter();
        let poller = event.watch().await.unwrap();

        let new_rewards_init_address = SECOND_ADDRESS;

        let tx_hash = avs_writer
            .set_rewards_initiator(new_rewards_init_address)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(tx_status.status());

        // Assert that event `RewardsInitiatorUpdated` is the same as `new_rewards_init_address`
        let mut stream = poller.into_stream();
        let (stream_event, _) = stream.next().await.unwrap().unwrap();
        assert_eq!(stream_event.newRewardsInitiator, new_rewards_init_address);
    }

    // this function is caller from test_avs_writer_methods
    async fn test_update_stake_of_operator_subset(
        avs_writer: &AvsRegistryChainWriter,
        operator_addr: Address,
        http_url: String,
    ) {
        let tx_hash = avs_writer
            .update_stakes_of_operator_subset_for_all_quorums(vec![operator_addr])
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_url, tx_hash).await.unwrap().status();
        assert!(tx_status);
    }

    // this function is caller from test_avs_writer_methods
    async fn test_update_stake_of_entire_operator_set(
        avs_writer: &AvsRegistryChainWriter,
        operator_id: Address,
        quorum_nums: Bytes,
        http_url: String,
    ) {
        let tx_hash = avs_writer
            .update_stakes_of_entire_operator_set_for_quorums(vec![vec![operator_id]], quorum_nums)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_url, tx_hash).await.unwrap().status();
        assert!(tx_status);
    }

    // this function is called from test_avs_writer_methods
    async fn test_register_operator(
        avs_writer: &AvsRegistryChainWriter,
        private_key_decimal: String,
        quorum_nums: Bytes,
        http_url: String,
    ) {
        let bls_key_pair = BlsKeyPair::new(private_key_decimal).unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);

        // this is set to U256::MAX so that the registry does not take the signature as expired.
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

        let tx_status = wait_transaction(&http_url, tx_hash).await.unwrap().status();
        assert!(tx_status);
    }

    // this function is caller from test_avs_writer_methods
    async fn test_deregister_operator(
        avs_writer: &AvsRegistryChainWriter,
        quorum_nums: Bytes,
        http_url: String,
    ) {
        let tx_hash = avs_writer.deregister_operator(quorum_nums).await.unwrap();

        let tx_status = wait_transaction(&http_url, tx_hash).await.unwrap().status();
        assert!(tx_status);
    }

    #[tokio::test]
    async fn test_update_socket() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key =
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string();
        let private_key =
            "8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba".to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let quorum_nums = Bytes::from([0]);

        test_register_operator(&avs_writer, bls_key, quorum_nums, http_endpoint.clone()).await;

        // Set up event poller to listen to update socket events
        let provider = get_signer(&avs_writer.signer.clone(), &avs_writer.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(avs_writer.registry_coordinator_addr, provider);

        let event = contract_registry_coordinator.OperatorSocketUpdate_filter();

        let poller = event.watch().await.unwrap();

        let new_socket_addr = "not a socket";

        // Update the socket for operator
        let tx_hash = avs_writer
            .update_socket(new_socket_addr.into())
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        // Assert that event socket is the same as passed in the update socket function
        let mut stream = poller.into_stream();
        let (stream_event, _) = stream.next().await.unwrap().unwrap();

        assert_eq!(stream_event.socket, new_socket_addr);
    }

    #[tokio::test]
    async fn test_set_operator_set_param() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key =
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string();
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let quorum_nums = Bytes::from([0]);

        test_register_operator(&avs_writer, bls_key, quorum_nums, http_endpoint.clone()).await;

        let registry_coordinator_contract = RegistryCoordinator::new(
            avs_writer.registry_coordinator_addr,
            get_signer(&avs_writer.signer.clone(), &avs_writer.provider),
        );

        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 50,
            kickBIPsOfTotalStake: 50,
        };

        let tx_hash = avs_writer
            .set_operator_set_param(0, operator_set_params.clone())
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let op_params = registry_coordinator_contract
            .getOperatorSetParams(0)
            .call()
            .await
            .unwrap();

        assert_eq!(
            op_params._0.maxOperatorCount,
            operator_set_params.maxOperatorCount
        );
        assert_eq!(
            op_params._0.kickBIPsOfOperatorStake,
            operator_set_params.kickBIPsOfOperatorStake
        );
        assert_eq!(
            op_params._0.kickBIPsOfTotalStake,
            operator_set_params.kickBIPsOfTotalStake
        );
    }

    #[tokio::test]
    async fn test_create_slashable_stake_quorum() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let service_manager_address = get_service_manager_address(http_endpoint.to_string()).await;

        let service_manager = ServiceManagerBase::new(
            service_manager_address,
            get_signer(&avs_writer.signer.clone(), &avs_writer.provider),
        );

        let allocation_manager_addr = get_allocation_manager_address(http_endpoint.clone()).await;

        service_manager
            .setAppointee(
                avs_writer.registry_coordinator_addr,
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::createOperatorSetsCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        let operator_set_param = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 50,
            kickBIPsOfTotalStake: 50,
        };
        let minimum_stake = U96::from(100);

        let strategy_param = StrategyParams {
            strategy: get_erc20_mock_strategy(http_endpoint.clone()).await,
            multiplier: U96::from(1),
        };

        let look_ahead_period = 10;

        let tx_hash = avs_writer
            .create_slashable_stake_quorum(
                operator_set_param.clone(),
                minimum_stake,
                vec![strategy_param],
                look_ahead_period,
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let registry_coordinator_contract = RegistryCoordinator::new(
            avs_writer.registry_coordinator_addr,
            get_signer(&avs_writer.signer.clone(), &avs_writer.provider),
        );

        let params = registry_coordinator_contract
            .getOperatorSetParams(0)
            .call()
            .await
            .unwrap();
        assert_eq!(
            params._0.maxOperatorCount,
            operator_set_param.maxOperatorCount,
        );
        assert_eq!(
            params._0.kickBIPsOfOperatorStake,
            operator_set_param.kickBIPsOfOperatorStake,
        );
        assert_eq!(
            params._0.kickBIPsOfTotalStake,
            operator_set_param.kickBIPsOfTotalStake
        );

        let quorum = registry_coordinator_contract
            .quorumCount()
            .call()
            .await
            .unwrap()
            ._0;

        assert_eq!(quorum, 1);
    }
}
