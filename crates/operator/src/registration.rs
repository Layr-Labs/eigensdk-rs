use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::{hex, primitives::U256};
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_client_elcontracts::writer::ELChainWriter;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::logger::SharedLogger;
use eigen_signer::signer::Config as EcdsaSignerConfig;
use eigen_types::operator::Operator;
use eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet;
use eigen_utils::slashing::core::allocationmanager::IAllocationManagerTypes::AllocateParams;
use std::str::FromStr;

use crate::error::OperatorError;
use crate::register_config::OperatorRegistrationConfig;

/// Registers an operator with EigenLayer. Use this function for testing purposes.
///
/// 1. Registers the operator with EigenLayer
/// 2. Deposits ERC20 into the strategy
/// 3. Sets the allocation delay
/// 4. Modifies the allocation magnitude for the operator in specific strategies
/// 5. Registers the operator for operator sets
///
/// # Arguments
///
/// * `config` - The operator registration config
/// * `logger` - The logger
/// * `http_rpc_url` - The HTTP RPC URL
/// * `bls_key_pair` - The BLS key pair
///
/// # Returns
///
/// * `Result<(), OperatorError>` - The result of the operation
pub async fn register_operator(
    config: OperatorRegistrationConfig,
    logger: SharedLogger,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorError> {
    let signer: LocalSigner<SigningKey> = match config.signer {
        EcdsaSignerConfig::PrivateKey { key: private_key } => {
            PrivateKeySigner::from_str(&private_key)?
        }
        EcdsaSignerConfig::Keystore {
            path: keystore_path,
            password: keystore_password,
        } => LocalSigner::decrypt_keystore(keystore_path, keystore_password)?,
    };

    let el_chain_reader = ELChainReader::new(
        logger.clone(),
        Some(config.allocation_manager_address),
        config.delegation_manager_address,
        config.rewards_coordinator_address,
        config.avs_directory_address,
        Some(config.permission_controller_address),
        http_rpc_url.clone(),
    );
    let el_chain_writer = ELChainWriter::new(
        config.strategy_manager_address,
        config.rewards_coordinator_address,
        Some(config.permission_controller_address),
        Some(config.allocation_manager_address),
        config.registry_coordinator_address,
        el_chain_reader.clone(),
        http_rpc_url.clone(),
        hex::encode(signer.to_field_bytes()).to_string(),
    );

    let operator_details = Operator {
        address: signer.address(),
        delegation_approver_address: signer.address(),
        staker_opt_out_window_blocks: Some(0),
        metadata_url: config.metadata_uri,
        allocation_delay: Some(config.allocation_delay),
        _deprecated_earnings_receiver_address: None,
    };

    el_chain_writer
        .register_as_operator(operator_details)
        .await?;

    el_chain_writer
        .deposit_erc20_into_strategy(
            config.erc20_strategy_address,
            U256::from_str(&config.deposit_tokens)
                .map_err(|_| OperatorError::InvalidDepositTokens)?,
        )
        .await?;

    el_chain_writer
        .set_allocation_delay(signer.address(), config.allocation_delay)
        .await?;

    let allocate_params = vec![AllocateParams {
        operatorSet: OperatorSet {
            avs: config.avs_address,
            id: config.operator_set_id,
        },
        strategies: vec![config.erc20_strategy_address],
        newMagnitudes: config.new_magnitude,
    }];
    el_chain_writer
        .modify_allocations(signer.address(), allocate_params)
        .await?;

    el_chain_writer
        .register_for_operator_sets(
            signer.address(),
            config.avs_address,
            vec![config.operator_set_id],
            bls_key_pair,
            &config.socket,
        )
        .await?;

    Ok(())
}
