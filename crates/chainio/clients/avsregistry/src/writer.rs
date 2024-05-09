use crate::error::AvsRegistryError;
use alloy_sol_types::sol;
use ark_bn254::G1Projective;
use eigensdk_chainio_utils::{
    convert_bn254_to_ark, convert_to_bn254_g1_point, convert_to_bn254_g2_point,
};
use eigensdk_client_elcontracts::reader::ELChainReader;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    RegistryCoordinator,
    "../../../../crates/contracts/bindings/utils/json/RegistryCoordinator.json"
);
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ServiceManagerBase,
    "../../../../crates/contracts/bindings/utils/json/ServiceManagerBase.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StakeRegistry,
    "StakeRegistry.json"
);

use alloy_primitives::{Address, Bytes, TxHash, U256};
use alloy_provider::{Provider, ProviderBuilder};
use eigensdk_crypto_bls::attestation::KeyPair;
use eigensdk_txmgr::SimpleTxManager;
use eigensdk_types::avs;
use std::sync::Arc;
use tracing::info;

// use ethers::{
//     middleware::SignerMiddleware,
//     signers::{Signer, Wallet},
// };

pub struct AvsRegistryChainWriter {
    service_manager_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever_addr: Address,
    stake_registry_addr: Address,
    bls_apk_registry_addr: Address,
    el_reader: ELChainReader,
    provider: String,
    tx_mgr: SimpleTxManager,
}

impl AvsRegistryChainWriter {
    async fn new_avs_registry_chain_writer(
        service_manager_addr: Address,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        stake_registry_addr: Address,
        bls_apk_registry_addr: Address,
        el_reader: ELChainReader,
        provider: String,
        tx_mgr: SimpleTxManager,
    ) -> Self {
        AvsRegistryChainWriter {
            service_manager_addr,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr,
            bls_apk_registry_addr,
            el_reader,
            provider,
            tx_mgr,
        }
    }

    async fn build_avs_registry_chain_writer(
        &self,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        tx_mgr: SimpleTxManager,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_builtin(&self.provider)
            .await?;
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
        let contract_stake_registry = StakeRegistry::new(stake_registry, provider);

        let delegation_manager_addr = contract_stake_registry.delegation().call().await?;

        let avs_directory_addr = contract_service_manager_base.avsDirectory().call().await?;

        let ServiceManagerBase::avsDirectoryReturn { _0: avs_directory } = avs_directory_addr;

        let el_reader =
            ELChainReader::build(delegation_manager_addr, avs_directory, &provider).await?;

        return Ok(AvsRegistryChainWriter {
            service_manager_addr: service_manager,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr: stake_registry,
            bls_apk_registry_addr: bls_apk_registry,
            el_reader,
            provider: self.provider,
            tx_mgr,
        });
    }

    async fn reigster_operator_in_quorum_with_avs_registry_coordinator(
        &self,
        pvt_key: &str,
        bls_key_pair: KeyPair,
        operator_to_avs_registration_sig_salt: [u8; 32],
        operator_to_avs_registration_sig_expiry: U256,
        quorum_numbers: Bytes,
        socket: String,
    ) -> Result<TxHash, AvsRegistryError> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_builtin(&self.provider)
            .await?;
        let wallet = self.tx_mgr.wallet.signer.clone();
        // tracing info
        // info!(avs_service_manager = %self.service_manager_addr, operator= %wallet.address(),quorum_numbers = ?quorum_numbers,"quorum_numbers,registering operator with the AVS's registry coordinator");
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );
        let wallet = Wallet::from_str(pvt_key).unwrap();
        let g1_hashes_msg_to_sign_result = contract_registry_coordinator
            .pubkey_registration_message_hash(wallet.address())
            .call()
            .await;

        match g1_hashes_msg_to_sign_result {
            Ok(g1_hashes_msg_to_sign) => {
                let signed_msg = convert_to_bn254_g1_point(
                    bls_key_pair
                        .sign_hashes_to_curve_message(G1Projective::from(
                            convert_bn254_to_ark(g1_hashes_msg_to_sign).point,
                        ))
                        .sig(),
                );

                let g1_pubkey_bn254 = convert_to_bn254_g1_point(bls_key_pair.get_pub_key_g1());
                let g2_projective_result = bls_key_pair.gt_pub_key_g2();

                match g2_projective_result {
                    Ok(g2_projective) => {
                        let g2_pubkey_bn254 = convert_to_bn254_g2_point(g2_projective);

                        let pub_key_reg_params = PubkeyRegistrationParams {
                            pubkey_registration_signature: signed_msg,
                            pubkey_g1: g1_pubkey_bn254,
                            pubkey_g2: g2_pubkey_bn254,
                        };

                        let msg_to_sign_result = self
                            .el_reader
                            .calculate_operator_avs_registration_digest_hash(
                                wallet.address(),
                                self.service_manager_addr,
                                operator_to_avs_registration_sig_salt,
                                operator_to_avs_registration_sig_expiry,
                            )
                            .await;

                        match msg_to_sign_result {
                            Ok(msg_to_sign) => {
                                let operator_signature =
                                    wallet.sign_message(msg_to_sign).await.unwrap();

                                let operator_signature_with_salt_and_expiry =
                                    SignatureWithSaltAndExpiry {
                                        signature: operator_signature.to_vec().into(),
                                        salt: operator_to_avs_registration_sig_salt,
                                        expiry: operator_to_avs_registration_sig_expiry,
                                    };

                                let contract_call = contract_registry_coordinator
                                    .register_operator(
                                        quorum_numbers.clone(),
                                        socket,
                                        pub_key_reg_params,
                                        operator_signature_with_salt_and_expiry,
                                    );

                                let tx_result = contract_call.send().await;

                                match tx_result {
                                    Ok(tx) => {
                                        // tracing info
                                        info!(tx_hash = %tx.tx_hash(), avs_service_manager = %self.service_manager_addr,operator = %wallet.address(),quorum_numbers = ?quorum_numbers , "successfully registered operator with AVS registry coordinator");
                                        Ok(tx.tx_hash())
                                    }
                                    Err(_) => return Err(AvsRegistryError::RegisterOperator),
                                }
                            }
                            Err(_) => {
                                return Err(
                                    AvsRegistryError::CalculateOperatorAvsRegistrationDigestHash,
                                );
                            }
                        }
                    }
                    Err(_) => return Err(AvsRegistryError::PUbKeyG2),
                }
            }
            Err(_) => return Err(AvsRegistryError::PubKeyRegistrationMessageHash),
        }
    }

    async fn update_stakes_of_entire_operator_set_for_quorums(
        &self,
        operators_per_quorum: Vec<Vec<Address>>,
        quorum_number: Bytes,
    ) -> Result<TxHash, AvsRegistryError> {
        info!(quorum_numbers = %quorum_number, "updating stakes for entire operator set");
        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );

        let contract_call = contract_registry_coordinator
            .update_operators_for_quorum(operators_per_quorum, quorum_number.clone());

        let tx_result = contract_call.send().await;

        match tx_result {
            Ok(tx) => {
                // tracing info
                info!(tx_hash = ?tx, quorum_numbers = %quorum_number,"succesfully updated stakes for entire operator set" );
                return Ok(tx.tx_hash());
            }
            Err(_) => return Err(AvsRegistryError::UpdateOperatorForQuorum),
        }
    }

    async fn update_stakes_of_operator_subset_for_all_quorums(
        &self,
        operators: Vec<Address>,
    ) -> Result<TxHash, AvsRegistryError> {
        info!(operators = ?operators, "updating stakes of operator subset for all quorums");

        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );

        let contract_call = contract_registry_coordinator.update_operators(operators);

        let tx_result = contract_call.send().await;

        match tx_result {
            Ok(tx) => {
                info!(tx_hash = ?tx,"ssuccesfully updated stakes of operator subset for all quorums" );
                Ok(tx.tx_hash())
            }
            Err(_) => {
                return Err(AvsRegistryError::UpdateStakeForAllQuorums);
            }
        }
    }

    async fn deregister_operator(
        &self,
        quorum_numbers: Bytes,
        pub_key: G1Point,
    ) -> Result<TxHash, AvsRegistryError> {
        info!("deregistering operator with the AVS's registry coordinator");
        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );

        let contract_call = contract_registry_coordinator.deregister_operator(quorum_numbers);

        let tx_result = contract_call.send().await;

        match tx_result {
            Ok(tx) => {
                info!(tx_hash = ?tx,"succesfully deregistered operator with the AVS's registry coordinator" );
                return Ok(tx.tx_hash());
            }
            Err(_) => return Err(AvsRegistryError::DeregisterOperator),
        }
    }
}
