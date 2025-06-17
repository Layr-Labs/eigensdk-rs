//! This module contains the logic for registering an operator with EigenLayer and specified AVS configurations.
//! It is the main entry point for operator registration. And this is used by the `operator` crate.
//!
//! The goal is to provide a declarative registration system where users specify the desired end state:
//! "operator registered in X and Y AVSs, with x, y, and z stake in strategies a, b, and c",
//! and the SDK automatically handles all the necessary steps to reach that state.
//!
//! NOTE:
//! The logic below mirrors that in the `eigen-client-elcontracts` crate but uses the V2 signer
//! instead of V1 due to incompatibility issues. So we need to use the bindings.

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
use eigen_utils::slashing::core::allocationmanager::AllocationManager;
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
    AvsRegistrationConfig, DepositInfo, OperatorGlobalConfig, OperatorRegistrationConfig,
    OperatorSet,
};

/// Sets up and registers an operator with EigenLayer and specified AVS configurations.
///
/// This is the main entry point for operator registration. It performs a complete setup process:
///
/// - EigenLayer Registration: Registers the operator globally with EigenLayer
///
/// Then, for each operator set, it performs the following steps:
/// - Token Deposits: Deposits ERC20 tokens into specified strategies
/// - Stake Allocation: Configures allocation magnitudes for strategies
/// - AVS Registration: Registers for specified operator sets in each AVS
///
/// # Arguments
///
/// * `config` - Operator registration configuration
/// * `http_rpc_url` - HTTP RPC endpoint
/// * `bls_key_pair` - BLS key pair
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
pub async fn setup_operator(
    config: OperatorRegistrationConfig,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    let OperatorRegistrationConfig {
        signer,
        operator_global_config,
        avs_registration_configs,
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
        operator_global_config.allocation_delay,
        operator_global_config.metadata_uri.clone(),
        operator_global_config.delegation_manager_address,
    )
    .await?;

    // 2. Register operator to each AVS individually
    for avs_config in avs_registration_configs {
        register_operator_to_avs(
            provider.clone(),
            operator_address,
            avs_config,
            operator_global_config.clone(),
            bls_key_pair.clone(),
        )
        .await?;
    }

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
    operator_global_config: OperatorGlobalConfig,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    // 1. Ensure required token deposits exist in strategies. If the operator has already deposited
    // the required amount, skip the deposit. If not, deposit the difference.
    handle_deposit_tokens_amounts(
        provider.clone(),
        operator_address,
        avs_config.deposits.clone(),
        operator_global_config.strategy_manager_address,
        operator_global_config.delegation_manager_address,
    )
    .await?;

    // 2. Configure stake allocation across operator sets. If the operator has already allocated
    // the required amount, skip the allocation. If not, allocate the difference.
    handle_allocation_of_stake_in_strategies(
        provider.clone(),
        operator_address,
        avs_config.deposits.clone(),
        avs_config.allocation_manager_address,
        avs_config.operator_sets.clone(),
    )
    .await?;

    // 3. Register for operator sets. Check if the operator is already registered for the operator sets.
    // If not, register the operator for the operator sets.
    handle_registration_for_operator_sets(
        provider.clone(),
        operator_address,
        avs_config.operator_sets,
        avs_config.allocation_manager_address,
        avs_config.registry_coordinator_address,
        avs_config.socket,
        bls_key_pair,
    )
    .await?;

    Ok(())
}

/// Handles the global EigenLayer operator registration process.
///
/// This function checks if the operator is already registered with EigenLayer and performs
/// registration if needed. Registration requires three components:
/// - Allocation delay (time delay for allocation changes)
/// - Metadata URI (off-chain operator information)
/// - Delegation manager address (contract managing delegations)
///
/// If any required parameter is missing, the registration is skipped with a warning.
/// This allows for flexible configuration where EigenLayer registration is optional.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator to register
/// * `allocation_delay` - Optional time delay for allocation changes (in seconds)
/// * `metadata_uri` - Optional URI pointing to operator metadata
/// * `delegation_manager_address` - Optional address of the delegation manager contract
///
/// # Errors
///
/// Returns error if registration is attempted but fails due to network or contract issues
async fn handle_eigenlayer_registration(
    provider: SdkSigner,
    operator_address: Address,
    allocation_delay: Option<u32>,
    metadata_uri: Option<String>,
    delegation_manager_address: Option<Address>,
) -> Result<(), OperatorRegistrationError> {
    // Only proceed if all required parameters are provided
    // This design allows partial configuration where EigenLayer registration is optional
    if let (Some(allocation_delay), Some(metadata_uri), Some(delegation_manager_address)) =
        (allocation_delay, metadata_uri, delegation_manager_address)
    {
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
    } else {
        warn!("Skipping registration to EigenLayer since necessary parameters are not set");
    };

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
///
/// # Errors
///
/// Returns error if the contract call fails
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
/// # Errors
///
/// Returns error if the registration transaction fails or is reverted
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
    if let (Some(strategy_manager_address), Some(delegation_manager_address)) =
        (strategy_manager_address, delegation_manager_address)
    {
        for deposit in deposits.clone() {
            let amount = U256::from_str(&deposit.amount)
                .map_err(|_| OperatorRegistrationError::U256ParseError)?;

            let deposit_amount = get_deposit_amount_in_strategy(
                provider.clone(),
                operator_address,
                delegation_manager_address,
                deposit.token_address,
            )
            .await?;

            info!(
                "Operator has deposited {deposit_amount} tokens into strategy {:#x}",
                deposit.token_address,
            );

            if deposit_amount < amount {
                let amount_to_deposit = amount - deposit_amount;
                info!(
                    "Expected deposit amount: {amount}. Difference between expected and deposited amount: {amount_to_deposit}"
                );
                info!(
                    "Depositing {amount_to_deposit} tokens into strategy {:#x}",
                    deposit.token_address
                );

                deposit_erc20_into_strategy(
                    provider.clone(),
                    amount_to_deposit,
                    deposit.token_address,
                    strategy_manager_address,
                )
                .await?;
            } else {
                info!(
                    "Operator has deposited the correct amount of tokens into the strategy {:#x}",
                    deposit.token_address
                );
            }
        }
    } else {
        warn!(
            "Skipping deposit of tokens into the strategy since necessary parameters are not set"
        );
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
/// * `strategy_address` - Address of the strategy contract
///
/// # Returns
///
/// * Result<U256, OperatorRegistrationError> - The amount of shares the operator has in the strategy
async fn get_deposit_amount_in_strategy(
    provider: SdkSigner,
    operator_address: Address,
    delegation_manager: Address,
    strategy_address: Address,
) -> Result<U256, OperatorRegistrationError> {
    let contract_delegation_manager = DelegationManager::new(delegation_manager, provider);

    Ok(contract_delegation_manager
        .operatorShares(operator_address, strategy_address)
        .call()
        .await?
        .shares)
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

// TODO: These functions are commented out but kept for potential future use
// They handle allocation delay management which may be needed in certain scenarios

// /// Gets the current allocation delay for an operator.
// ///
// /// The allocation delay is the time period that must pass before allocation changes take effect.
// /// This is a security mechanism to prevent rapid allocation changes.
// async fn get_allocation_delay(
//     provider: SdkSigner,
//     operator_address: Address,
//     allocation_manager_address: Address,
// ) -> Result<u32, OperatorRegistrationError> {
//     let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

//     let delay = contract_allocation_manager
//         .getAllocationDelay(operator_address)
//         .call()
//         .await?
//         ._1;

//     Ok(delay)
// }

// /// Sets the allocation delay for an operator.
// ///
// /// This function allows operators to configure how long they want allocation changes to take effect.
// /// Longer delays provide more security but less flexibility.
// async fn set_allocation_delay(
//     provider: SdkSigner,
//     operator_address: Address,
//     delay: u32,
//     allocation_manager_address: Address,
// ) -> Result<(), OperatorRegistrationError> {
//     let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);
//     contract_allocation_manager
//         .setAllocationDelay(operator_address, delay)
//         .send()
//         .await?
//         .get_receipt()
//         .await?;

//     Ok(())
// }

// TODO: Determine which function is more appropriate - getAllocatedStake vs getAllocation
// Both functions serve similar purposes but may have different use cases

// /// Gets the allocated stake amounts for operators across multiple strategies and operator sets.
// ///
// /// This function provides a batch query mechanism to efficiently retrieve allocation information
// /// for multiple operators and strategies at once.
// async fn get_allocated_stake(
//     provider: SdkSigner,
//     operator_set: OperatorSet,
//     operators: Vec<Address>,
//     strategies: Vec<Address>,
//     allocation_manager_address: Address,
// ) -> Result<Vec<Vec<U256>>, OperatorRegistrationError> {
//     let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);
//     let allocated_stake = contract_allocation_manager
//         .getAllocatedStake(operator_set, operators, strategies)
//         .call()
//         .await?
//         ._0;

//     Ok(allocated_stake)
// }

/// Handles the allocation of stake across strategies for specified operator sets.
///
/// This function ensures that the operator has properly allocated stake to each strategy
/// for each operator set they want to participate in. If the operator has already allocated
/// the required amount, skip the allocation. If not, allocate the difference.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `deposits` - Deposit information containing desired allocation magnitudes
/// * `allocation_manager_address` - Optional address of the allocation manager contract
/// * `operator_sets` - Vector of operator sets to allocate stake for
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn handle_allocation_of_stake_in_strategies(
    provider: SdkSigner,
    operator_address: Address,
    deposits: Vec<DepositInfo>,
    allocation_manager_address: Option<Address>,
    operator_sets: Vec<OperatorSet>,
) -> Result<(), OperatorRegistrationError> {
    if let Some(allocation_manager_address) = allocation_manager_address {
        let mut allocate_params = Vec::new();

        // Build a list of allocation changes needed across all operator sets and strategies
        for operator_set in operator_sets.clone() {
            for deposit in deposits.clone() {
                let current_allocation = get_current_allocation(
                    provider.clone(),
                    operator_address,
                    &operator_set,
                    deposit.token_address,
                    allocation_manager_address,
                )
                .await?;

                // TODO: Should we check current_allocation.pendingDiff???
                if current_allocation.currentMagnitude != deposit.allocation_magnitude {
                    info!(
                        "Current allocation: {}, desired: {} for strategy {:#x}",
                        current_allocation.currentMagnitude,
                        deposit.allocation_magnitude,
                        deposit.token_address
                    );

                    // Prepare allocation parameters for batch transaction
                    allocate_params.push(AllocateParams {
                        operatorSet: operator_set.clone().into(),
                        strategies: vec![deposit.token_address],
                        newMagnitudes: vec![deposit.allocation_magnitude],
                    });
                } else {
                    info!(
                        "Allocation already correct ({}) for strategy {:#x}",
                        current_allocation.currentMagnitude, deposit.token_address
                    );
                }
            }
        }

        // Execute batch allocation changes if any are needed
        if !allocate_params.is_empty() {
            info!("Modifying {} allocations", allocate_params.len());
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
    } else {
        warn!("Skipping allocation of stake - necessary parameters are not set");
    }

    Ok(())
}

/// Retrieves the current allocation information for a specific operator set and strategy.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `operator_set` - The operator set to query
/// * `strategy_address` - Address of the strategy contract
/// * `allocation_manager_address` - Address of the allocation manager contract
///
/// # Returns
///
/// * Result<Allocation, OperatorRegistrationError> - The current allocation details
async fn get_current_allocation(
    provider: SdkSigner,
    operator_address: Address,
    operator_set: &OperatorSet,
    strategy_address: Address,
    allocation_manager_address: Address,
) -> Result<IAllocationManagerTypes::Allocation, OperatorRegistrationError> {
    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

    Ok(contract_allocation_manager
        .getAllocation(
            operator_address,
            operator_set.clone().into(),
            strategy_address,
        )
        .call()
        .await?
        ._0)
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

/// Handles registration for operator sets across potentially multiple AVSs.
///
/// This function manages the operator set registration process.
///
/// # Arguments
///
/// * `provider` - Blockchain provider for contract interactions
/// * `operator_address` - Address of the operator
/// * `operator_sets` - Vector of operator sets to register for
/// * `allocation_manager_address` - Optional address of the allocation manager contract
/// * `registry_coordinator_address` - Optional address of the registry coordinator contract
/// * `socket` - Optional socket address for operator communication
/// * `bls_key_pair` - BLS key pair for generating registration signatures
///
/// # Returns
///
/// * Result<(), OperatorRegistrationError> - The result of the operation
async fn handle_registration_for_operator_sets(
    provider: SdkSigner,
    operator_address: Address,
    operator_sets: Vec<OperatorSet>,
    allocation_manager_address: Option<Address>,
    registry_coordinator_address: Option<Address>,
    socket: Option<String>,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    if let (Some(socket), Some(allocation_manager_address), Some(registry_coordinator_address)) = (
        socket,
        allocation_manager_address,
        registry_coordinator_address,
    ) {
        info!(
            "Checking registration for {} operator sets",
            operator_sets.len()
        );

        // Group operator sets by AVS address for efficient batch processing
        // Each AVS requires a separate registration transaction
        let mut operator_sets_by_avs = HashMap::new();

        for operator_set in operator_sets {
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
                    operator_set.avs_address, operator_set.id
                );
                // Group by AVS address for batch registration
                operator_sets_by_avs
                    .entry(operator_set.avs_address)
                    .or_insert_with(Vec::new)
                    .push(operator_set);
            } else {
                info!(
                    "Operator set {:#x}/{} already registered",
                    operator_set.avs_address, operator_set.id
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
    } else {
        warn!("Skipping registration of operator sets - necessary parameters are not set");
    }

    Ok(())
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
        registered_set.id == operator_set.id && registered_set.avs == operator_set.avs_address
    });

    Ok(is_registered)
}
