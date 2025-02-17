//! register operator in quorum with avs registry coordinator
use alloy::primitives::U256;
use alloy::primitives::{Bytes, FixedBytes};
use alloy::signers::local::PrivateKeySigner;
use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_client_elcontracts::writer::ELChainWriter;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_test_logger;
use eigen_testing_utils::anvil_constants::get_registry_coordinator_address;
use eigen_testing_utils::m2_holesky_constants::{
    AVS_DIRECTORY_ADDRESS, DELEGATION_MANAGER_ADDRESS, OPERATOR_STATE_RETRIEVER,
    REGISTRY_COORDINATOR, REWARDS_COORDINATOR, STRATEGY_MANAGER_ADDRESS,
};
use eigen_types::operator::Operator;
use eyre::Result;
use lazy_static::lazy_static;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    /// 1 day
    static ref SIGNATURE_EXPIRY: U256 = U256::from(86400);
}
#[tokio::main]
#[allow(clippy::expect_used)]
async fn main() -> Result<()> {
    let holesky_provider = "https://ethereum-holesky.blockpi.network/v1/rpc/public";
    let pvt_key = "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8";
    let test_logger = get_test_logger();
    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        test_logger.clone(),
        holesky_provider.to_string(),
        pvt_key.to_string(),
        REGISTRY_COORDINATOR,
        OPERATOR_STATE_RETRIEVER,
    )
    .await
    .expect("avs writer build fail ");

    // Create a new key pair instance using the secret key
    let bls_key_pair = BlsKeyPair::new(
        "12248929636257230549931416853095037629726205319386239410403476017439825112537".to_string(),
    )?;

    let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);

    // Get the current SystemTime
    let now = SystemTime::now();
    let mut sig_expiry: U256 = U256::from(0);
    // Convert SystemTime to a Duration since the UNIX epoch
    if let Ok(duration_since_epoch) = now.duration_since(UNIX_EPOCH) {
        // Convert the duration to seconds
        let seconds = duration_since_epoch.as_secs(); // Returns a u64

        // Convert seconds to U256
        sig_expiry = U256::from(seconds) + *SIGNATURE_EXPIRY;
    } else {
        println!("System time seems to be before the UNIX epoch.");
    }
    let quorum_nums = Bytes::from([0x01]);

    // A new ElChainReader instance
    let el_chain_reader = ELChainReader::new(
        get_test_logger().clone(),
        None,
        DELEGATION_MANAGER_ADDRESS,
        REWARDS_COORDINATOR,
        AVS_DIRECTORY_ADDRESS,
        None,
        holesky_provider.to_string(),
    );

    let registry_coordinator = get_registry_coordinator_address(holesky_provider.to_string()).await;

    // A new ElChainWriter instance
    let el_writer = ELChainWriter::new(
        STRATEGY_MANAGER_ADDRESS,
        REWARDS_COORDINATOR,
        None,
        None,
        registry_coordinator,
        el_chain_reader,
        holesky_provider.to_string(),
        "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8".to_string(),
    );

    let wallet = PrivateKeySigner::from_str(
        "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8",
    )
    .expect("no key ");

    let operator_details = Operator {
        address: wallet.address(),
        delegation_approver_address: wallet.address(),
        staker_opt_out_window_blocks: 3,
        metadata_url: "eigensdk-rs".to_string(),
        allocation_delay: 1,
    };
    // Register the address as operator in delegation manager
    let _s = el_writer.register_as_operator(operator_details).await;

    // Register the operator in registry coordinator
    avs_registry_writer
        .register_operator_in_quorum_with_avs_registry_coordinator(
            bls_key_pair,
            digest_hash,
            sig_expiry,
            quorum_nums,
            "65.109.158.181:33078;31078".to_string(), // socket
        )
        .await?;
    Ok(())
}
