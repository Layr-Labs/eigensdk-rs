use crate::error::AvsRegistryError;
use alloy::primitives::aliases::U96;
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy::providers::WalletProvider;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::Signer;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_common::{get_provider, get_signer};
use eigen_crypto_bls::{
    alloy_g1_point_to_g1_affine, convert_to_g1_point, convert_to_g2_point, BlsKeyPair,
};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::operator_id_from_g1_pub_key;
use eigen_types::operator::QuorumNum;
use eigen_utils::convert_stake_registry_strategy_params_to_registry_coordinator_strategy_params;
use eigen_utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorKickParam;
use eigen_utils::slashing::middleware::registrycoordinator::{
    IBLSApkRegistryTypes::PubkeyRegistrationParams, ISignatureUtils::SignatureWithSaltAndExpiry,
    ISlashingRegistryCoordinatorTypes::OperatorSetParam, RegistryCoordinator,
};
use eigen_utils::slashing::middleware::servicemanagerbase::IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission;
use eigen_utils::slashing::middleware::servicemanagerbase::{
    IRewardsCoordinatorTypes::RewardsSubmission, ServiceManagerBase,
};
use eigen_utils::slashing::middleware::stakeregistry::IStakeRegistryTypes::StrategyParams;
use eigen_utils::slashing::middleware::stakeregistry::StakeRegistry;
use std::str::FromStr;
use tracing::{info, warn};

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
        service_manager_addr: Address,
    ) -> Result<Self, AvsRegistryError> {
        let fill_provider = get_provider(&provider);
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, &fill_provider);
        let contract_service_manager_base =
            ServiceManagerBase::new(service_manager_addr, &fill_provider);
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
            service_manager_addr,
            registry_coordinator_addr,
            stake_registry_addr: stake_registry,
            el_reader,
            provider: provider.clone(),
            signer: signer.clone(),
        })
    }

    /// Sets signer for AvsRegistryChainWriter
    ///
    /// # Arguments
    ///
    /// * `signer` - signer string
    ///
    pub fn set_signer(&mut self, signer: String) {
        self.signer = signer;
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

    /// Registers an operator while replacing existing operators in full quorums. If any quorum reaches
    /// its maximum operator capacity, `operatorKickParams` is used to replace an old operator with the new one.
    ///
    /// # Arguments
    ///
    /// * `bls_key_pair` - bls key pair of the operator
    /// * `operator_to_avs_registration_sig_salt` - operator signature salt
    /// * `operator_to_avs_registration_sig_expiry` - operator signature expiry
    /// * `quorum_numbers` - quorum numbers to register the new operator
    /// * `socket` - socket used for calling the contract with `registerOperator` function
    /// * `operators_to_kick` - operators to kick if quorum is full
    /// * `churn_signer_private_key` - private key of the churn signer
    /// * `churn_sig_salt` - churn signature salt
    /// * `churn_sig_expiry` - churn signature expiry
    ///
    /// # Returns
    ///
    /// * `TxHash` - transaction hash of the register operator with churn transaction
    #[allow(clippy::too_many_arguments)]
    pub async fn register_operator_with_churn(
        &self,
        bls_key_pair: BlsKeyPair,
        operator_to_avs_registration_sig_salt: FixedBytes<32>,
        operator_to_avs_registration_sig_expiry: U256,
        quorum_numbers: Bytes,
        socket: String,
        operators_to_kick: Vec<Address>,
        churn_signer_private_key: String,
        churn_sig_salt: FixedBytes<32>,
        churn_sig_expiry: U256,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = get_signer(&self.signer.clone(), &self.provider);
        let operator_wallet = PrivateKeySigner::from_str(&self.signer)
            .map_err(|_| AvsRegistryError::InvalidPrivateKey)?;
        let operator_address = operator_wallet.address();

        info!(
            avs_service_manager = %self.service_manager_addr,
            operator = %operator_address,
            quorum_numbers = ?quorum_numbers,
            "registering operator with churn the AVS's registry coordinator"
        );

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, &provider);

        let g1_hashed_msg_to_sign = contract_registry_coordinator
            .pubkeyRegistrationMessageHash(operator_address)
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
            pubkeyRegistrationSignature: alloy_g1_point_signed_msg.clone(),
            pubkeyG1: g1_pub_key_bn254.clone(),
            pubkeyG2: g2_pub_key_bn254.clone(),
        };

        let msg_to_sign = self
            .el_reader
            .calculate_operator_avs_registration_digest_hash(
                operator_address,
                self.service_manager_addr,
                operator_to_avs_registration_sig_salt,
                operator_to_avs_registration_sig_expiry,
            )
            .await?;

        let operator_signature = operator_wallet
            .sign_hash(&msg_to_sign)
            .await
            .map_err(|_| AvsRegistryError::InvalidSignature)?;

        let operator_signature_with_salt_and_expiry = SignatureWithSaltAndExpiry {
            signature: operator_signature.as_bytes().into(),
            salt: operator_to_avs_registration_sig_salt,
            expiry: operator_to_avs_registration_sig_expiry,
        };

        let operators_to_kick_params: Vec<OperatorKickParam> = operators_to_kick
            .iter()
            .zip(quorum_numbers.iter())
            .map(|(address, quorum_number)| OperatorKickParam {
                operator: *address,
                quorumNumber: *quorum_number,
            })
            .collect();

        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key())
            .map_err(|_| AvsRegistryError::GetOperatorId)?;

        let churn_wallet = PrivateKeySigner::from_str(&churn_signer_private_key)
            .map_err(|_| AvsRegistryError::InvalidPrivateKey)?;

        let churn_digest_hash = contract_registry_coordinator
            .calculateOperatorChurnApprovalDigestHash(
                operator_address,
                operator_id,
                operators_to_kick_params.clone(),
                churn_sig_salt,
                churn_sig_expiry,
            )
            .call()
            .await?
            ._0;

        let churn_signature = churn_wallet
            .sign_hash(&churn_digest_hash)
            .await
            .map_err(|_| AvsRegistryError::InvalidSignature)?;

        let churn_signature_with_salt_and_expiry = SignatureWithSaltAndExpiry {
            signature: churn_signature.as_bytes().into(),
            salt: churn_sig_salt,
            expiry: churn_sig_expiry,
        };

        let contract_call = contract_registry_coordinator.registerOperatorWithChurn(
            quorum_numbers,
            socket,
            pub_key_reg_params,
            operators_to_kick_params,
            churn_signature_with_salt_and_expiry,
            operator_signature_with_salt_and_expiry,
        );

        let tx = contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        info!(
            tx_hash = ?tx.tx_hash(),
            "Sent transaction to register operator with churn in the AVS's registry coordinator"
        );
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
    /// * `TxHash` - hash of the sent transaction.
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

    /// Create a new operator directed AVS rewards submission
    ///
    /// # Arguments
    ///
    /// * `operator_rewards_submission` - The operator directed rewards submission to create
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the create operator directed AVS rewards submission transaction
    pub async fn create_operator_directed_avs_rewards_submission(
        &self,
        operator_rewards_submission: Vec<OperatorDirectedRewardsSubmission>,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("creating operator directed AVS rewards submission with ServiceManagerBase");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let service_manager = ServiceManagerBase::new(self.service_manager_addr, &provider);

        service_manager.createOperatorDirectedAVSRewardsSubmission(operator_rewards_submission)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully created operator directed AVS rewards submission with ServiceManagerBase"))
            .map(|tx| *tx.tx_hash())
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
        info!(tx_hash = ?tx,"successfully updated the socket with the AVS's registry coordinator");
        Ok(*tx.tx_hash())
    }

    /// Set churn approver
    ///
    /// This function sets a new churn approver for the AVS's registry coordinator.
    ///
    /// # Arguments
    ///
    /// * `new_churn_approver` - address of the new churn approver.
    ///
    /// # Returns
    ///
    /// * `TxHash` - hash of the transaction.
    pub async fn set_churn_approver(
        &self,
        new_churn_approver: Address,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("set new churn approver with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        contract_registry_coordinator.setChurnApprover(new_churn_approver)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx,"successfully updated the new churn approver with the AVS's registry coordinator"))
            .map(|tx| *tx.tx_hash())
    }

    /// Force a deregistration of an operator from one or more quorums
    ///
    /// # Arguments
    ///
    /// * `operator_address` - The address of the operator to be ejected
    /// * `quorum_numbers` - The quorum numbers to eject the operator from
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the eject operator transaction
    pub async fn eject_operator(
        &self,
        operator_address: Address,
        quorum_numbers: Bytes,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("ejecting operator from quorum with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        contract_registry_coordinator.ejectOperator(operator_address, quorum_numbers)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully ejected operator from quorum with the AVS's registry coordinator"))
            .map(|tx| *tx.tx_hash())
    }

    /// This function is used to update the account identifier of the AVS's RegistryCoordinator.
    ///
    /// # Arguments
    ///
    /// * `new_account_identifier` - address of the new account identifier.
    ///
    /// # Returns
    ///
    /// * `TxHash` - hash of the sent transaction.
    pub async fn set_avs(
        &self,
        new_account_identifier: Address,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("updating the account identifier of the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        RegistryCoordinator::new(self.registry_coordinator_addr, provider)
            .setAVS(new_account_identifier)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully updated the account identifier of the AVS's registry coordinator"))
            .map(|tx| *tx.tx_hash())
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
        quorum_number: QuorumNum,
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
        info!(tx_hash = ?tx, "successfully set operator set param with the AVS's registry coordinator");
        Ok(*tx.tx_hash())
    }

    /// Create a new rewards submission
    ///
    /// This function is used to create a new rewards submission for the AVS
    ///
    /// # Arguments
    ///
    /// * `rewardsSubmissions` - The rewards submissions to create
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the create rewards submission transaction
    pub async fn create_avs_rewards_submission(
        &self,
        rewards_submissions: Vec<RewardsSubmission>,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("setting a new Rewards Submission list with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_service_manager_base =
            ServiceManagerBase::new(self.service_manager_addr, provider);
        let contract_call =
            contract_service_manager_base.createAVSRewardsSubmission(rewards_submissions);

        contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully created a new rewards submission with the AVS's registry coordinator"))
            .map(|tx| *tx.tx_hash())
    }

    /// Update the AVS metadata URI.
    ///
    /// This function updates the AVS metadata URI of the AVS's RegistryCoordinator.
    ///
    /// # Arguments
    ///
    /// * `avs_metadata_uri` - The new AVS metadata URI.
    ///
    /// # Returns
    ///
    /// * `TxHash` - hash of the sent transaction.
    pub async fn update_avs_metadata_uri(
        &self,
        avs_metadata_uri: &str,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("updating the AVS metadata URI of the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        ServiceManagerBase::new(self.service_manager_addr, provider)
            .updateAVSMetadataURI(avs_metadata_uri.to_string())
            .send()
            .await
            .inspect(|tx| info!(tx_hash = ?tx, "successfully updated AVS metadata URI"))
            .map_err(AvsRegistryError::AlloyContractError)
            .map(|tx| *tx.tx_hash())
    }

    /// Sets the minimum stake for the quorum
    ///
    /// Can only be called by the registry coordinator's owner.
    /// The quorum number must exist, or else the tx will fail.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The respective quorum number.
    /// * `minimum_stake` - The minimum stake as a [`U96`].
    ///
    /// # Returns
    ///
    /// * [`TxHash`] - hash of the transaction.
    pub async fn set_minimum_stake_for_quorum(
        &self,
        quorum_number: QuorumNum,
        minimum_stake: U96,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let reg_coordinator_owner =
            RegistryCoordinator::new(self.registry_coordinator_addr, get_provider(&self.provider))
                .owner()
                .call()
                .await?
                ._0;
        let caller_address =
            get_signer(&self.signer.clone(), &self.provider).default_signer_address();
        if !reg_coordinator_owner.eq(&caller_address) {
            warn!(caller = %caller_address,registry_coordinator_owner = %reg_coordinator_owner,"Caller must be registry coordinator's owner when calling set_minimum_stake_for_quorum");
        }

        let tx = contract_stake_registry
            .setMinimumStakeForQuorum(quorum_number, minimum_stake)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx.tx_hash(),quorum_number = %quorum_number,"Setting minimum stake for quorum");
        Ok(*tx.tx_hash())
    }

    /// Add strategies for the given quorum.
    ///
    /// Can only be called by the registry coordinator's owner.
    /// The quorum numbers must exist, or else the tx will fail.
    ///
    /// # Arguments
    ///
    /// * `quorum_number` - The respective quorum's number.
    /// * `strategy_params` - list of parameters for the strategies to add.
    ///
    /// # Returns
    ///
    /// * [`TxHash`] - hash of the transaction.
    pub async fn add_strategies(
        &self,
        quorum_number: QuorumNum,
        strategy_params: Vec<StrategyParams>,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let reg_coordinator_owner =
            RegistryCoordinator::new(self.registry_coordinator_addr, get_provider(&self.provider))
                .owner()
                .call()
                .await?
                ._0;
        let caller_address =
            get_signer(&self.signer.clone(), &self.provider).default_signer_address();
        if !reg_coordinator_owner.eq(&caller_address) {
            warn!(caller = %caller_address,registry_coordinator_owner = %reg_coordinator_owner,"Caller must be registry coordinator's owner when adding strategies");
        }
        let tx = contract_stake_registry
            .addStrategies(quorum_number, strategy_params)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;

        info!(tx_hash = ?tx.tx_hash(),"Adding strategies");
        Ok(*tx.tx_hash())
    }

    /// Remove strategies and their associated weights from the quorum's considered strategies.
    ///
    /// Can only be called by the registry coordinator's owner.
    /// The quorum numbers must exist, else the tx will fail.
    /// higher indices should be *first* in the list of `indices_to_remove`, since otherwise
    /// the removal of lower index entries will cause a shift in the indices of the other strategies to remove
    ///  
    /// # Arguments
    ///
    /// # `quorum_number` - The respective quorum numbers in [`QuorumNum`].
    /// # `indices_to_remove` - [`Vec<U256>`].
    ///
    /// # Returns
    ///
    /// * [`TxHash`] - The transaction hash of the transaction.
    pub async fn remove_strategies(
        &self,
        quorum_number: QuorumNum,
        indices_to_remove: Vec<U256>,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let reg_coordinator_owner =
            RegistryCoordinator::new(self.registry_coordinator_addr, get_provider(&self.provider))
                .owner()
                .call()
                .await?
                ._0;
        let caller_address =
            get_signer(&self.signer.clone(), &self.provider).default_signer_address();
        if !reg_coordinator_owner.eq(&caller_address) {
            warn!(caller = %caller_address,registry_coordinator_owner = %reg_coordinator_owner,"Caller must be registry coordinator's owner when removing strategies");
        }
        let tx = contract_stake_registry
            .removeStrategies(quorum_number, indices_to_remove)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx.tx_hash(),"Removing strategies");
        Ok(*tx.tx_hash())
    }

    /// Modifies the weights of existing strategies for a specific quorum.
    ///
    /// Can only be called by the registry coordinator's owner.
    /// The quorum numbers must exist, else the tx will fail.
    ///  
    /// # Arguments
    ///
    /// # `quorum_number` - The respective quorum numbers in [`QuorumNum`].
    /// # `strategy_indices` - Indices of the strategies to change in [`Vec<U256>`].
    /// # `new_multipliers` -  New multipliers for the strategies in [`Vec<U96>`].
    ///
    /// # Returns
    ///
    /// * [`TxHash`] - The transaction hash of the transaction.
    pub async fn modify_strategy_params(
        &self,
        quorum_number: QuorumNum,
        strategy_indices: Vec<U256>,
        new_multipliers: Vec<U96>,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_stake_registry = StakeRegistry::new(self.stake_registry_addr, provider);
        let reg_coordinator_owner =
            RegistryCoordinator::new(self.registry_coordinator_addr, get_provider(&self.provider))
                .owner()
                .call()
                .await?
                ._0;
        let caller_address =
            get_signer(&self.signer.clone(), &self.provider).default_signer_address();
        if !reg_coordinator_owner.eq(&caller_address) {
            warn!(caller = %caller_address,registry_coordinator_owner = %reg_coordinator_owner,"Caller must be registry coordinator's owner when modifying strategy params");
        }

        let tx = contract_stake_registry
            .modifyStrategyParams(quorum_number, strategy_indices, new_multipliers)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)?;
        info!(tx_hash = ?tx.tx_hash(),"modifying strategy params");
        Ok(*tx.tx_hash())
    }

    /// Create a new quorum that tracks total delegated stake for operators.
    ///
    /// # Arguments
    ///
    /// * `operator_set_param` - Configures the quorum's max operator count and churn parameters
    /// * `minimum_stake` - Sets the minimum stake required for an operator to register or remain registered
    /// * `strategy_params` - A list of strategies and multipliers used by the StakeRegistry to calculate an operator's stake weight for the quorum
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the create total delegated stake quorum transaction
    pub async fn create_total_delegated_stake_quorum(
        &self,
        operator_set_param: OperatorSetParam,
        minimum_stake: U96,
        strategy_params: Vec<StrategyParams>,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("creating total delegated stake quorum with the AVS's registry coordinator");

        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);
        let registry_coordinator_strategy_params = strategy_params
            .iter()
            .map(|param| {
                convert_stake_registry_strategy_params_to_registry_coordinator_strategy_params(
                    param.clone(),
                )
            })
            .collect();
        let contract_call = contract_registry_coordinator.createTotalDelegatedStakeQuorum(
            operator_set_param,
            minimum_stake,
            registry_coordinator_strategy_params,
        );

        contract_call
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully created total delegated stake quorum with the AVS's registry coordinator"))
            .map(|tx| *tx.tx_hash())
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

        let registry_coordinator_strategy_params = strategy_params
            .iter()
            .map(|param| {
                convert_stake_registry_strategy_params_to_registry_coordinator_strategy_params(
                    param.clone(),
                )
            })
            .collect();
        contract_registry_coordinator.createSlashableStakeQuorum(
            operator_set_param,
            minimum_stake,
            registry_coordinator_strategy_params,
            look_ahead_period,
            ).send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx,"successfully created slashable stake quorum with the AVS's registry coordinator" ))
            .map(|tx| *tx.tx_hash())
    }

    /// Set the ejection cooldow that an operator must wait in seconds after ejection before registering for any quorum
    ///
    /// # Arguments
    ///
    /// * `cooldown` - the new ejection cooldown in seconds
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the set ejection cooldown transaction
    pub async fn set_ejection_cooldown(&self, cooldown: U256) -> Result<TxHash, AvsRegistryError> {
        info!("setting ejecting cooldown with the AVS's registry coordinator");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        contract_registry_coordinator.setEjectionCooldown(cooldown)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx, "successfully set ejection cooldown with the AVS's registry coordinator"))
            .map(|tx| *tx.tx_hash())
    }

    /// Sets an address as the ejector, which can force-deregister operators from quorums.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to set as the ejector
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash of the set ejector transaction
    pub async fn set_ejector(&self, address: Address) -> Result<TxHash, AvsRegistryError> {
        info!("setting ejector");
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        contract_registry_coordinator
            .setEjector(address)
            .send()
            .await
            .map_err(AvsRegistryError::AlloyContractError)
            .inspect(|tx| info!(tx_hash = ?tx,"successfully set ejector with the AVS's registry coordinator" ))
            .map(|tx| *tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::AvsRegistryChainWriter;
    use crate::test_utils::{
        build_avs_registry_chain_reader, build_avs_registry_chain_writer, create_operator_set,
        test_deregister_operator, test_register_operator,
    };
    use alloy::primitives::{address, aliases::U96, Address, Bytes, FixedBytes, U256};
    use alloy::sol_types::SolCall;
    use eigen_common::{get_provider, get_signer};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_testing_utils::anvil::{start_anvil_container, start_m2_anvil_container};
    use eigen_testing_utils::anvil_constants::OPERATOR_BLS_KEY_2;
    use eigen_testing_utils::anvil_constants::SECOND_PRIVATE_KEY;
    use eigen_testing_utils::anvil_constants::THIRD_ADDRESS;
    use eigen_testing_utils::anvil_constants::THIRD_PRIVATE_KEY;
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_erc20_mock_strategy, get_rewards_coordinator_address,
        get_service_manager_address,
    };
    use eigen_testing_utils::anvil_constants::{
        FIFTH_ADDRESS, FIFTH_PRIVATE_KEY, FIRST_ADDRESS, FIRST_PRIVATE_KEY, OPERATOR_BLS_KEY,
        SECOND_ADDRESS,
    };
    use eigen_testing_utils::transaction::wait_transaction;
    use eigen_utils::slashing::core::allocationmanager::AllocationManager;
    use eigen_utils::slashing::core::irewardscoordinator::IRewardsCoordinator;
    use eigen_utils::slashing::middleware::registrycoordinator::{
        ISlashingRegistryCoordinatorTypes::OperatorSetParam, RegistryCoordinator,
    };
    use eigen_utils::slashing::middleware::servicemanagerbase::IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission;
    use eigen_utils::slashing::middleware::servicemanagerbase::IRewardsCoordinatorTypes::OperatorReward;
    use eigen_utils::slashing::middleware::servicemanagerbase::{
        IRewardsCoordinatorTypes::RewardsSubmission,
        IRewardsCoordinatorTypes::StrategyAndMultiplier, ServiceManagerBase,
    };
    use eigen_utils::slashing::middleware::stakeregistry::IStakeRegistryTypes::StrategyParams;
    use eigen_utils::slashing::middleware::stakeregistry::StakeRegistry;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_avs_writer_methods() {
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
    async fn test_create_operator_directed_avs_rewards_submission() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;

        let (_, token_address) = avs_writer
            .el_reader
            .get_strategy_and_underlying_token(strategy_address)
            .await
            .unwrap();

        let operator_rewards = OperatorReward {
            operator: SECOND_ADDRESS,
            amount: U256::from(100),
        };

        let strategies = StrategyAndMultiplier {
            strategy: strategy_address,
            multiplier: U96::from(1),
        };

        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.clone()).await;
        let provider = get_provider(&http_endpoint);
        let rewards_coordinator = IRewardsCoordinator::new(rewards_coordinator_address, &provider);

        let calculation_interval_seconds = rewards_coordinator
            .CALCULATION_INTERVAL_SECONDS()
            .call()
            .await
            .unwrap()
            ._0;

        assert_ne!(calculation_interval_seconds, 0);

        let rewards_duration = rewards_coordinator
            .MAX_REWARDS_DURATION()
            .call()
            .await
            .unwrap()
            ._0;

        // Calculate the most recent interval start time that is less than the current timestamp
        // This ensures the reward submission aligns with the contract's time-based requirements
        let current_timestamp: u32 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap();
        let intervals_since_genesis = current_timestamp / calculation_interval_seconds;
        let last_valid_interval_start =
            (intervals_since_genesis - 2) * calculation_interval_seconds;

        // These values are set to align with the contract's requirements for the `OperatorDirectedRewardsSubmission`.
        // https://github.com/Layr-Labs/eigenlayer-contracts/blob/5341ef83500476c62a4406ff00cdde7f5c2cc11f/src/contracts/core/RewardsCoordinator.sol#L438
        // https://github.com/Layr-Labs/eigenlayer-contracts/blob/5341ef83500476c62a4406ff00cdde7f5c2cc11f/src/contracts/core/RewardsCoordinator.sol#L485
        let duration = rewards_duration;
        let start_timestamp = last_valid_interval_start;

        let operator_rewards_submission = OperatorDirectedRewardsSubmission {
            token: token_address,
            description: "test".to_string(),
            duration,
            startTimestamp: start_timestamp,
            operatorRewards: vec![operator_rewards.clone()],
            strategiesAndMultipliers: vec![strategies],
        };

        let tx_hash = avs_writer
            .create_operator_directed_avs_rewards_submission(vec![operator_rewards_submission])
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let is_registered = avs_writer
            .el_reader
            .is_operator_registered(operator_rewards.operator)
            .await
            .unwrap();

        assert!(is_registered);
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

    #[tokio::test]
    async fn test_update_socket() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key = OPERATOR_BLS_KEY.to_string();
        let private_key = FIFTH_PRIVATE_KEY.to_string();
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
    async fn test_update_avs_metadata_uri() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), FIRST_PRIVATE_KEY.to_string())
                .await;

        let new_metadata = "https://avs-metadata-uri.com";

        let tx_hash = avs_writer
            .update_avs_metadata_uri(new_metadata)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);
    }

    #[tokio::test]
    async fn test_set_churn_approver() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let provider = get_signer(&avs_writer.signer.clone(), &avs_writer.provider);

        let regcoord = RegistryCoordinator::new(avs_writer.registry_coordinator_addr, &provider);

        let current_churn_approver = regcoord.churnApprover().call().await.unwrap()._0;
        let new_churn_approver = SECOND_ADDRESS;
        assert_ne!(current_churn_approver, new_churn_approver);

        let tx_hash = avs_writer
            .set_churn_approver(new_churn_approver)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let current_churn_approver = regcoord.churnApprover().call().await.unwrap()._0;
        assert_eq!(current_churn_approver, new_churn_approver);
    }

    #[tokio::test]
    async fn test_set_avs() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), FIRST_PRIVATE_KEY.to_string())
                .await;

        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;

        let provider = get_provider(&http_endpoint);
        let regcoord = RegistryCoordinator::new(avs_writer.registry_coordinator_addr, &provider);

        let old_account_identifier = regcoord.avs().call().await.unwrap()._0;
        assert_eq!(old_account_identifier, service_manager_address);

        let new_account_identifier = FIRST_ADDRESS;

        let tx_hash = avs_writer.set_avs(new_account_identifier).await.unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let current_account_identifier = regcoord.avs().call().await.unwrap()._0;
        assert_eq!(current_account_identifier, new_account_identifier);
    }

    #[tokio::test]
    async fn test_set_operator_set_param() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key = OPERATOR_BLS_KEY.to_string();
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
    async fn test_register_operator_with_churn() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key = OPERATOR_BLS_KEY.to_string();
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let quorum_nums = Bytes::from([0]);
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;

        test_register_operator(
            &avs_writer,
            bls_key.clone(),
            quorum_nums.clone(),
            http_endpoint.clone(),
        )
        .await;

        let is_registered = avs_reader
            .is_operator_registered(FIRST_ADDRESS)
            .await
            .unwrap();
        assert!(is_registered);

        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 1,
            kickBIPsOfOperatorStake: 10,
            kickBIPsOfTotalStake: 10000,
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

        let tx_hash = avs_writer.set_churn_approver(THIRD_ADDRESS).await.unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let avs_writer_2 =
            build_avs_registry_chain_writer(http_endpoint.clone(), SECOND_PRIVATE_KEY.to_string())
                .await;

        let bls_key_2 = OPERATOR_BLS_KEY_2.to_string();

        let operator_sig_salt = FixedBytes::from([0x02; 32]);
        let churn_sig_salt = FixedBytes::from([0x05; 32]);
        let sig_expiry = U256::MAX;
        let churn_private_key = THIRD_PRIVATE_KEY.to_string();

        let tx_hash = avs_writer_2
            .register_operator_with_churn(
                BlsKeyPair::new(bls_key_2).unwrap(),
                operator_sig_salt,
                sig_expiry,
                quorum_nums.clone(),
                "socket".to_string(),
                vec![FIRST_ADDRESS],
                churn_private_key,
                churn_sig_salt,
                sig_expiry,
            )
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let is_registered = avs_reader
            .is_operator_registered(FIRST_ADDRESS)
            .await
            .unwrap();
        assert!(!is_registered);

        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let is_registered = avs_reader
            .is_operator_registered(SECOND_ADDRESS)
            .await
            .unwrap();
        assert!(is_registered);
    }

    #[tokio::test]
    async fn test_set_minimum_stake_for_quorum() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;
        let quorum_number = 0;
        let minimum_stake = U96::from(10);
        let tx_hash = avs_writer
            .set_minimum_stake_for_quorum(quorum_number, minimum_stake)
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let contract_stake_registry =
            StakeRegistry::new(avs_writer.stake_registry_addr, get_provider(&http_endpoint));
        assert_eq!(
            contract_stake_registry
                .minimumStakeForQuorum(quorum_number)
                .call()
                .await
                .unwrap()
                ._0,
            minimum_stake
        );
    }

    #[tokio::test]
    async fn test_create_total_delegated_stake_quorum() {
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
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 50,
            kickBIPsOfTotalStake: 50,
        };
        let minimum_stake = U96::from(10);
        let strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        let strategy_params = StrategyParams {
            strategy,
            multiplier: U96::from(1),
        };
        let strategy_params = vec![strategy_params];

        let tx_hash = avs_writer
            .create_total_delegated_stake_quorum(
                operator_set_params.clone(),
                minimum_stake,
                strategy_params,
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
            operator_set_params.maxOperatorCount,
        );

        assert_eq!(
            params._0.kickBIPsOfOperatorStake,
            operator_set_params.kickBIPsOfOperatorStake,
        );
        assert_eq!(
            params._0.kickBIPsOfTotalStake,
            operator_set_params.kickBIPsOfTotalStake
        );

        let quorum = registry_coordinator_contract
            .quorumCount()
            .call()
            .await
            .unwrap()
            ._0;

        assert_eq!(quorum, 1);
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
    }

    #[tokio::test]
    async fn test_set_add_strategies() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIRST_PRIVATE_KEY.to_string();

        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let quorum_number = 0;
        let strategy_params = [StrategyParams {
            strategy: address!("54945180dB7943c0ed0FEE7EdaB2Bd24620256bc"),
            multiplier: U96::from(1),
        }];

        let tx_hash = avs_writer
            .add_strategies(quorum_number, strategy_params.to_vec())
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let contract_stake_registry =
            StakeRegistry::new(avs_writer.stake_registry_addr, get_provider(&http_endpoint));
        let expected_strategy = contract_stake_registry
            .strategyParams(quorum_number, "1".parse().unwrap())
            .call()
            .await
            .unwrap()
            .strategy;

        assert_eq!(strategy_params[0].strategy, expected_strategy);
    }

    #[tokio::test]
    async fn test_remove_strategies() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIRST_PRIVATE_KEY.to_string();

        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let quorum_number = 0;
        let strategy_params = [StrategyParams {
            strategy: address!("54945180dB7943c0ed0FEE7EdaB2Bd24620256bc"),
            multiplier: U96::from(1),
        }];

        avs_writer
            .add_strategies(quorum_number, strategy_params.to_vec())
            .await
            .unwrap();

        let tx_hash = avs_writer
            .remove_strategies(quorum_number, [U256::from(1)].to_vec())
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);
        let contract_stake_registry =
            StakeRegistry::new(avs_writer.stake_registry_addr, get_provider(&http_endpoint));
        assert!(contract_stake_registry
            .strategyParams(quorum_number, "1".parse().unwrap())
            .call()
            .await
            .is_err());
    }
    #[tokio::test]
    async fn test_set_ejector_cooldown() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();

        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let registry_contract = RegistryCoordinator::new(
            avs_writer.registry_coordinator_addr,
            get_signer(&avs_writer.signer.clone(), &avs_writer.provider),
        );

        let new_cooldown = U256::from(100);
        let tx_hash = avs_writer
            .set_ejection_cooldown(new_cooldown)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);
        let cooldown = registry_contract.ejectionCooldown().call().await.unwrap();
        assert_eq!(cooldown._0, new_cooldown);
    }

    #[tokio::test]
    async fn test_eject_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let bls_key = OPERATOR_BLS_KEY.to_string();
        let register_operator_address = FIRST_ADDRESS;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let quorum_nums = Bytes::from([0]);

        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        test_register_operator(
            &avs_writer,
            bls_key,
            quorum_nums.clone(),
            http_endpoint.clone(),
        )
        .await;

        let avs_reader = build_avs_registry_chain_reader(http_endpoint.clone()).await;
        let is_registered = avs_reader
            .is_operator_registered(register_operator_address)
            .await
            .unwrap();
        assert!(is_registered);

        let tx_hash = avs_writer
            .eject_operator(register_operator_address, quorum_nums)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);
    }

    #[tokio::test]
    async fn test_modify_strategy_params() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;

        let private_key = FIRST_PRIVATE_KEY.to_string();

        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let quorum_number = 0;
        let strategy_params = [StrategyParams {
            strategy: address!("54945180dB7943c0ed0FEE7EdaB2Bd24620256bc"),
            multiplier: U96::from(1),
        }];

        avs_writer
            .add_strategies(quorum_number, strategy_params.to_vec())
            .await
            .unwrap();

        let strategy_indices = [U256::from(1)];
        let new_multipliers = [U96::from(2)];

        let tx_hash = avs_writer
            .modify_strategy_params(
                quorum_number,
                strategy_indices.to_vec(),
                new_multipliers.to_vec(),
            )
            .await
            .unwrap();
        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();
        assert!(tx_status);

        let contract_stake_registry =
            StakeRegistry::new(avs_writer.stake_registry_addr, get_provider(&http_endpoint));
        assert_eq!(
            U96::from(2),
            contract_stake_registry
                .strategyParams(quorum_number, "1".parse().unwrap())
                .call()
                .await
                .unwrap()
                .multiplier
        );
    }

    #[tokio::test]
    async fn test_set_ejector() {
        let (_container, http_endpoint, _ws_endpoint) = start_m2_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let new_ejector_address = SECOND_ADDRESS;

        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let registry_contract = RegistryCoordinator::new(
            avs_writer.registry_coordinator_addr,
            get_signer(&avs_writer.signer.clone(), &avs_writer.provider),
        );
        let ejecutor = registry_contract.ejector().call().await.unwrap();
        assert_ne!(ejecutor._0, new_ejector_address);

        let tx_hash = avs_writer.set_ejector(new_ejector_address).await.unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        let ejecutor = registry_contract.ejector().call().await.unwrap();
        assert_eq!(ejecutor._0, new_ejector_address);
    }

    #[tokio::test]
    async fn test_create_avs_rewards_submission() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.clone()).await;
        let provider = get_provider(&http_endpoint);
        let rewards_coordinator = IRewardsCoordinator::new(rewards_coordinator_address, &provider);

        let rewards_duration = rewards_coordinator
            .MAX_REWARDS_DURATION()
            .call()
            .await
            .unwrap()
            ._0;

        let calculation_interval_seconds = rewards_coordinator
            .CALCULATION_INTERVAL_SECONDS()
            .call()
            .await
            .unwrap()
            ._0;

        // These values are set to align with the contract's requirements for the `OperatorDirectedRewardsSubmission`.
        // https://github.com/Layr-Labs/eigenlayer-contracts/blob/5341ef83500476c62a4406ff00cdde7f5c2cc11f/src/contracts/core/RewardsCoordinator.sol#L438
        // https://github.com/Layr-Labs/eigenlayer-contracts/blob/5341ef83500476c62a4406ff00cdde7f5c2cc11f/src/contracts/core/RewardsCoordinator.sol#L485
        // Calculate the most recent interval start time that is less than the current timestamp
        // This ensures the reward submission aligns with the contract's time-based requirements
        let current_timestamp: u32 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap();
        let intervals_since_genesis = current_timestamp / calculation_interval_seconds;
        let start_timestamp = (intervals_since_genesis + 1) * calculation_interval_seconds;

        let strategy_address = dbg!(get_erc20_mock_strategy(http_endpoint.clone()).await);
        let (_, token) = avs_writer
            .el_reader
            .get_strategy_and_underlying_token(strategy_address)
            .await
            .unwrap();

        let strategies_and_multipliers = vec![StrategyAndMultiplier {
            strategy: strategy_address,
            multiplier: U96::from(1),
        }];
        let rewards_submissions = vec![RewardsSubmission {
            strategiesAndMultipliers: strategies_and_multipliers,
            token,
            amount: U256::from(1_000),
            startTimestamp: start_timestamp,
            duration: rewards_duration,
        }];

        let tx_hash = avs_writer
            .create_avs_rewards_submission(rewards_submissions)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);
    }
}
