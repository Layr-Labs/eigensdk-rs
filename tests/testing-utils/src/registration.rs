use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::{LocalSigner, PrivateKeySigner};
use alloy::{
    hex,
    primitives::{aliases::U96, Address, FixedBytes, U256},
};
use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
use eigen_client_elcontracts::error::ElContractsError;
use eigen_client_elcontracts::reader::ELChainReader;
use eigen_client_elcontracts::writer::ELChainWriter;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::logger::SharedLogger;
use eigen_operator::error::OperatorError;
use eigen_operator::register_config::OperatorRegistrationConfig;
use eigen_types::operator::Operator;
use eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet;
use eigen_utils::slashing::core::allocationmanager::IAllocationManagerTypes::AllocateParams;
use eigen_utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigen_utils::slashing::middleware::stakeregistry::IStakeRegistryTypes::StrategyParams;
use std::str::FromStr;

/// Creates a total delegated stake quorum and registers the operator
/// This function has the same functionality as the `register_test_operator` function
/// but it also creates a total delegated stake quorum for the operator
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
pub async fn create_quorum_and_register_operator(
    config: OperatorRegistrationConfig,
    logger: SharedLogger,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorError> {
    let signer: LocalSigner<SigningKey> =
        if let Some(operator_key) = config.operator_pvt_key.clone() {
            PrivateKeySigner::from_str(&operator_key).unwrap()
        } else {
            LocalSigner::decrypt_keystore(
                config.ecdsa_keystore_path.clone(),
                config.ecdsa_keystore_password.clone(),
            )
            .unwrap()
        };

    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        logger.clone(),
        http_rpc_url.to_string(),
        hex::encode(signer.to_field_bytes()).to_string(),
        config.registry_coordinator_address,
        config.avs_address,
    )
    .await?;

    create_total_delegated_stake_quorum(config.erc20_strategy_address, avs_registry_writer).await?;

    register_test_operator(config, logger, http_rpc_url, bls_key_pair).await?;

    Ok(())
}

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
pub async fn register_test_operator(
    config: OperatorRegistrationConfig,
    logger: SharedLogger,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorError> {
    let signer: LocalSigner<SigningKey> = if let Some(operator_key) = config.operator_pvt_key {
        PrivateKeySigner::from_str(&operator_key).unwrap()
    } else {
        LocalSigner::decrypt_keystore(config.ecdsa_keystore_path, config.ecdsa_keystore_password)
            .unwrap()
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
    .await
    .unwrap();

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

    logger.info("Operator registered successfully", "eigen-testing-utils");
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
/// * `Result<()>` - The result of the operation
async fn register_operator_with_el(
    metadata_uri: String,
    allocation_delay: u32,
    signer: LocalSigner<SigningKey>,
    el_chain_reader: ELChainReader,
    el_chain_writer: ELChainWriter,
) -> Result<(), OperatorError> {
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
        .await
        .unwrap();
    if !is_already_registered {
        let _ = el_chain_writer
            .register_as_operator(operator_details)
            .await
            .unwrap();
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
/// * `Result<FixedBytes<32>>` - The result of the operation
async fn set_allocation_delay(
    allocation_delay: u32,
    signer: LocalSigner<SigningKey>,
    el_chain_writer: ELChainWriter,
) -> Result<FixedBytes<32>, OperatorError> {
    Ok(el_chain_writer
        .set_allocation_delay(signer.address(), allocation_delay)
        .await
        .unwrap())
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
/// * `Result<FixedBytes<32>>` - The result of the operation
async fn create_total_delegated_stake_quorum(
    strategy_address: Address,
    avs_registry_writer: AvsRegistryChainWriter,
) -> Result<FixedBytes<32>, OperatorError> {
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
/// * `Result<FixedBytes<32>, OperatorError>` - The result of the operation
async fn register_for_operator_sets(
    operator_set_id: u32,
    bls_key_pair: BlsKeyPair,
    avs: Address,
    socket: String,
    signer: LocalSigner<SigningKey>,
    el_chain_writer: ELChainWriter,
) -> Result<FixedBytes<32>, OperatorError> {
    Ok(el_chain_writer
        .register_for_operator_sets(
            signer.address(),
            avs,
            [operator_set_id].to_vec(),
            bls_key_pair,
            &socket,
        )
        .await
        .unwrap())
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
        .await
        .unwrap();
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
/// * `Result<FixedBytes<32>, OperatorError>` - The result of the operation
pub async fn modify_allocation_for_operator(
    operator_set_id: u32,
    avs: Address,
    strategies: Vec<Address>,
    new_magnitude: Vec<u64>,
    el_writer: ELChainWriter,
    operator_address: Address,
) -> Result<FixedBytes<32>, OperatorError> {
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
        .await
        .unwrap())
}
