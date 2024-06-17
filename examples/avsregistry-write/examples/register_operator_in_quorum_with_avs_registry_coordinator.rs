//! register operator in quorum with avs registry coordinator
use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_testing_utils::m2_holesky_constants::{
    AVS_DIRECTORY_ADDRESS, BLS_APK_REGISTRY, DELEGATION_MANAGER_ADDRESS, OPERATOR_STATE_RETRIEVER,
    REGISTRY_COORDINATOR, SERVICE_MANAGER_ADDRESS, SLASHER_ADDRESS, STAKE_REGISTRY,
};
use ark_bn254::Fr;
use eigen_crypto_bls::attestation::KeyPair;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let holesky_provider = "https://holesky.drpc.org";
    let pvt_key = "";
    let el_chain_reader: ELChainReader = ELChainReader::new(
        SLASHER_ADDRESS,
        DELEGATION_MANAGER_ADDRESS,
        AVS_DIRECTORY_ADDRESS,
        holesky_provider.to_string(),
    );
    let avs_registry_writer = AvsRegistryChainWriter::new(
        SERVICE_MANAGER_ADDRESS,
        REGISTRY_COORDINATOR,
        OPERATOR_STATE_RETRIEVER,
        STAKE_REGISTRY,
        BLS_APK_REGISTRY,
        el_chain_reader,
        holesky_provider.to_string(),
        pvt_key.to_string(),
    )
    .await;


    let key_pair :KeyPair = KeyPair::new();

    let tx_hash = avs_registry_writer.register_operator_in_quorum_with_avs_registry_coordinator();

    Ok(())
}
