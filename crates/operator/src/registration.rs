use alloy::dyn_abi::DynSolValue;
use alloy::network::{EthereumWallet, TxSigner};
use alloy::primitives::Address;
use alloy::primitives::U256;
use alloy::providers::ProviderBuilder;
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
use std::str::FromStr;
use tracing::info;
use url::Url;

use crate::error::OperatorRegistrationError;
use crate::register_config::OperatorRegistrationConfig;

type ProviderSDK = alloy::providers::fillers::FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy::providers::fillers::WalletFiller<EthereumWallet>,
    >,
    alloy::providers::RootProvider,
>;

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
/// * `http_rpc_url` - The HTTP RPC URL
/// * `bls_key_pair` - The BLS key pair
///
/// # Returns
///
/// * `Result<(), OperatorError>` - The result of the operation
pub async fn register_operator(
    config: OperatorRegistrationConfig,
    http_rpc_url: String,
    bls_key_pair: BlsKeyPair,
) -> Result<(), OperatorRegistrationError> {
    let signer = tx_signer_from_config(config.signer).await?;

    let operator_address = signer.address();

    let wallet = EthereumWallet::from(signer);
    let url =
        Url::parse(&http_rpc_url).map_err(|_| OperatorRegistrationError::HttpUrlParseError)?;
    let provider = ProviderBuilder::new().wallet(wallet.clone()).on_http(url);

    register_operator_to_eigenlayer(
        provider.clone(),
        operator_address,
        config.allocation_delay,
        config.metadata_uri,
        config.delegation_manager_address,
    )
    .await?;

    let amount = U256::from_str(&config.deposit_tokens)
        .map_err(|_| OperatorRegistrationError::U256ParseError)?;

    deposit_erc20_into_strategy(
        provider.clone(),
        amount,
        config.erc20_strategy_address,
        config.strategy_manager_address,
    )
    .await?;

    set_allocation_delay(
        provider.clone(),
        operator_address,
        config.allocation_delay,
        config.allocation_manager_address,
    )
    .await?;

    let allocate_params = vec![AllocateParams {
        operatorSet: OperatorSet {
            avs: config.avs_address,
            id: config.operator_set_id,
        },
        strategies: vec![config.erc20_strategy_address],
        newMagnitudes: config.new_magnitude,
    }];

    modify_allocations(
        provider.clone(),
        operator_address,
        allocate_params,
        config.allocation_manager_address,
    )
    .await?;

    register_for_operator_sets(
        provider.clone(),
        operator_address,
        vec![config.operator_set_id],
        bls_key_pair,
        &config.socket,
        config.allocation_manager_address,
        config.registry_coordinator_address,
        config.avs_address,
    )
    .await?;

    Ok(())
}

// The logic for the functions below is the same as the one in the `eigen-client-elcontracts` crate.
// With the difference that we are using the V2 signer instead of the V1 signer.
async fn register_operator_to_eigenlayer(
    provider: ProviderSDK,
    operator_address: Address,
    allocation_delay: u32,
    metadata_url: String,
    delegation_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    info!("registering operator {operator_address} to EigenLayer");

    let contract_delegation_manager = DelegationManager::new(delegation_manager_address, provider);
    contract_delegation_manager
        .registerAsOperator(operator_address, allocation_delay, metadata_url)
        .gas(300000)
        .send()
        .await?
        .get_receipt()
        .await
        .inspect(|tx| {
            info!(
                "operator registered to EigenLayer {:?}",
                tx.transaction_hash
            )
        })?;

    Ok(())
}

async fn deposit_erc20_into_strategy(
    provider: ProviderSDK,
    amount: U256,
    strategy_address: Address,
    strategy_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    info!("depositing {amount:?} tokens into strategy {strategy_address:?}");

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

async fn set_allocation_delay(
    provider: ProviderSDK,
    operator_address: Address,
    delay: u32,
    allocation_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    info!("setting allocation delay for operator {operator_address} to {delay}");

    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);
    contract_allocation_manager
        .setAllocationDelay(operator_address, delay)
        .send()
        .await?
        .get_receipt()
        .await?;

    Ok(())
}

async fn modify_allocations(
    provider: ProviderSDK,
    operator_address: Address,
    allocations: Vec<IAllocationManagerTypes::AllocateParams>,
    allocation_manager_address: Address,
) -> Result<(), OperatorRegistrationError> {
    info!("modifying allocations for operator {operator_address}");

    let contract_allocation_manager = AllocationManager::new(allocation_manager_address, provider);
    contract_allocation_manager
        .modifyAllocations(operator_address, allocations)
        .send()
        .await?
        .get_receipt()
        .await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn register_for_operator_sets(
    provider: ProviderSDK,
    operator_address: Address,
    operator_set_ids: Vec<u32>,
    bls_key_pair: BlsKeyPair,
    socket: &str,
    allocation_manager_address: Address,
    registry_coordinator_address: Address,
    avs_address: Address,
) -> Result<(), OperatorRegistrationError> {
    info!(
        "registering operator {operator_address} in AVS {avs_address} for operator sets {operator_set_ids:?}"
    );

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
