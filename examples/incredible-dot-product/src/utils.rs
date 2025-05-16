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

/// Sets up the operator
/// TODO: Move this to the SDK with the OperatorRegistryConfig struct
///
/// # Arguments
///
/// * `bls_key_pair` - The BLS key pair for the operator
/// * `operator_pvt_key` - The private key for the operator
/// * `ecdsa_keystore_path` - The path to the ECDSA keystore
/// * `ecdsa_keystore_password` - The password for the ECDSA keystore
/// * `rpc_url` - The RPC URL
/// * `metadata_uri` - The metadata URI for the operator
/// * `socket` - The socket for the operator
/// * `allocation_delay` - The allocation delay for the operator
/// * `operator_set_id` - The ID of the operator set
/// * `deposit_tokens` - The amount of tokens to deposit into the strategy
/// * `new_magnitude` - The new magnitude for the allocation
/// * `permission_controller_address` - The address of the permission controller
/// * `rewards_coordinator_address` - The address of the rewards coordinator
/// * `allocation_manager` - The address of the allocation manager
/// * `registry_coordinator_address` - The address of the registry coordinator
/// * `delegation_manager_address` - The address of the delegation manager
/// * `avs_directory_address` - The address of the AVS directory
/// * `strategy_manager_address` - The address of the strategy manager
/// * `erc20_strategy_address` - The address of the ERC20 strategy
/// * `avs` - The address of the AVS
///
/// # Returns
///
/// * `eyre::Result<()>` - The result of the operation
#[allow(clippy::too_many_arguments)]
pub async fn setup_operator(
    bls_key_pair: BlsKeyPair,
    operator_pvt_key: Option<String>,
    ecdsa_keystore_path: String,
    ecdsa_keystore_password: String,
    rpc_url: String,
    metadata_uri: String,
    socket: String,
    allocation_delay: u32,
    operator_set_id: u32,
    deposit_tokens: U256,
    new_magnitude: Vec<u64>,
    permission_controller_address: Address,
    rewards_coordinator_address: Address,
    allocation_manager: Address,
    registry_coordinator_address: Address,
    delegation_manager_address: Address,
    avs_directory_address: Address,
    strategy_manager_address: Address,
    erc20_strategy_address: Address,
    avs: Address,
) -> Result<()> {
    let signer: LocalSigner<SigningKey> = if let Some(operator_key) = operator_pvt_key {
        PrivateKeySigner::from_str(&operator_key)?
    } else {
        LocalSigner::decrypt_keystore(ecdsa_keystore_path, ecdsa_keystore_password)?
    };

    let el_chain_reader = ELChainReader::new(
        get_logger(),
        Some(allocation_manager),
        delegation_manager_address,
        rewards_coordinator_address,
        avs_directory_address,
        Some(permission_controller_address),
        rpc_url.clone(),
    );
    let el_chain_writer = ELChainWriter::new(
        strategy_manager_address,
        rewards_coordinator_address,
        Some(permission_controller_address),
        Some(allocation_manager),
        registry_coordinator_address,
        el_chain_reader.clone(),
        rpc_url.clone(),
        hex::encode(signer.to_field_bytes()).to_string(),
    );
    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        get_logger(),
        rpc_url.to_string(),
        hex::encode(signer.to_field_bytes()).to_string(),
        registry_coordinator_address,
        avs,
    )
    .await?;

    create_total_delegated_stake_quorum(erc20_strategy_address, avs_registry_writer).await?;

    register_operator_with_el(
        metadata_uri,
        allocation_delay,
        signer.clone(),
        el_chain_reader,
        el_chain_writer.clone(),
    )
    .await?;
    deposit_into_strategy(
        erc20_strategy_address,
        deposit_tokens,
        el_chain_writer.clone(),
    )
    .await?;

    set_allocation_delay(allocation_delay, signer.clone(), el_chain_writer.clone()).await?;

    modify_allocation_for_operator(
        operator_set_id,
        avs,
        vec![erc20_strategy_address],
        new_magnitude,
        el_chain_writer.clone(),
        signer.address(),
    )
    .await?;

    register_for_operator_sets(
        operator_set_id,
        bls_key_pair,
        avs,
        socket,
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
