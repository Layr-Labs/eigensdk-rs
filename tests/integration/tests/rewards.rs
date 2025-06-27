//! Integration test for rewards utilities

use alloy::primitives::{aliases::U96, U256};
use eigensdk::common::{get_provider, get_signer};
use eigensdk::testing_utils::{
    anvil::start_anvil_container,
    anvil_constants::{
        get_erc20_mock_strategy, get_rewards_coordinator_address, FIRST_PRIVATE_KEY, SECOND_ADDRESS,
    },
    chain_clients::{
        build_avs_registry_chain_writer, build_el_chain_reader, new_claim, new_test_writer,
    },
    transaction::wait_transaction,
};
use eigensdk::utils::slashing::{
    core::irewardscoordinator::IRewardsCoordinator,
    middleware::servicemanagerbase::IRewardsCoordinatorTypes::{
        RewardsSubmission, StrategyAndMultiplier,
    },
    sdk::mockerc20::MockERC20,
};

#[tokio::test]
async fn test_process_claim() {
    let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
    let rewards_initiator_pk = FIRST_PRIVATE_KEY;
    let signer = get_signer(rewards_initiator_pk, &http_endpoint);

    let el_chain_writer =
        new_test_writer(http_endpoint.to_string(), rewards_initiator_pk.to_string()).await;
    let el_chain_reader = build_el_chain_reader(http_endpoint.to_string()).await;

    let avs_writer =
        build_avs_registry_chain_writer(http_endpoint.clone(), rewards_initiator_pk.to_string())
            .await;

    let rewards_coordinator_address = get_rewards_coordinator_address(http_endpoint.clone()).await;
    let provider = get_provider(&http_endpoint);
    let rewards_coordinator = IRewardsCoordinator::new(rewards_coordinator_address, &provider);

    let mock_strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
    let claimer_address = SECOND_ADDRESS;

    let (_, token_address) = el_chain_reader
        .get_strategy_and_underlying_token(mock_strategy)
        .await
        .unwrap();

    let rewards_duration = rewards_coordinator
        .MAX_REWARDS_DURATION()
        .call()
        .await
        .unwrap()
        ._0;

    let calculation_interval_seconds = rewards_coordinator
        .CALCULATION_INTERVAL_SECONDS()
        .call()
        .await
        .unwrap()
        ._0;

    let current_timestamp: u32 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .try_into()
        .unwrap();
    let intervals_since_genesis = current_timestamp / calculation_interval_seconds;
    let start_timestamp = (intervals_since_genesis + 1) * calculation_interval_seconds;

    let strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
    let (_, token) = el_chain_reader
        .get_strategy_and_underlying_token(strategy_address)
        .await
        .unwrap();

    let strategies_and_multipliers = vec![StrategyAndMultiplier {
        strategy: strategy_address,
        multiplier: U96::from(1),
    }];
    let rewards_submissions = vec![RewardsSubmission {
        strategiesAndMultipliers: strategies_and_multipliers,
        token,
        amount: U256::from(1_000),
        startTimestamp: start_timestamp,
        duration: rewards_duration,
    }];

    let tx_hash = avs_writer
        .create_avs_rewards_submission(rewards_submissions)
        .await
        .unwrap();

    let tx_status = wait_transaction(&http_endpoint, tx_hash)
        .await
        .unwrap()
        .status();

    assert!(tx_status);

    // Check claimer balance at strategy before claim
    let token = MockERC20::new(token_address, &signer);
    let initial_balance = token.balanceOf(claimer_address).call().await.unwrap()._0;

    let rewards_amount = U256::from(42);
    let (_root, claim) = new_claim(&http_endpoint, rewards_amount).await;

    let tx_hash = el_chain_writer
        .process_claim(claim, claimer_address)
        .await
        .unwrap();

    let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
    assert!(receipt.status());

    // Check balance at strategy after claim
    let balance_after_claim = token.balanceOf(claimer_address).call().await.unwrap()._0;

    assert!(balance_after_claim == initial_balance + rewards_amount);
}
