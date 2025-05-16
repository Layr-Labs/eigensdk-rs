use std::str::FromStr;

use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::k256::elliptic_curve::consts::U2;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::{
    hex,
    primitives::{aliases::U96, Address, FixedBytes, U256},
};
use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_client_elcontracts::writer::ELChainWriter;
use eigen_logging::logger::SharedLogger;

use crate::error::OperatorError;
use crate::register_config::OperatorRegistrationConfig;

pub async fn register_operator(
    config: OperatorRegistrationConfig,
    logger: SharedLogger,
    http_rpc_url: String,
) -> Result<bool, OperatorError> {
    let signer: LocalSigner<SigningKey> = if let Some(operator_key) = config.operator_pvt_key {
        PrivateKeySigner::from_str(&operator_key).map_err(|_| OperatorError::BlsKeystoreError)?
    } else {
        LocalSigner::decrypt_keystore(config.ecdsa_keystore_path, config.ecdsa_keystore_password)?
    };

    let el_chain_reader = ELChainReader::new(
        logger,
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
    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        logger,
        http_rpc_url.to_string(),
        hex::encode(signer.to_field_bytes()).to_string(),
        config.registry_coordinator_address,
        config.avs_address,
    )
    .await?;

    create_total_delegated_stake_quorum(config.erc20_strategy_address, avs_registry_writer).await?;

    register_operator_with_el(
        config.metadata_uri,
        config.allocation_delay,
        signer.clone(),
        el_chain_reader,
        el_chain_writer.clone(),
    )
    .await?;
    deposit_into_strategy(
        config.erc20_strategy_address,
        U256::from_str(&config.deposit_tokens).map_err(|_| OperatorError::InvalidDepositTokens)?,
        el_chain_writer.clone(),
    )
    .await?;

    set_allocation_delay(
        config.allocation_delay,
        signer.clone(),
        el_chain_writer.clone(),
    )
    .await?;

    modify_allocation_for_operator(
        config.operator_set_id,
        config.avs_address,
        vec![config.erc20_strategy_address],
        config.new_magnitude,
        el_chain_writer.clone(),
        signer.address(),
    )
    .await?;

    register_for_operator_sets(
        config.operator_set_id,
        config.bls_key_pair,
        config.avs_address,
        config.socket,
        signer,
        el_chain_writer,
    )
    .await?;

    Ok(())
}

/// Registers Operator in EigenLayer
///
/// # Arguments
///
/// * `metadata_uri` - The metadata URI for the operator
/// * `allocation_delay` - The allocation delay for the operator
/// * `signer` - The signer for the operator
/// * `el_chain_reader` - The EL chain reader
/// * `el_chain_writer` - The EL chain writer
///
/// # Returns
///
/// * `eyre::Result<()>` - The result of the operation
async fn register_operator_with_el(
    metadata_uri: String,
    allocation_delay: u32,
    signer: LocalSigner<SigningKey>,
    el_chain_reader: ELChainReader,
    el_chain_writer: ELChainWriter,
) -> eyre::Result<()> {
    let operator_details = Operator {
        address: signer.address(),
        delegation_approver_address: signer.address(),
        staker_opt_out_window_blocks: Some(0),
        metadata_url: metadata_uri,
        allocation_delay: Some(allocation_delay),
        _deprecated_earnings_receiver_address: None,
    };
    let is_already_registered = el_chain_reader
        .is_operator_registered(signer.address())
        .await?;
    if !is_already_registered {
        let _ = el_chain_writer
            .register_as_operator(operator_details)
            .await?;
    }
    Ok(())
}

/// Sets the allocation delay for the operator
///
/// # Arguments
///
/// * `allocation_delay` - The allocation delay for the operator
/// * `signer` - The signer for the operator
/// * `el_chain_writer` - The EL chain writer
///
/// # Returns
///
/// * `eyre::Result<FixedBytes<32>>` - The result of the operation
async fn set_allocation_delay(
    allocation_delay: u32,
    signer: LocalSigner<SigningKey>,
    el_chain_writer: ELChainWriter,
) -> eyre::Result<FixedBytes<32>> {
    Ok(el_chain_writer
        .set_allocation_delay(signer.address(), allocation_delay)
        .await?)
}

/// Creates Total Delegated Stake Quorum
///
/// # Arguments
///
/// * `strategy_address` - The address of the strategy
/// * `avs_registry_writer` - The AVS registry writer
///
/// # Returns
///
/// * `eyre::Result<FixedBytes<32>>` - The result of the operation
async fn create_total_delegated_stake_quorum(
    strategy_address: Address,
    avs_registry_writer: AvsRegistryChainWriter,
) -> eyre::Result<FixedBytes<32>> {
    let operator_set_param = OperatorSetParam {
        maxOperatorCount: 3,
        kickBIPsOfOperatorStake: 100,
        kickBIPsOfTotalStake: 1000,
    };
    let minimum_stake = U96::from(0);
    let strategy_params = vec![StrategyParams {
        strategy: strategy_address,
        multiplier: U96::from(1),
    }];

    let s = avs_registry_writer
        .create_total_delegated_stake_quorum(operator_set_param, minimum_stake, strategy_params)
        .await?;
    Ok(s)
}

/// Register Operator for Operator Sets
///
/// # Arguments
///
/// * `operator_set_id` - The ID of the operator set
/// * `bls_key_pair` - The BLS key pair for the operator
/// * `avs` - The address of the AVS
/// * `socket` - The socket for the operator
/// * `signer` - The signer for the operator
/// * `el_chain_writer` - The EL chain writer
///
/// # Returns
///
/// * `eyre::Result<FixedBytes<32>>` - The result of the operation
async fn register_for_operator_sets(
    operator_set_id: u32,
    bls_key_pair: BlsKeyPair,
    avs: Address,
    socket: String,
    signer: LocalSigner<SigningKey>,
    el_chain_writer: ELChainWriter,
) -> eyre::Result<FixedBytes<32>> {
    Ok(el_chain_writer
        .register_for_operator_sets(
            signer.address(),
            avs,
            [operator_set_id].to_vec(),
            bls_key_pair,
            &socket,
        )
        .await?)
}

/// Deposits ERC20 into Strategy
///
/// # Arguments
///
/// * `strategy_address` - The address of the strategy
/// * `amount` - The amount to deposit
/// * `el_writer` - The EL chain writer
///
/// # Returns
///
/// * `Result<(), ElContractsError>` - The result of the operation
async fn deposit_into_strategy(
    strategy_address: Address,
    amount: U256,
    el_writer: ELChainWriter,
) -> Result<(), ElContractsError> {
    el_writer
        .deposit_erc20_into_strategy(strategy_address, amount)
        .await?;
    Ok(())
}

/// Modifies the allocation magnitude for the operator in specific strategies
///
/// # Arguments
///
/// * `operator_set_id` - The ID of the operator set
/// * `avs` - The address of the AVS
/// * `strategies` - The strategies to modify the allocation for
/// * `new_magnitude` - The new magnitude for the allocation
/// * `el_writer` - The EL chain writer
/// * `operator_address` - The address of the operator
///
/// # Returns
///
/// * `eyre::Result<FixedBytes<32>>` - The result of the operation
pub async fn modify_allocation_for_operator(
    operator_set_id: u32,
    avs: Address,
    strategies: Vec<Address>,
    new_magnitude: Vec<u64>,
    el_writer: ELChainWriter,
    operator_address: Address,
) -> eyre::Result<FixedBytes<32>> {
    let allocate_params = vec![AllocateParams {
        operatorSet: OperatorSet {
            avs,
            id: operator_set_id,
        },
        strategies,
        newMagnitudes: new_magnitude,
    }];
    Ok(el_writer
        .modify_allocations(operator_address, allocate_params)
        .await?)
}
