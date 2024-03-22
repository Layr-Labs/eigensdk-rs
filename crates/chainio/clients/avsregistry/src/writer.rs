use ark_bn254::G1Projective;
use eigensdk_chainio_utils::{
    convert_bn254_to_ark, convert_to_bn254_g1_point, convert_to_bn254_g2_point,
};
use eigensdk_client_elcontracts::reader::ELChainReader;
use eigensdk_contracts_bindings::{
    BLSApkRegistry::bls_apk_registry,
    OperatorStateRetriever::operator_state_retriever,
    RegistryCoordinator::{
        registry_coordinator::{self, REGISTRYCOORDINATOR_ABI},
        G1Point, PubkeyRegistrationParams, RegistryCoordinator, SignatureWithSaltAndExpiry,
    },
    ServiceManagerBase::service_manager_base,
    StakeRegistry::stake_registry,
};
use eigensdk_crypto_bls::attestation::KeyPair;
use eigensdk_logging::logger::Logger;
use eigensdk_txmgr::{SimpleTxManager, TxManager};
use ethers_core::types::{Address, Bytes, TxHash, H256, U256};
use ethers_providers::{Http, Middleware, Provider};
use std::sync::Arc;
use std::{any::Any, str::FromStr};

use ethers::{
    abi::AbiEncode,
    middleware::{MiddlewareBuilder, SignerMiddleware},
    signers::{Signer, Wallet},
};

pub struct AvsRegistryChainWriter {
    service_manager_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever_addr: Address,
    stake_registry_addr: Address,
    bls_apk_registry_addr: Address,
    el_reader: ELChainReader,
    logger: Logger,
    client: Provider<Http>,
    tx_mgr: SimpleTxManager,
}

trait AvsRegistryWriter {}

impl AvsRegistryChainWriter {
    async fn new_avs_registry_chain_writer(
        service_manager_addr: Address,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        stake_registry_addr: Address,
        bls_apk_registry_addr: Address,
        el_reader: ELChainReader,
        logger: Logger,
        client: Provider<Http>,
        tx_mgr: SimpleTxManager,
    ) -> Self {
        AvsRegistryChainWriter {
            service_manager_addr,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr,
            bls_apk_registry_addr,
            el_reader,
            logger,
            client,
            tx_mgr,
        }
    }

    async fn build_avs_registry_chain_writer(
        &self,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        logger: Logger,
        client: Provider<Http>,
        tx_mgr: SimpleTxManager,
    ) -> Self {
        let provider = Arc::new(client.clone());
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            registry_coordinator_addr,
            provider.clone(),
        );

        let contract_operator_state_retriever =
            operator_state_retriever::OperatorStateRetriever::new(
                operator_state_retriever_addr,
                provider.clone(),
            );

        let service_manager_addr = contract_registry_coordinator
            .service_manager()
            .call()
            .await
            .unwrap();

        let contract_service_manager_base =
            service_manager_base::ServiceManagerBase::new(service_manager_addr, provider.clone());

        let bls_apk_registry_addr = contract_registry_coordinator
            .bls_apk_registry()
            .call()
            .await
            .unwrap();

        let contract_bls_apk_registry =
            bls_apk_registry::BLSApkRegistry::new(bls_apk_registry_addr, provider.clone());

        let stake_registry_addr = contract_registry_coordinator
            .stake_registry()
            .call()
            .await
            .unwrap();

        let contract_stake_registry =
            stake_registry::StakeRegistry::new(stake_registry_addr, provider);

        let delegation_manager_addr = contract_stake_registry.delegation().call().await.unwrap();

        let avs_directory_addr = contract_service_manager_base
            .avs_directory()
            .call()
            .await
            .unwrap();

        let el_reader = ELChainReader::build(
            delegation_manager_addr,
            avs_directory_addr,
            logger.clone(),
            client.clone(),
        )
        .await;

        return AvsRegistryChainWriter {
            service_manager_addr,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr,
            bls_apk_registry_addr,
            el_reader,
            logger,
            client,
            tx_mgr,
        };
    }

    async fn reigster_operator_in_quorum_with_avs_registry_coordinator(
        &self,
        pvt_key: &str,
        bls_key_pair: KeyPair,
        operator_to_avs_registration_sig_salt: [u8; 32],
        operator_to_avs_registration_sig_expiry: U256,
        quorum_numbers: Bytes,
        socket: String,
    ) -> Result<TxHash, String> {
        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );
        let wallet = Wallet::from_str(pvt_key).unwrap();
        let g1_hashes_msg_to_sign = contract_registry_coordinator
            .pubkey_registration_message_hash(wallet.address())
            .call()
            .await
            .unwrap();

        let x_point = g1_hashes_msg_to_sign.x;
        let y_point = g1_hashes_msg_to_sign.y;

        let signed_msg = convert_to_bn254_g1_point(
            bls_key_pair
                .sign_hashes_to_curve_message(G1Projective::from(
                    convert_bn254_to_ark(g1_hashes_msg_to_sign).point,
                ))
                .sig(),
        );

        let g1_pubkey_bn254 = convert_to_bn254_g1_point(bls_key_pair.get_pub_key_g1());
        let g2_pubkey_bn254 = convert_to_bn254_g2_point(bls_key_pair.gt_pub_key_g2());

        let pub_key_reg_params = PubkeyRegistrationParams {
            pubkey_registration_signature: signed_msg,
            pubkey_g1: g1_pubkey_bn254,
            pubkey_g2: g2_pubkey_bn254,
        };

        let msg_to_sign = self
            .el_reader
            .calculate_operator_avs_registration_digest_hash(
                wallet.address(),
                self.service_manager_addr,
                operator_to_avs_registration_sig_salt,
                operator_to_avs_registration_sig_expiry,
            )
            .await
            .unwrap();

        let operator_signature = wallet.sign_message(msg_to_sign).await.unwrap();

        let operator_signature_with_salt_and_expiry = SignatureWithSaltAndExpiry {
            signature: operator_signature.to_vec().into(),
            salt: operator_to_avs_registration_sig_salt,
            expiry: operator_to_avs_registration_sig_expiry,
        };

        let contract_call = contract_registry_coordinator.register_operator(
            quorum_numbers,
            socket,
            pub_key_reg_params,
            operator_signature_with_salt_and_expiry,
        );

        let tx = contract_call.send().await.unwrap();

        Ok(tx.tx_hash())
    }

    async fn update_stakes_of_entire_operator_set_for_quorums(
        &self,
        operators_per_quorum: Vec<Vec<Address>>,
        quorum_number: Bytes,
    ) -> Result<TxHash, String> {
        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );

        let contract_call = contract_registry_coordinator
            .update_operators_for_quorum(operators_per_quorum, quorum_number);

        let tx = contract_call.send().await.unwrap();

        return Ok(tx.tx_hash());
    }

    async fn deregister_operator(
        &self,
        quorum_numbers: Bytes,
        pub_key: G1Point,
    ) -> Result<TxHash, String> {
        let provider = Arc::new(&self.client);
        let wallet = self.tx_mgr.wallet.signer.clone();
        let signer = SignerMiddleware::new(provider.clone(), wallet);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            signer.into(),
        );

        let contract_call = contract_registry_coordinator.deregister_operator(quorum_numbers);

        let tx = contract_call.send().await.unwrap();

        return Ok(tx.tx_hash());
    }
}
