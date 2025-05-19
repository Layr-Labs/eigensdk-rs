use std::fs;
use std::path::Path;
use std::str::FromStr;

use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::{
    hex,
    primitives::{aliases::U96, Address, FixedBytes, U256},
};

use eigensdk::client_avsregistry::writer::AvsRegistryChainWriter;
use eigensdk::client_elcontracts::error::ElContractsError;
use eigensdk::crypto_bls::BlsKeyPair;
use eigensdk::operator::register_config::OperatorRegistrationConfig;
use eigensdk::utils::slashing::core::allocationmanager::AllocationManager::OperatorSet;
use eigensdk::utils::slashing::core::allocationmanager::IAllocationManagerTypes::AllocateParams;
use eigensdk::utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::stakeregistry::IStakeRegistryTypes::StrategyParams;
use eigensdk::{
    client_elcontracts::{reader::ELChainReader, writer::ELChainWriter},
    logging::get_logger,
    types::operator::Operator,
};
use eyre::Result;
use serde::de::DeserializeOwned;

/// Loads a config from a file
///
/// # Arguments
///
/// * `path` - The path to the config file
///
/// # Returns
///
/// * `eyre::Result<T>` - The config struct
pub fn load_config<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let s =
        fs::read_to_string(&path).map_err(|e| eyre::eyre!("Could not read config file: {}", e))?;
    toml::from_str(&s).map_err(|e| eyre::eyre!("Could not parse config file: {}", e))
}

/// Sets up the operator
/// TODO: Move this to the SDK with the OperatorRegistryConfig struct
///
/// # Arguments
///
/// * `config` - The config for the operator
/// * `bls_key_pair` - The BLS key pair for the operator
/// * `http_rpc_url` - The HTTP RPC URL
///
/// # Returns
///
/// * `eyre::Result<()>` - The result of the operation
pub async fn setup_operator(
    config: OperatorRegistrationConfig,
    bls_key_pair: BlsKeyPair,
    http_rpc_url: String,
) -> Result<()> {
    let signer: LocalSigner<SigningKey> = if let Some(operator_key) = config.operator_pvt_key {
        PrivateKeySigner::from_str(&operator_key)?
    } else {
        LocalSigner::decrypt_keystore(config.ecdsa_keystore_path, config.ecdsa_keystore_password)?
    };

    let el_chain_reader = ELChainReader::new(
        get_logger(),
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
        get_logger(),
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
        U256::from_str(&config.deposit_tokens).unwrap(),
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
        bls_key_pair,
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
