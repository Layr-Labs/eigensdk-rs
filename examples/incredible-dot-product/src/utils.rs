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
use eigensdk::utils::slashing::middleware::registrycoordinator::ISlashingRegistryCoordinatorTypes::OperatorSetParam;
use eigensdk::utils::slashing::middleware::stakeregistry::IStakeRegistryTypes::StrategyParams;
use eigensdk::{
    client_elcontracts::{reader::ELChainReader, writer::ELChainWriter},
    logging::get_logger,
    types::operator::Operator,
};
use eyre::Result;

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
    .await
    .unwrap();

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

/// Registers operator on eigenlayer and avs
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

/// set allocation delay
async fn set_allocation_delay(
    allocation_delay: u32,
    signer: LocalSigner<SigningKey>,
    el_chain_writer: ELChainWriter,
) -> eyre::Result<FixedBytes<32>> {
    Ok(el_chain_writer
        .set_allocation_delay(signer.address(), allocation_delay)
        .await?)
}

/// Creates Total Delegated stake
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

#[allow(clippy::too_many_arguments)]
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
