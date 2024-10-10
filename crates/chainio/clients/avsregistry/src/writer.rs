use crate::error::AvsRegistryError;
use alloy_primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy_signer::Signer;
use alloy_signer_local::PrivateKeySigner;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_crypto_bls::{
    alloy_g1_point_to_g1_affine, convert_to_g1_point, convert_to_g2_point, BlsKeyPair,
};
use eigen_logging::logger::SharedLogger;
use eigen_utils::registrycoordinator::{
    IBLSApkRegistry::PubkeyRegistrationParams, ISignatureUtils::SignatureWithSaltAndExpiry,
    RegistryCoordinator, BN254::G1Point as RegistryG1Point, BN254::G2Point as RegistryG2Point,
};
use eigen_utils::{
    get_provider, get_signer,
    {servicemanagerbase::ServiceManagerBase, stakeregistry::StakeRegistry},
};
use std::str::FromStr;
use tracing::info;

/// Gas limit for registerOperator in [`RegistryCoordinator`]
pub const GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR: u64 = 2000000;

/// AvsRegistry Writer
#[derive(Debug)]
pub struct AvsRegistryChainWriter {
    logger: SharedLogger,
    service_manager_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever_addr: Address,
    stake_registry_addr: Address,
    bls_apk_registry_addr: Address,
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
        operator_state_retriever_addr: Address,
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

        let bls_apk_registry_addr_result = contract_registry_coordinator
            .blsApkRegistry()
            .call()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        let RegistryCoordinator::blsApkRegistryReturn {
            _0: bls_apk_registry,
        } = bls_apk_registry_addr_result;
        let stake_registry_addr = contract_registry_coordinator.stakeRegistry().call().await?;
        let RegistryCoordinator::stakeRegistryReturn { _0: stake_registry } = stake_registry_addr;
        let contract_stake_registry = StakeRegistry::new(stake_registry, &fill_provider);

        let delegation_manager_return = contract_stake_registry.delegation().call().await?;

        let StakeRegistry::delegationReturn {
            _0: delegation_manager_addr,
        } = delegation_manager_return;
        let avs_directory_addr = contract_service_manager_base.avsDirectory().call().await?;

        let ServiceManagerBase::avsDirectoryReturn { _0: avs_directory } = avs_directory_addr;

        let el_reader = ELChainReader::build(
            logger.clone(),
            delegation_manager_addr,
            avs_directory,
            &provider,
        )
        .await
        .map_err(|e| AvsRegistryError::ElContractsError(e.to_string()))?;

        Ok(AvsRegistryChainWriter {
            logger,
            service_manager_addr: service_manager,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr: stake_registry,
            bls_apk_registry_addr: bls_apk_registry,
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
        let g1_hashes_msg_to_sign_return = contract_registry_coordinator
            .pubkeyRegistrationMessageHash(wallet.address())
            .call()
            .await
            .map_err(|_| AvsRegistryError::PubKeyRegistrationMessageHash)?;

        let RegistryCoordinator::pubkeyRegistrationMessageHashReturn {
            _0: g1_hashes_msg_to_sign,
        } = g1_hashes_msg_to_sign_return;
        let sig = bls_key_pair
            .sign_hashed_to_curve_message(alloy_g1_point_to_g1_affine(g1_hashes_msg_to_sign))
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

        let operator_signature_with_salt_and_expiry = SignatureWithSaltAndExpiry {
            signature: operator_signature.as_bytes().into(),
            salt: operator_to_avs_registration_sig_salt,
            expiry: operator_to_avs_registration_sig_expiry,
        };

        let contract_call = contract_registry_coordinator.registerOperator(
            quorum_numbers.clone(),
            socket,
            PubkeyRegistrationParams {
                pubkeyRegistrationSignature: RegistryG1Point {
                    X: pub_key_reg_params.pubkeyRegistrationSignature.X,
                    Y: pub_key_reg_params.pubkeyRegistrationSignature.Y,
                },
                pubkeyG1: RegistryG1Point {
                    X: pub_key_reg_params.pubkeyG1.X,
                    Y: pub_key_reg_params.pubkeyG1.Y,
                },
                pubkeyG2: RegistryG2Point {
                    X: pub_key_reg_params.pubkeyG2.X,
                    Y: pub_key_reg_params.pubkeyG2.Y,
                },
            },
            operator_signature_with_salt_and_expiry,
        );

        let tx_call = contract_call.gas(GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR);
        let tx = tx_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        info!(tx_hash = ?tx,"Sent transaction to register operator in the AVS's registry coordinator" );
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

        info!(tx_hash = ?tx,"succesfully updated stakes of operator subset for all quorums" );
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

        let contract_call = contract_registry_coordinator.deregisterOperator(quorum_numbers);

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx,"succesfully deregistered operator with the AVS's registry coordinator" );
        Ok(*tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::AvsRegistryChainWriter;
    use alloy_primitives::{Address, Bytes, FixedBytes, U256};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil::start_anvil_container;
    use eigen_testing_utils::anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address,
    };
    use eigen_testing_utils::transaction::get_transaction_status;
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
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;

        let bls_key =
            "1371012690269088913462269866874713266643928125698382731338806296762673180359922"
                .to_string();
        let private_key =
            "8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba".to_string();
        let avs_writer = build_avs_registry_chain_writer(http_endpoint.clone(), private_key).await;
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
        test_deregister_operator(&avs_writer, quorum_nums, http_endpoint).await;
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

        let tx_status = get_transaction_status(http_url, tx_hash).await;
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

        let tx_status = get_transaction_status(http_url, tx_hash).await;
        assert!(tx_status);
    }

    // this function is caller from test_avs_writer_methods
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

        let tx_status = get_transaction_status(http_url, tx_hash).await;
        assert!(tx_status);
    }

    // this function is caller from test_avs_writer_methods
    async fn test_deregister_operator(
        avs_writer: &AvsRegistryChainWriter,
        quorum_nums: Bytes,
        http_url: String,
    ) {
        let tx_hash = avs_writer.deregister_operator(quorum_nums).await.unwrap();

        let tx_status = get_transaction_status(http_url, tx_hash).await;
        assert!(tx_status);
    }
}
