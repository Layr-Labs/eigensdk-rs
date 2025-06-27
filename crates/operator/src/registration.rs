//! This module defines the logic for registering an operator with EigenLayer and specifies the
//! desired state of the operator across various AVSs. It serves as the main entry point for
//! operator registration and is used by the [`operator`](crate::Operator) when it is initialized.
//!
//! The goal is to provide a declarative registration system: users declare the desired end state
//! for example, "operator registered in AVSs X and Y, with stakes x, y, and z in strategies A, B, and C"
//! and the SDK automatically executes all necessary steps to reach that state.
//!
//! NOTE: We are using the V2 signer instead of V1 here. Since we are using the V2 signer, we need to
//! use the bindings since the V2 signer is not compatible with `eigen-client-elcontracts`.

use alloy::dyn_abi::DynSolValue;
use alloy::network::{EthereumWallet, TxSigner};
use alloy::primitives::Address;
use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
use eigen_common::SdkSigner;
use eigen_crypto_bls::{
    alloy_g1_point_to_g1_affine, convert_to_g1_point, convert_to_g2_point, BlsKeyPair,
};
use eigen_signer::tx_signer_from_config;
use eigen_utils::slashing::core::allocationmanager::AllocationManager::{self, OperatorSet};
use eigen_utils::slashing::core::allocationmanager::IAllocationManagerTypes::{
    self, AllocateParams,
};
use eigen_utils::slashing::core::delegationmanager::DelegationManager;
use eigen_utils::slashing::core::istrategy::IStrategy;
use eigen_utils::slashing::core::strategymanager::StrategyManager;
use eigen_utils::slashing::middleware::ierc20::IERC20;
use eigen_utils::slashing::middleware::registrycoordinator::RegistryCoordinator;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{info, warn};
use url::Url;

use crate::error::OperatorRegistrationError;
use crate::register_config::{
    AvsRegistrationConfig, DepositInfo, OperatorELConfig, OperatorRegistrationConfig,
    OperatorSetConfig,
};

/// Performs full setup and registration of an operator with EigenLayer.
///
/// * Registers the operator globally with EigenLayer
///
/// Then, for each operator set, it performs the following steps:
///
/// * Deposits ERC20 tokens into specified strategies
/// * Configures allocation magnitudes for strategies
/// * Registers for specified operator sets in each AVS
///
/// Each step is skipped if its target state is already satisfied.
///
/// # Arguments
///
/// * `config` - Operator registration configuration
/// * `http_rpc_url` - HTTP RPC endpoint
/// * `bls_key_pair` - BLS key pair
///
/// # Returns
///
/// * `Result<(), OperatorRegistrationError>` - The result of the operation
pub async fn setup_operator(
    config: OperatorRegistrationConfig,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    let OperatorRegistrationConfig {
        signer,
        operator_global_config,
        avs_registration_config,
    } = config;

    let signer = tx_signer_from_config(signer).await?;
    let operator_address = signer.address();

    let wallet = EthereumWallet::from(signer);
    let url =
        Url::parse(&http_rpc_url).map_err(|_| OperatorRegistrationError::HttpUrlParseError)?;
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    // 1. Operator registration in EigenLayer
    handle_eigenlayer_registration(
        provider.clone(),
        operator_address,
        operator_global_config.clone(),
    )
    .await?;

    // 2. Register operator to each AVS individually

    register_operator_to_avs(
        provider.clone(),
        operator_address,
        avs_registration_config,
        operator_global_config.clone(),
        bls_key_pair.clone(),
    )
    .await?;

    Ok(())
}

/// Registers the operator to a single AVS by handling deposits, allocations, and registration.
///
/// This function orchestrates the three main phases of AVS setup:
/// 1. Ensures required token deposits exist in strategies
/// 2. Configures proper stake allocation across operator sets
/// 3. Registers the operator for the specified operator sets
///
/// # Arguments
///
/// * `provider` - Blockchain provider
/// * `operator_address` - Operator address
/// * `avs_config` - Specific configuration for this AVS
/// * `operator_global_config` - Specific configuration for registration
/// * `bls_key_pair` - BLS key pair
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn register_operator_to_avs(
    provider: SdkSigner,
    operator_address: Address,
    avs_config: AvsRegistrationConfig,
    operator_global_config: OperatorELConfig,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    // 1. Ensure required token deposits exist in strategies. If the operator has already deposited
    // the required amount, skip the deposit. If not, deposit the difference.

    // Get all deposits for all operator sets
    let deposits = avs_config
        .operator_set_configs
        .iter()
        .flat_map(|config| config.deposits.clone())
        .collect();

    handle_deposit_tokens_amounts(
        provider.clone(),
        operator_address,
        deposits,
        avs_config.strategy_manager_address,
        operator_global_config.delegation_manager_address,
    )
    .await?;

    // 2. Configure stake allocation across operator sets. If the operator has already allocated
    // the required amount, skip the allocation. If not, allocate the difference.

    handle_allocation_of_stake_in_strategies(
        provider.clone(),
        operator_address,
        avs_config.allocation_manager_address,
        avs_config.operator_set_configs.clone(),
        avs_config.avs_address,
    )
    .await?;

    // 3. Set the allocation delay for the operator. If the operator has already set the allocation delay,
    // skip the allocation delay. If not, set the allocation delay.
    handle_allocation_delay(
        provider.clone(),
        operator_address,
        avs_config.allocation_manager_address,
        operator_global_config.allocation_delay,
    )
    .await?;

    // 4. Register for operator sets. Check if the operator is already registered for the operator sets.
    // If not, register the operator for the operator sets.
    handle_registration_for_operator_sets(
        provider.clone(),
        operator_address,
        avs_config.avs_address,
        avs_config.operator_set_configs,
        avs_config.allocation_manager_address,
        avs_config.registry_coordinator_address,
        avs_config.socket,
        bls_key_pair,
    )
    .await?;

    Ok(())
}

/// Handles operator registration process to EigenLayer.
///
/// This function checks if the operator is already registered with EigenLayer and performs
/// registration if needed.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator to register
/// * `operator_el_config` - Operator configuration for EigenLayer registration
///
/// # Returns
///
/// * `Result<(), OperatorRegistrationError>` - The result of the operation
async fn handle_eigenlayer_registration(
    provider: SdkSigner,
    operator_address: Address,
    operator_el_config: OperatorELConfig,
) -> Result<(), OperatorRegistrationError> {
    let (Some(delegation_manager_address), Some(allocation_delay), Some(metadata_uri)) = (
        operator_el_config.delegation_manager_address,
        operator_el_config.allocation_delay,
        operator_el_config.metadata_uri,
    ) else {
        warn!("Skipping registration to EigenLayer since necessary parameters are not set");
        return Ok(());
    };

    info!("Checking if operator {operator_address:#x} is already registered in EigenLayer");

    // Check current registration status to avoid unnecessary transactions
    let is_operator_registered = is_operator_registered_in_eigenlayer(
        provider.clone(),
        operator_address,
        delegation_manager_address,
    )
    .await?;

    if !is_operator_registered {
        info!("Operator is not registered in EigenLayer");
        register_operator_to_eigenlayer(
            provider.clone(),
            operator_address,
            allocation_delay,
            metadata_uri,
            delegation_manager_address,
        )
        .await?;
        info!("Operator {operator_address:#x} registered in EigenLayer");
    } else {
        info!("Operator {operator_address:#x} is already registered in EigenLayer");
    }

    Ok(())
}

/// Checks if an operator is already registered in EigenLayer.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator to check
/// * `delegation_manager_address` - Address of the delegation manager contract
///
/// # Returns
///
/// * `true` if the operator is registered, `false` otherwise
async fn is_operator_registered_in_eigenlayer(
    provider: SdkSigner,
    operator_address: Address,
    delegation_manager_address: Address,
) -> Result<bool, OperatorRegistrationError> {
    let contract_delegation_manager = DelegationManager::new(delegation_manager_address, provider);

    // Query the delegation manager to check operator status
    let is_operator = contract_delegation_manager
        .isOperator(operator_address)
        .call()
        .await?
        ._0;

    Ok(is_operator)
}

/// Registers an operator with EigenLayer by calling the delegation manager contract.
///
/// This function performs the actual on-chain registration transaction with the specified
/// allocation delay and metadata URI.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator to register
/// * `allocation_delay` - Time delay for allocation changes (in seconds)
/// * `metadata_url` - URI pointing to operator metadata (typically IPFS or HTTP)
/// * `delegation_manager_address` - Address of the delegation manager contract
///
/// # Returns
///
/// * `Result<(), OperatorRegistrationError>` - The result of the operation
async fn register_operator_to_eigenlayer(
    provider: SdkSigner,
    operator_address: Address,
    allocation_delay: u32,
    metadata_url: String,
    delegation_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    let contract_delegation_manager = DelegationManager::new(delegation_manager_address, provider);

    // Submit registration transaction with sufficient gas limit
    // Gas limit is set conservatively to handle various contract conditions
    contract_delegation_manager
        .registerAsOperator(operator_address, allocation_delay, metadata_url)
        .gas(300000)
        .send()
        .await?
        .get_receipt()
        .await?;
    Ok(())
}

/// Handles token deposits into EigenLayer strategies.
///
/// This function ensures that the operator has deposited the required amounts of tokens
/// into each specified strategy. If the operator has already deposited the required amount,
/// skip the deposit. If not, deposit the difference between the required amount and the deposited amount.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator making deposits
/// * `deposits` - Vector of deposit configurations specifying amounts and strategies
/// * `strategy_manager_address` - Optional address of the strategy manager contract
/// * `delegation_manager_address` - Optional address of the delegation manager contract
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn handle_deposit_tokens_amounts(
    provider: SdkSigner,
    operator_address: Address,
    deposits: Vec<DepositInfo>,
    strategy_manager_address: Option<Address>,
    delegation_manager_address: Option<Address>,
) -> Result<(), OperatorRegistrationError> {
    let (Some(strategy_manager_address), Some(delegation_manager_address)) =
        (strategy_manager_address, delegation_manager_address)
    else {
        warn!(
            "Skipping deposit of tokens into the strategy since necessary parameters are not set"
        );
        return Ok(());
    };

    // Get all strategy addresses for all deposits
    let strategy_addresses = deposits
        .iter()
        .map(|deposit| deposit.strategy_address)
        .collect();

    // Get the current deposit amount for all strategies
    let deposit_amounts = get_deposit_amount_in_strategy(
        provider.clone(),
        operator_address,
        delegation_manager_address,
        strategy_addresses,
    )
    .await?;

    for (deposit, deposit_amount) in deposits.iter().zip(deposit_amounts.iter()) {
        // Amount declared by the user
        let amount = U256::from_str(&deposit.amount)
            .map_err(|_| OperatorRegistrationError::U256ParseError)?;

        // If the operator has not deposited the required amount, deposit the difference
        // If not, skip the deposit
        if deposit_amount < &amount {
            let amount_to_deposit = amount - deposit_amount;
            info!(
                "Expected deposit amount: {amount}. Difference between expected and deposited amount: {amount_to_deposit}"
            );
            info!(
                "Depositing {amount_to_deposit} tokens into strategy {:#x}",
                deposit.strategy_address
            );

            deposit_erc20_into_strategy(
                provider.clone(),
                amount_to_deposit,
                deposit.strategy_address,
                strategy_manager_address,
            )
            .await?;
        } else {
            info!(
                "Operator has deposited the correct amount of tokens into the strategy {:#x}",
                deposit.strategy_address
            );
        }
    }

    Ok(())
}

/// Retrieves the current deposit amount for an operator in a specific strategy.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `delegation_manager` - Address of the delegation manager contract
/// * `strategy_addresses` - Addresses of the strategy contracts
///
/// # Returns
///
/// * Result<U256, OperatorRegistrationError> - The amount of shares the operator has in the strategy
async fn get_deposit_amount_in_strategy(
    provider: SdkSigner,
    operator_address: Address,
    delegation_manager: Address,
    strategy_addresses: Vec<Address>,
) -> Result<Vec<U256>, OperatorRegistrationError> {
    let contract_delegation_manager = DelegationManager::new(delegation_manager, provider);

    Ok(contract_delegation_manager
        .getOperatorShares(operator_address, strategy_addresses)
        .call()
        .await?
        ._0)
}

/// Deposits ERC20 tokens into a specific strategy through the strategy manager.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `amount` - Amount of tokens to deposit
/// * `strategy_address` - Address of the strategy contract
/// * `strategy_manager_address` - Address of the strategy manager contract
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn deposit_erc20_into_strategy(
    provider: SdkSigner,
    amount: U256,
    strategy_address: Address,
    strategy_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    let contract_strategy = IStrategy::new(strategy_address, provider.clone());
    let token_address = contract_strategy.underlyingToken().call().await?._0;

    let token_contract = IERC20::new(token_address, &provider);
    token_contract
        .approve(strategy_manager_address, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    let contract_strategy_manager = StrategyManager::new(strategy_manager_address, &provider);
    contract_strategy_manager
        .depositIntoStrategy(strategy_address, token_address, amount)
        .send()
        .await?
        .get_receipt()
        .await?;

    Ok(())
}

/// Handles the allocation of stake in strategies for an operator.
///
/// This function ensures that the operator has allocated the required amounts of stake
/// into each specified strategy. If the operator has already allocated the required amount,
/// skip the allocation. If not, allocate the difference between the required amount and the allocated amount.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `allocation_manager_address` - Address of the allocation manager contract
/// * `operator_set_configs` - Configuration for operator sets
/// * `avs_address` - Address of the AVS contract
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn handle_allocation_of_stake_in_strategies(
    provider: SdkSigner,
    operator_address: Address,
    allocation_manager_address: Option<Address>,
    operator_set_configs: Vec<OperatorSetConfig>,
    avs_address: Address,
) -> Result<(), OperatorRegistrationError> {
    let Some(allocation_manager_address) = allocation_manager_address else {
        warn!("Skipping allocation of stake - necessary parameters are not set");
        return Ok(());
    };

    let mut allocate_params = Vec::new();

    // Build a list of allocation changes needed across all operator sets and strategies
    for operator_set_config in operator_set_configs.clone() {
        let operator_set = operator_set_config.operator_set(avs_address);
        let strategy_addresses: Vec<Address> = operator_set_config
            .deposits
            .iter()
            .map(|deposit| deposit.strategy_address)
            .collect();

        // Get the current allocated stake for all strategies
        let current_allocated_stakes = get_current_allocated_stake(
            provider.clone(),
            operator_address,
            &operator_set,
            strategy_addresses.clone(),
            allocation_manager_address,
        )
        .await?;

        // Collect strategies and magnitudes that need updates
        let mut strategies_to_update = Vec::new();
        let mut new_magnitudes = Vec::new();

        for (i, deposit) in operator_set_config.deposits.iter().enumerate() {
            let current_stake = current_allocated_stakes.get(i).unwrap_or(&U256::ZERO);
            let desired_stake = U256::from_str(&deposit.amount)
                .map_err(|_| OperatorRegistrationError::U256ParseError)?;

            if current_stake != &desired_stake {
                info!(
                    "Current allocated stake: {current_stake}, desired: {desired_stake} for strategy {:#x}",
                    deposit.strategy_address
                );

                strategies_to_update.push(deposit.strategy_address);
                new_magnitudes.push(deposit.allocation_magnitude);
            } else {
                info!(
                    "Allocation already correct for strategy {:#x}: {current_stake}",
                    deposit.strategy_address
                );
            }
        }

        // Create AllocateParams if there are strategies to update for this operator set
        if !strategies_to_update.is_empty() {
            allocate_params.push(AllocateParams {
                operatorSet: operator_set,
                strategies: strategies_to_update,
                newMagnitudes: new_magnitudes,
            });
        }
    }

    // Execute batch allocation changes if any are needed
    if !allocate_params.is_empty() {
        info!("Modifying {} allocation batches", allocate_params.len());
        modify_allocations(
            provider,
            operator_address,
            allocate_params,
            allocation_manager_address,
        )
        .await?;
    } else {
        info!("All allocations are already correct");
    }

    Ok(())
}

/// Retrieves the current allocated stake for a specific operator set and strategy.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `operator_set` - The operator set to query
/// * `strategy_addresses` - Addresses of the strategy contracts
/// * `allocation_manager_address` - Address of the allocation manager contract
///
/// # Returns
///
/// * Result<U256, OperatorRegistrationError> - The current allocated stake
async fn get_current_allocated_stake(
    provider: SdkSigner,
    operator_address: Address,
    operator_set: &OperatorSet,
    strategy_addresses: Vec<Address>,
    allocation_manager_address: Address,
) -> Result<Vec<U256>, OperatorRegistrationError> {
    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

    let allocated_stakes = contract_allocation_manager
        .getAllocatedStake(
            operator_set.clone(),
            vec![operator_address],
            strategy_addresses,
        )
        .call()
        .await?
        ._0;

    // Return the stake for the first (and only) operator and strategy
    Ok(allocated_stakes
        .first()
        .unwrap_or(&vec![U256::ZERO])
        .clone())
}

/// Executes batch allocation modifications for an operator.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `allocations` - Vector of allocation parameters to modify
/// * `allocation_manager_address` - Address of the allocation manager contract
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn modify_allocations(
    provider: SdkSigner,
    operator_address: Address,
    allocations: Vec<IAllocationManagerTypes::AllocateParams>,
    allocation_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

    contract_allocation_manager
        .modifyAllocations(operator_address, allocations)
        .send()
        .await?
        .get_receipt()
        .await?;

    Ok(())
}

/// Handles the allocation delay for an operator.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `allocation_manager_address` - Address of the allocation manager contract
/// * `allocation_delay` - Allocation delay to set
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn handle_allocation_delay(
    provider: SdkSigner,
    operator_address: Address,
    allocation_manager_address: Option<Address>,
    allocation_delay: Option<u32>,
) -> Result<(), OperatorRegistrationError> {
    let (Some(allocation_manager_address), Some(allocation_delay)) =
        (allocation_manager_address, allocation_delay)
    else {
        warn!("Skipping allocation delay - necessary parameters are not set");
        return Ok(());
    };

    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

    let allocation_return = contract_allocation_manager
        .getAllocationDelay(operator_address)
        .call()
        .await?;

    let AllocationManager::getAllocationDelayReturn {
        _0: is_allocation_delay_set,
        _1: current_allocation_delay,
    } = allocation_return;

    if current_allocation_delay != allocation_delay || !is_allocation_delay_set {
        info!("Current allocation delay: {current_allocation_delay}, desired: {allocation_delay}");
        contract_allocation_manager
            .setAllocationDelay(operator_address, allocation_delay)
            .send()
            .await?
            .get_receipt()
            .await?;
        info!("Allocation delay set to {allocation_delay}");
    } else {
        info!("Allocation delay is already correct");
    }

    Ok(())
}

/// Handles registration for operator sets across potentially multiple AVSs.
///
/// This function manages the operator set registration process.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `avs_address` - Address of the AVS contract
/// * `operator_set_configs` - Configuration for operator sets
/// * `allocation_manager_address` - Optional address of the allocation manager contract
/// * `registry_coordinator_address` - Optional address of the registry coordinator contract
/// * `socket` - Optional socket address for operator communication
/// * `bls_key_pair` - BLS key pair for generating registration signatures
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
#[allow(clippy::too_many_arguments)]
async fn handle_registration_for_operator_sets(
    provider: SdkSigner,
    operator_address: Address,
    avs_address: Address,
    operator_set_configs: Vec<OperatorSetConfig>,
    allocation_manager_address: Option<Address>,
    registry_coordinator_address: Option<Address>,
    socket: Option<String>,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    let (Some(socket), Some(allocation_manager_address), Some(registry_coordinator_address)) = (
        socket,
        allocation_manager_address,
        registry_coordinator_address,
    ) else {
        warn!("Skipping registration of operator sets - necessary parameters are not set");
        return Ok(());
    };

    info!(
        "Checking registration for {} operator sets",
        operator_set_configs.len()
    );

    // Group operator sets by AVS address for efficient batch processing
    // Each AVS requires a separate registration transaction
    let mut operator_sets_by_avs = HashMap::new();

    for operator_set_config in operator_set_configs.clone() {
        let operator_set = operator_set_config.operator_set(avs_address);
        // Check if operator is already registered for this specific operator set
        let is_registered = is_operator_registered_for_operator_set(
            provider.clone(),
            operator_address,
            &operator_set,
            allocation_manager_address,
        )
        .await?;

        if !is_registered {
            info!(
                "Operator set {:#x}/{} requires registration",
                operator_set.avs, operator_set.id
            );
            // Group by AVS address for batch registration
            operator_sets_by_avs
                .entry(operator_set.avs)
                .or_insert_with(Vec::new)
                .push(operator_set);
        } else {
            info!(
                "Operator set {:#x}/{} already registered",
                operator_set.avs, operator_set.id
            );
        }
    }

    for (avs_address, sets) in operator_sets_by_avs {
        let operator_set_ids: Vec<u32> = sets.iter().map(|s| s.id).collect();

        info!(
            "Registering {} operator sets for AVS {avs_address:#x}",
            operator_set_ids.len(),
        );

        register_for_operator_sets(
            provider.clone(),
            operator_address,
            operator_set_ids,
            bls_key_pair.clone(),
            &socket,
            allocation_manager_address,
            registry_coordinator_address,
            avs_address,
        )
        .await?;
    }

    Ok(())
}

/// Checks if an operator is registered for a specific operator set.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `operator_set` - The operator set to check registration for
/// * `allocation_manager_address` - Address of the allocation manager contract
///
/// # Returns
///
/// * Result<bool, OperatorRegistrationError> - True if the operator is registered for the operator set, false otherwise
async fn is_operator_registered_for_operator_set(
    provider: SdkSigner,
    operator_address: Address,
    operator_set: &OperatorSet,
    allocation_manager_address: Address,
) -> Result<bool, OperatorRegistrationError> {
    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

    // Get all registered operator sets for this operator
    let registered_sets = contract_allocation_manager
        .getRegisteredSets(operator_address)
        .call()
        .await?
        ._0;

    // Check if our target operator set is in the registered list
    // We match both the operator set ID and the AVS address
    let is_registered = registered_sets.iter().any(|registered_set| {
        registered_set.id == operator_set.id && registered_set.avs == operator_set.avs
    });

    Ok(is_registered)
}

/// Registers the operator for a set of operator sets.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `operator_set_ids` - Vector of operator set IDs to register for within this AVS
/// * `bls_key_pair` - BLS key pair for signature generation
/// * `socket` - Socket address for operator communication
/// * `allocation_manager_address` - Address of the allocation manager contract
/// * `registry_coordinator_address` - Address of the registry coordinator contract
/// * `avs_address` - Address of the AVS contract
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
#[allow(clippy::too_many_arguments)]
async fn register_for_operator_sets(
    provider: SdkSigner,
    operator_address: Address,
    operator_set_ids: Vec<u32>,
    bls_key_pair: BlsKeyPair,
    socket: &str,
    allocation_manager_address: Address,
    registry_coordinator_address: Address,
    avs_address: Address,
) -> Result<(), OperatorRegistrationError> {
    let contract_allocation_manager =
        AllocationManager::new(allocation_manager_address, provider.clone());
    let contract_registry_coordinator =
        RegistryCoordinator::new(registry_coordinator_address, provider);

    let g1_hashed_msg_to_sign = contract_registry_coordinator
        .pubkeyRegistrationMessageHash(operator_address)
        .call()
        .await?
        ._0;

    let sig = bls_key_pair
        .sign_hashed_to_curve_message(alloy_g1_point_to_g1_affine(g1_hashed_msg_to_sign))
        .g1_point();

    let alloy_g1_point_signed_msg = convert_to_g1_point(sig.g1())?;
    let g1_pub_key_bn254 = convert_to_g1_point(bls_key_pair.public_key().g1())?;
    let g2_pub_key_bn254 = convert_to_g2_point(bls_key_pair.public_key_g2().g2())?;

    let g2_point_x: Vec<DynSolValue> = vec![
        DynSolValue::Uint(g2_pub_key_bn254.X[0], 256),
        DynSolValue::Uint(g2_pub_key_bn254.X[1], 256),
    ];
    let g2_point_y: Vec<DynSolValue> = vec![
        DynSolValue::Uint(g2_pub_key_bn254.Y[0], 256),
        DynSolValue::Uint(g2_pub_key_bn254.Y[1], 256),
    ];

    let encoded_params_with_socket = DynSolValue::Tuple(vec![
        DynSolValue::Uint(U256::from(0), 256), // Registration type flag
        DynSolValue::String(socket.to_string()), // Operator socket address
        DynSolValue::Uint(alloy_g1_point_signed_msg.X, 256), // Signature X coordinate
        DynSolValue::Uint(alloy_g1_point_signed_msg.Y, 256), // Signature Y coordinate
        DynSolValue::Uint(g1_pub_key_bn254.X, 256), // G1 public key X
        DynSolValue::Uint(g1_pub_key_bn254.Y, 256), // G1 public key Y
        DynSolValue::FixedArray(g2_point_x),   // G2 public key X components
        DynSolValue::FixedArray(g2_point_y),   // G2 public key Y components
    ])
    .abi_encode_params();

    let params = IAllocationManagerTypes::RegisterParams {
        avs: avs_address,
        operatorSetIds: operator_set_ids,
        data: encoded_params_with_socket.into(),
    };

    contract_allocation_manager
        .registerForOperatorSets(operator_address, params)
        .send()
        .await?
        .get_receipt()
        .await?;

    Ok(())
}
