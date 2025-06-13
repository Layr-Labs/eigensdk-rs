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
use eigen_types::avs_state::OperatorSet;
use eigen_utils::slashing::core::allocationmanager::AllocationManager;
use eigen_utils::slashing::core::allocationmanager::IAllocationManagerTypes::{
    self, AllocateParams,
};
use eigen_utils::slashing::core::delegationmanager::DelegationManager;
use eigen_utils::slashing::core::istrategy::IStrategy;
use eigen_utils::slashing::core::strategymanager::StrategyManager;
use eigen_utils::slashing::middleware::ierc20::IERC20;
use eigen_utils::slashing::middleware::registrycoordinator::RegistryCoordinator;
use std::str::FromStr;
use tracing::{info, warn};
use url::Url;

use crate::error::OperatorRegistrationError;
use crate::register_config::{DepositInfo, OperatorRegistrationConfig};

// The idea is for it to be declarative, where the user declares the end state “operator registered
// in X and Y AVSs, with x, y, and z stake in strategies a, b, and c”, and the SDK registers and deposits to reach that state.

/// Registers an operator with EigenLayer.
///
/// 1. Registers the operator with EigenLayer
/// 2. Deposits ERC20 into the strategy
/// 3. Sets the allocation delay - SHOULD DELETE THIS?
/// 4. Modifies the allocation magnitude for the operator in specific strategies
/// 5. Registers the operator for operator sets
///
/// # Arguments
///
/// * `config` - The operator registration config
/// * `http_rpc_url` - The HTTP RPC URL
/// * `bls_key_pair` - The BLS key pair
///
/// # Returns
///
/// * `Result<(), OperatorError>` - The result of the operation
pub async fn setup_operator(
    config: OperatorRegistrationConfig,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    let OperatorRegistrationConfig {
        signer,
        metadata_uri,
        socket,
        allocation_delay,
        allocation_manager_address,
        registry_coordinator_address,
        delegation_manager_address,
        strategy_manager_address,
        operator_sets,
        deposits,
    } = config;

    let signer = tx_signer_from_config(signer).await?;
    let operator_address = signer.address();

    let wallet = EthereumWallet::from(signer);
    let url =
        Url::parse(&http_rpc_url).map_err(|_| OperatorRegistrationError::HttpUrlParseError)?;
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    // Register the operator to EigenLayer
    handle_eigenlayer_registration(
        provider.clone(),
        operator_address,
        allocation_delay,
        metadata_uri,
        delegation_manager_address,
    )
    .await?;

    // Check if the operator has deposited tokens into the strategy and if it matches the amount in the config
    // If not, deposit the tokens into the strategy or the difference between the amount in the config and the amount in the strategy
    handle_deposit_tokens_amounts(
        provider.clone(),
        operator_address,
        deposits.clone(),
        strategy_manager_address,
        delegation_manager_address,
    )
    .await?;

    // Allocate stake in the strategy
    handle_allocation_of_stake_in_strategies(
        provider.clone(),
        operator_address,
        deposits.clone(),
        allocation_manager_address,
        operator_sets.clone(),
    )
    .await?;

    // Register the operator for operator sets
    handle_registration_for_operator_sets(
        provider.clone(),
        operator_address,
        operator_sets.clone(),
        allocation_manager_address,
        registry_coordinator_address,
        socket,
        bls_key_pair,
    )
    .await?;

    Ok(())
}

// The logic for the functions below is the same as the one in the `eigen-client-elcontracts` crate.
// With the difference that we are using the V2 signer instead of the V1 signer.
// There is an incompatibility with `eigen-client-elcontracts`, therefore, we need
// to perform operator registration using the bindings.

/// Registers the operator to EigenLayer. This will check if the operator is already registered
/// and if not, it will register the operator.
///
/// To do this actions, we need the following parameters:
/// - Allocation delay
/// - Metadata URI
/// - Delegation manager address
///
/// If one of the parameters is not set, we skip the registration process.
///
/// # Arguments
///
/// * `provider` - The provider
/// * `operator_address` - The operator address
/// * `allocation_delay` - The allocation delay
/// * `metadata_uri` - The metadata URI
/// * `delegation_manager_address` - The delegation manager address
///
/// # Returns
///
/// * `Result<(), OperatorRegistrationError>` - The result of the operation
async fn handle_eigenlayer_registration(
    provider: SdkSigner,
    operator_address: Address,
    allocation_delay: Option<u32>,
    metadata_uri: Option<String>,
    delegation_manager_address: Option<Address>,
) -> Result<(), OperatorRegistrationError> {
    // Check if operator is already registered in EigenLayer, if so, skip the registration process
    if let (Some(allocation_delay), Some(metadata_uri), Some(delegation_manager_address)) =
        (allocation_delay, metadata_uri, delegation_manager_address)
    {
        info!("Checking if operator {operator_address:#x} is already registered in EigenLayer");

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

/// Checks if the operator is registered in EigenLayer.
///
/// # Arguments
///
/// * `provider` - The provider
/// * `operator_address` - The operator address
/// * `delegation_manager_address` - The delegation manager address
///
/// # Returns
///
/// * `Result<bool, OperatorRegistrationError>` - The result of the operation
async fn is_operator_registered_in_eigenlayer(
    provider: SdkSigner,
    operator_address: Address,
    delegation_manager_address: Address,
) -> Result<bool, OperatorRegistrationError> {
    let contract_delegation_manager = DelegationManager::new(delegation_manager_address, provider);

    let is_operator = contract_delegation_manager
        .isOperator(operator_address)
        .call()
        .await?
        ._0;

    Ok(is_operator)
}

/// Registers the operator to EigenLayer.
///
/// # Arguments
///
/// * `provider` - The provider
/// * `operator_address` - The operator address
/// * `allocation_delay` - The allocation delay
/// * `metadata_url` - The metadata URL
/// * `delegation_manager_address` - The delegation manager address
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
    contract_delegation_manager
        .registerAsOperator(operator_address, allocation_delay, metadata_url)
        .gas(300000)
        .send()
        .await?
        .get_receipt()
        .await?;
    Ok(())
}

/// Handles the deposit of tokens into the strategies. This will check if the operator has deposited
/// the correct amount of tokens into the strategies and if not, it will deposit the difference
/// between the expected amount and the deposited amount.
///
/// To do this actions, we need the following parameters:
/// - Strategy manager address
/// - Delegation manager address
///
/// If one of the parameters is not set, we skip the deposit process.
///
/// # Arguments
///
/// * `provider` - The provider
/// * `operator_address` - The operator address
/// * `deposits` - The deposits
/// * `strategy_manager_address` - The strategy manager address
/// * `delegation_manager_address` - The delegation manager address
///
/// # Returns
///
/// * `Result<(), OperatorRegistrationError>` - The result of the operation
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

// TODO: CHECK IF THIS IS NEEDED
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

// TODO: CHECK IF THIS IS NEEDED
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

// TODO: CHECK WHICH FUNCTION TO USE - getAllocatedStake or getAllocation
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

async fn handle_allocation_of_stake_in_strategies(
    provider: SdkSigner,
    operator_address: Address,
    deposits: Vec<DepositInfo>,
    allocation_manager_address: Option<Address>,
    operator_sets: Vec<OperatorSet>,
) -> Result<(), OperatorRegistrationError> {
    // Allocate stake in the strategy
    if let Some(allocation_manager_address) = allocation_manager_address {
        let mut allocate_params = Vec::new();
        for operator_set in operator_sets.clone() {
            for deposit in deposits.clone() {
                allocate_params.push(AllocateParams {
                    operatorSet: operator_set.clone().into(),
                    strategies: vec![deposit.token_address],
                    newMagnitudes: vec![deposit.allocation_magnitude],
                });
            }
        }

        modify_allocations(
            provider.clone(),
            operator_address,
            allocate_params,
            allocation_manager_address,
        )
        .await?;
    } else {
        warn!(
            "Skipping allocation of stake in the strategy since necessary parameters are not set"
        );
    }

    Ok(())
}

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

async fn is_operator_registered_for_operator_sets(
    provider: SdkSigner,
    operator_set: OperatorSet,
    operator_address: Address,
    allocation_manager_address: Address,
) -> Result<bool, OperatorRegistrationError> {
    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);

    let operator_sets = contract_allocation_manager
        .getRegisteredSets(operator_address)
        .call()
        .await?
        ._0;

    let is_registered = operator_sets.iter().any(|registered_operator_set| {
        registered_operator_set.id == operator_set.id
            && registered_operator_set.avs == operator_set.avs
    });
    Ok(is_registered)
}

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
        DynSolValue::Uint(U256::from(0), 256),
        DynSolValue::String(socket.to_string()),
        DynSolValue::Uint(alloy_g1_point_signed_msg.X, 256),
        DynSolValue::Uint(alloy_g1_point_signed_msg.Y, 256),
        DynSolValue::Uint(g1_pub_key_bn254.X, 256),
        DynSolValue::Uint(g1_pub_key_bn254.Y, 256),
        DynSolValue::FixedArray(g2_point_x),
        DynSolValue::FixedArray(g2_point_y),
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
        info!("Checking if operator is registered for operator sets");

        for operator_set in operator_sets {
            let is_registered = is_operator_registered_for_operator_sets(
                provider.clone(),
                operator_set.clone(),
                operator_address,
                allocation_manager_address,
            )
            .await?;

            if !is_registered {
                info!("Operator is not registered for operator sets");
                info!("Registering operator for operator sets");

                register_for_operator_sets(
                    provider.clone(),
                    operator_address,
                    vec![operator_set.id],
                    bls_key_pair.clone(),
                    &socket,
                    allocation_manager_address,
                    registry_coordinator_address,
                    operator_set.avs,
                )
                .await?;
            } else {
                info!("Operator is registered for operator sets");
            }
        }
    } else {
        warn!("Skipping registration for operator sets since necessary parameters are not set");
    }

    Ok(())
}
