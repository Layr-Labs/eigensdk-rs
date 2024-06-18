use alloy_signer::SignerSync;
use alloy_signer_wallet::LocalWallet;
use ark_bn254::G1Projective;
use eigen_chainio_utils::{
    convert_bn254_to_ark, convert_to_bn254_g1_point, convert_to_bn254_g2_point,
};
use eigen_client_elcontracts::reader::ELChainReader;
use std::str::FromStr;

use eigen_utils::binding::{
    BLSApkRegistry::{G1Point, PubkeyRegistrationParams},
    RegistryCoordinator::{
        self, G1Point as RegistryG1Point, G2Point as RegistryG2Point,
        PubkeyRegistrationParams as RegistryPubkeyRegistrationParams,
    },
};

use alloy_primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use eigen_crypto_bls::attestation::KeyPair;
use tracing::info;
use RegistryCoordinator::SignatureWithSaltAndExpiry;

use eigen_utils::{
    binding::{ServiceManagerBase, StakeRegistry},
    get_provider, get_signer,
};

/// AvsRegistry Writer
#[derive(Debug)]
pub struct AvsRegistryChainWriter {
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
    pub async fn new(
        service_manager_addr: Address,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        stake_registry_addr: Address,
        bls_apk_registry_addr: Address,
        el_reader: ELChainReader,
        provider: String,
        signer: String,
    ) -> Self {
        AvsRegistryChainWriter {
            service_manager_addr,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr,
            bls_apk_registry_addr,
            el_reader,
            provider,
            signer,
        }
    }

    pub async fn build_avs_registry_chain_writer(
        &self,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = get_provider(&self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, &provider);

        let service_manager_addr = contract_registry_coordinator
            .serviceManager()
            .call()
            .await?;
        let RegistryCoordinator::serviceManagerReturn {
            _0: service_manager,
        } = service_manager_addr;
        let contract_service_manager_base = ServiceManagerBase::new(service_manager, &provider);

        let bls_apk_registry_addr = contract_registry_coordinator
            .blsApkRegistry()
            .call()
            .await?;
        let RegistryCoordinator::blsApkRegistryReturn {
            _0: bls_apk_registry,
        } = bls_apk_registry_addr;
        let stake_registry_addr = contract_registry_coordinator.stakeRegistry().call().await?;
        let RegistryCoordinator::stakeRegistryReturn { _0: stake_registry } = stake_registry_addr;
        let contract_stake_registry = StakeRegistry::new(stake_registry, &provider);

        let delegation_manager_return = contract_stake_registry.delegation().call().await?;

        let StakeRegistry::delegationReturn {
            _0: delegation_manager_addr,
        } = delegation_manager_return;
        let avs_directory_addr = contract_service_manager_base.avsDirectory().call().await?;

        let ServiceManagerBase::avsDirectoryReturn { _0: avs_directory } = avs_directory_addr;

        let el_reader =
            ELChainReader::build(delegation_manager_addr, avs_directory, &self.provider).await?;

        Ok(AvsRegistryChainWriter {
            service_manager_addr: service_manager,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr: stake_registry,
            bls_apk_registry_addr: bls_apk_registry,
            el_reader,
            provider: self.provider.clone(),
            signer: self.signer.clone(),
        })
    }

    pub async fn register_operator_in_quorum_with_avs_registry_coordinator(
        &self,
        bls_key_pair: KeyPair,
        operator_to_avs_registration_sig_salt: FixedBytes<32>,
        operator_to_avs_registration_sig_expiry: U256,
        quorum_numbers: Bytes,
        socket: String,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        let provider = get_signer(self.signer.clone(), &self.provider);
        let wallet = LocalWallet::from_str(&self.signer).expect("failed to generate wallet ");

        // tracing info
        info!(avs_service_manager = %self.service_manager_addr, operator= %wallet.address(),quorum_numbers = ?quorum_numbers,"quorum_numbers,registering operator with the AVS's registry coordinator");
        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);
        let g1_hashes_msg_to_sign_return = contract_registry_coordinator
            .pubkeyRegistrationMessageHash(wallet.address())
            .call()
            .await?;
        let RegistryCoordinator::pubkeyRegistrationMessageHashReturn {
            _0: g1_hashes_msg_to_sign,
        } = g1_hashes_msg_to_sign_return;
        let signed_msg = convert_to_bn254_g1_point(
            bls_key_pair
                .sign_hashes_to_curve_message(G1Projective::from(
                    convert_bn254_to_ark(G1Point {
                        X: g1_hashes_msg_to_sign.X,
                        Y: g1_hashes_msg_to_sign.Y,
                    })
                    .point,
                ))
                .sig(),
        );

        let g1_pubkey_bn254 = convert_to_bn254_g1_point(bls_key_pair.get_pub_key_g1());
        let g2_projective = bls_key_pair
            .get_pub_key_g2()
            .expect("Failed to get g2 projective");

        let g2_pubkey_bn254 = convert_to_bn254_g2_point(g2_projective);

        let pub_key_reg_params = PubkeyRegistrationParams {
            pubkeyRegistrationSignature: signed_msg,
            pubkeyG1: g1_pubkey_bn254,
            pubkeyG2: g2_pubkey_bn254,
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
            .sign_message_sync(msg_to_sign.as_slice())
            .expect("failed to sign message");

        let operator_signature_with_salt_and_expiry = SignatureWithSaltAndExpiry {
            signature: operator_signature.as_bytes().into(),
            salt: operator_to_avs_registration_sig_salt,
            expiry: operator_to_avs_registration_sig_expiry,
        };

        let contract_call = contract_registry_coordinator.registerOperator(
            quorum_numbers.clone(),
            socket,
            RegistryPubkeyRegistrationParams {
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

        let tx_call = contract_call.gas(2000000);
        let tx = tx_call.send().await?;

        // tracing info
        info!(tx_hash = %tx.tx_hash(), avs_service_manager = %self.service_manager_addr,operator = %wallet.address(),quorum_numbers = ?quorum_numbers , "successfully registered operator with AVS registry coordinator");
        Ok(*tx.tx_hash())
    }

    pub async fn update_stakes_of_entire_operator_set_for_quorums(
        &self,
        operators_per_quorum: Vec<Vec<Address>>,
        quorum_number: Bytes,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        info!(quorum_numbers = %quorum_number, "updating stakes for entire operator set");
        let provider = get_signer(self.signer.clone(), &self.provider);
        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call = contract_registry_coordinator
            .updateOperatorsForQuorum(operators_per_quorum, quorum_number.clone());

        let tx = contract_call.send().await?;

        // tracing info
        info!(tx_hash = ?tx, quorum_numbers = %quorum_number,"succesfully updated stakes for entire operator set" );
        return Ok(*tx.tx_hash());
    }

    pub async fn update_stakes_of_operator_subset_for_all_quorums(
        &self,
        operators: Vec<Address>,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        info!(operators = ?operators, "updating stakes of operator subset for all quorums");

        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call = contract_registry_coordinator.updateOperators(operators);

        let tx = contract_call.send().await?;

        info!(tx_hash = ?tx,"ssuccesfully updated stakes of operator subset for all quorums" );
        Ok(*tx.tx_hash())
    }

    pub async fn deregister_operator(
        &self,
        quorum_numbers: Bytes,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        info!("deregistering operator with the AVS's registry coordinator");
        let provider = get_signer(self.signer.clone(), &self.provider);

        let contract_registry_coordinator =
            RegistryCoordinator::new(self.registry_coordinator_addr, provider);

        let contract_call = contract_registry_coordinator.deregisterOperator(quorum_numbers);

        let tx = contract_call.send().await?;

        info!(tx_hash = ?tx,"succesfully deregistered operator with the AVS's registry coordinator" );
        return Ok(*tx.tx_hash());
    }
}
