# Changelog

All notable changes to this project will be documented in this file.

Changelog is present since v0.1.2

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each version will have a separate `Breaking Changes` section as well. To describe in how to upgrade from one version to another if needed.

Those changes in added, changed or breaking changes, should include usage examples to add clarity to the sdk user.

## [Unreleased]

### Security 🔒

### Added 🎉

* Added all features of the `eigensdk` crate to its `"full"` feature [#370](https://github.com/Layr-Labs/eigensdk-rs/pull/370)
  * This includes: `"types"`, `"utils"`, `"metrics-collectors-economic"`, and `"metrics-collectors-rpc-calls"` features.
* Bump alloy to 0.12 in [#381](https://github.com/Layr-Labs/eigensdk-rs/pull/381).

* Added `register_for_operator_sets_with_churn` method to `elcontracts/writer` in [#382](https://github.com/Layr-Labs/eigensdk-rs/pull/382).
  
  ```rust
    let el_chain_writer_2 =
        new_test_writer(http_endpoint.clone(), SECOND_PRIVATE_KEY.to_string()).await;

    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY_2.to_string()).unwrap();
    let churn_private_key = FIRST_PRIVATE_KEY.to_string();
    let churn_sig_salt = FixedBytes::from([0x05; 32]);
    let churn_sig_expiry = U256::MAX;
    
    let tx_hash = el_chain_writer_2
        .register_for_operator_sets_with_churn(
            SECOND_ADDRESS,         // Operator address to register
            bls_key_pair,           // Operator's BLS key pair
            avs_address,            // AVS address
            vec![operator_set_id],  // Operator set ID
            "socket".to_string(),   // Socket address
            Bytes::from([0]),       // Quorum numbers
            vec![FIRST_ADDRESS],    // Operators to kick if quorum is full
            churn_private_key,      // Churn approver's private key
            churn_sig_salt,         // Churn signature salt
            churn_sig_expiry,       // Churn signature expiry
        )
        .await
        .unwrap();
  ```

### Breaking Changes 🛠

* Updated slashing bindings to [the v1.1.1 eigenlayer-middleware release](https://github.com/Layr-Labs/eigenlayer-middleware/releases/tag/v1.1.1-testnet-slashing) [#365](https://github.com/Layr-Labs/eigensdk-rs/pull/365)

* Renamed `set_account_identifier` to `set_avs` [#365](https://github.com/Layr-Labs/eigensdk-rs/pull/365)
  * The underlying call was renamed in [the v1.1.1 eigenlayer-middleware release](https://github.com/Layr-Labs/eigenlayer-middleware/releases/tag/v1.1.1-testnet-slashing).

* Changed the signature of `build_avs_registry_chain_writer` in [#384](https://github.com/Layr-Labs/eigensdk-rs/pull/384).
  * The `operator_state_retriever_addr` parameter was replaced by `service_manager_addr`.
  * This change was made because later middleware versions do not include `ServiceManager`.
  
  ```rust
  // Before
  pub async fn build_avs_registry_chain_writer(
      logger: SharedLogger,
      provider: String,
      signer: String,
      registry_coordinator_addr: Address,
      _operator_state_retriever_addr: Address,
  ) -> Result<Self, AvsRegistryError> {}

  // After
  pub async fn build_avs_registry_chain_writer(
      logger: SharedLogger,
      provider: String,
      signer: String,
      registry_coordinator_addr: Address,
      service_manager_addr: Address,
  ) -> Result<Self, AvsRegistryError> {}
  ```

### Deprecated ⚠️

### Removed 🗑

* Removed unused empty structs from the library in [#371](https://github.com/Layr-Labs/eigensdk-rs/pull/371)
  * `eigen_client_eth::client::Client`
  * `eigen_services_operatorsinfo::OperatorPubKeysService`
* Removed the `AvsRegistryChainReader::is_operator_set_quorum` method [#365](https://github.com/Layr-Labs/eigensdk-rs/pull/365)
  * This function was removed in [the v1.1.1 eigenlayer-middleware release](https://github.com/Layr-Labs/eigenlayer-middleware/releases/tag/v1.1.1-testnet-slashing).

### Documentation 📚

* Reflect 2 bindings(rewardsv2 and slashing) in readme in [#383](https://github.com/Layr-Labs/eigensdk-rs/pull/383).

### Other Changes

## [0.4.0] - 2025-02-20

### Security 🔒

### Added 🎉

* Implemented `create_avs_rewards_submission` [#345](https://github.com/Layr-Labs/eigensdk-rs/pull/345)

  ```rust
    let rewards_submissions = vec![RewardsSubmission {
        strategiesAndMultipliers: strategies_and_multipliers,
        token,
        amount: U256::from(1_000),
        startTimestamp: last_valid_interval_start,
        duration: rewards_duration,
    }];

    let tx_hash = avs_writer
        .create_avs_rewards_submission(rewards_submissions)
        .await
        .unwrap();
  ```

* Added functions `new()` and `validate()` to `Operator` struct in [#280](https://github.com/Layr-Labs/eigensdk-rs/pull/280).

```rust
    let operator = Operator::new( 
      "f39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
      "70997970C51812dc3A010C7d01b50e0d17dc79C8",
      "http://www.example.com/eigensdk-rs.json",
      3
    );
  operator.validate();
```

* Added `OperatorMetadata` struct in [#280](https://github.com/Layr-Labs/eigensdk-rs/pull/280).

```rust
    let operator_metadata = OperatorMetadata {
        name: "Ethereum Utopia".to_string(),
        description: "Rust operator is good operator".to_string(),
        logo: "https://goerli-operator-metadata.s3.amazonaws.com/eigenlayer.png".to_string(),
        website: Some("https://test.com".to_string()),
        twitter: Some("https://twitter.com/test".to_string()),
    };
    operator_metadata.validate();
```

* Added new method `update_avs_metadata_uri` in `avsregistry/writer` in [#344](https://github.com/Layr-Labs/eigensdk-rs/pull/344).

  ```rust
    let tx_hash = avs_writer
        .update_avs_metadata_uri(new_metadata)
        .await
        .unwrap();
  ```

* Added new method `register_operator_with_churn` in `avsregistry/writer` in [#354](https://github.com/Layr-Labs/eigensdk-rs/pull/354).

  ```rust
    let bls_key_pair = BlsKeyPair::new(BLS_KEY).unwrap();
    let operator_sig_salt = FixedBytes::from([0x02; 32]); 
    let operator_sig_expiry = U256::MAX;
    let quorum_nums = Bytes::from([0]);
    let socket = "socket".to_string();
    let churn_sig_salt = FixedBytes::from([0x05; 32]);
    let churn_sig_expiry = U256::MAX;


    let tx_hash = avs_writer_2
        .register_operator_with_churn(
            bls_key_pair,                 // Operator's BLS key pair
            operator_sig_salt,            // Operator signature salt
            operator_sig_expiry,          // Operator signature expiry
            quorum_nums,                  // Quorum numbers for registration
            socket,                       // Socket address
            vec![REGISTERED_OPERATOR],    // Operators to kick if quorum is full
            CHURN_PRIVATE_KEY,            // Churn approver's private key
            churn_sig_salt,               // Churn signature salt
            churn_sig_expiry,             // Churn signature expiry
        )
        .await
        .unwrap();
  ```

* Added new method `set_churn_approver` in `avsregistry/writer` in [#333](https://github.com/Layr-Labs/eigensdk-rs/pull/333).

  ```rust
  let tx_hash = avs_writer
      .set_churn_approver(new_churn_approver_address)
      .await
      .unwrap();
  ```

* Added new method `set_signer` in `ELChainWriter` and `AvsRegistryChainWriter` in [#364](https://github.com/Layr-Labs/eigensdk-rs/pull/364).

  ```rust
  avs_registry_chain_writer.set_signer(PRIVATE_KEY_STRING);
  el_chain_writer.set_signer(PRIVATE_KEY_STRING);
  ```

* Added additional method `register_as_operator_preslashing` in `ELChainWriter` in [#366](https://github.com/Layr-Labs/eigensdk-rs/pull/366).This method is to be used for pre-slashing.

  ```rust
   let operator = Operator {
            address: ADDRESS, 
            delegation_approver_address: ADDRESS,
            metadata_url: "metadata_uri".to_string(),
            allocation_delay: None,
            _deprecated_earnings_receiver_address: None,
            staker_opt_out_window_blocks: Some(0u32),
        };
    el_chain_writer
        .register_as_operator_preslashing(operator)
        .await
        .unwrap();
  ```

### Breaking Changes 🛠

* `TaskMetadata.task_created_block` field changed to `u64` [#362](https://github.com/Layr-Labs/eigensdk-rs/pull/362)

* Separated the interface and service in the `bls_agg` module in [#363](https://github.com/Layr-Labs/eigensdk-rs/pull/363).
  * To start the BLS aggregation service, use `BlsAggregationService::start`. It returns a tuple of `ServiceHandle` and `AggregateReceiver`.
  * To interact with the BLS aggregation service, use the returned structs.
    * Aggregation responses are now handled by the `AggregateReceiver` struct. Use `AggregateReceiver::receive_aggregated_response` instead of reading from the `aggregated_response_receiver` field of `BlsAggregationService`.
    * Task initialization and new signature processing are handled by `ServiceHandle`. It is cloneable, and can be sent to other threads or tasks. Use `ServiceHandle::initialize_task` instead of `BlsAggregationService::initialize_new_task`, and `ServiceHandle::process_signature` instead of `BlsAggregationService::process_new_signature`.
  * Removed `initialize_new_task` and `process_new_signature` from `BlsAggregationService`, along with the field `aggregated_response_receiver`, since their functionality is now exposed by `ServiceHandle` and `AggregateReceiver`.

  ```rust
  // Before
  let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());
  let metadata = TaskMetadata::new(
        task_index,
        block_number,
        quorum_numbers,
        quorum_threshold_percentages,
        time_to_expiry,
    );
    
  bls_agg_service.initialize_new_task(metadata).await.unwrap();

  bls_agg_service
      .process_new_signature(TaskSignature::new(
          task_index,
          task_response_digest,
          bls_signature,
          test_operator_1.operator_id,
      ))
      .await
      .unwrap();

  let aggregated_response = bls_agg_service
          .aggregated_response_receiver
          .lock()
          .await
          .recv()
          .await
          .unwrap();

  // After
  let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());
  let (handle, mut aggregator_response) = bls_agg_service.start();

  let metadata = TaskMetadata::new(
      task_index,
      block_number,
      quorum_numbers,
      quorum_threshold_percentages,
      time_to_expiry,
  );
  handle.initialize_task(metadata).await.unwrap();

  handle
      .process_signature(TaskSignature::new(
          task_index,
          task_response_digest,
          bls_signature,
          test_operator_1.operator_id,
      ))
      .await
      .unwrap();

  let aggregated_response = aggregator_response
      .receive_aggregated_response()
      .await
      .unwrap();
  ```

### Deprecated ⚠️

### Removed 🗑

* Removed `eigen-testing-utils` dependency from `eigen-cli` crate in [#353](https://github.com/Layr-Labs/eigensdk-rs/pull/353).
* Modifications to `eigen-testing-utils` in [#357](https://github.com/Layr-Labs/eigensdk-rs/pull/357).
  * Removed `mine_anvil_blocks_operator_set` from `eigen-testing-utils`. Users should use `mine_anvil_blocks` that does the same thing.
  * Removed the third parameter of `set_account_balance`. Now the port used is the default used on `start_anvil_container` and `start_m2_anvil_container`.

### Documentation 📚

### Other Changes

* fix: missing block while waiting for operator state history in [#290](https://github.com/Layr-Labs/eigensdk-rs/pull/290).

## [0.3.0] - 2025-02-11

### Added

* Added new method `set_slashable_stake_lookahead` in `avsregistry/writer` in [#278](https://github.com/Layr-Labs/eigensdk-rs/pull/278).

  ```rust
    let quorum_number = 0_u8;
    let lookahead = 10_u32;
    let tx_hash = avs_writer
        .set_slashable_stake_lookahead(quorum_number, lookahead)
        .await
        .unwrap();
  ```

* Added new method `set_rewards_initiator` in `avsregistry/writer` in [#273](https://github.com/Layr-Labs/eigensdk-rs/pull/273).

  ```rust
    let tx_hash = avs_writer
      .set_rewards_initiator(new_rewards_init_address)
      .await
      .unwrap();
  ```

* Added new method `clear_deallocation_queue` in `elcontracts/writer` in [#270](https://github.com/Layr-Labs/eigensdk-rs/pull/270)

  ```rust
  let tx_hash_clear = el_chain_writer
      .clear_deallocation_queue(
          operator_address,
          vec![strategy_addr],
          vec![num_to_clear],
      )
      .await
      .unwrap();
  ```

* Added new method `get_restakeable_strategies` in `avsregistry/reader` in [#349](https://github.com/Layr-Labs/eigensdk-rs/pull/349).

  ```rust
    let strategies = avs_reader.get_restakeable_strategies().await.unwrap();
  ```

* Added update_socket function for avs registry writer in [#268](https://github.com/Layr-Labs/eigensdk-rs/pull/268)
  An example of use is the following:

  ```rust
  // Given an avs writer and a new socket address:

  let tx_hash = avs_writer
    .update_socket(new_socket_addr.into())
    .await
    .unwrap();

  let tx_status = wait_transaction(&http_endpoint, tx_hash)
    .await
    .unwrap()
    .status(); 
  // tx_status should be true
  ```

* Added `get_operator_restaked_strategies` in `avsregistry/reader` in [#348](https://github.com/Layr-Labs/eigensdk-rs/pull/348).

  ```rust
    let strategies = avs_reader
      .get_operator_restaked_strategies(FIRST_ADDRESS)
      .await
      .unwrap();
  ```

* Added custom configuration for release-plz in [#281](https://github.com/Layr-Labs/eigensdk-rs/pull/281).

* Added Rewards2.1 support in [#323](https://github.com/Layr-Labs/eigensdk-rs/pull/323).

  * Set an operator's split on an operator set.

  ```rust
      let operator_set = OperatorSet {
            avs: avs_address,
            id: 0,
        };

        let new_split = 5;
        let tx_hash = el_chain_writer
            .set_operator_set_split(OPERATOR_ADDRESS, operator_set.clone(), new_split)
            .await
            .unwrap();
  ```

  * Get an operator's split on an operator set.

  ```rust
     let operator_set = OperatorSet {
            avs: avs_address,
            id: 0,
        };
       let split = el_chain_writer
            .el_chain_reader
            .get_operator_set_split(OPERATOR_ADDRESS, operator_set)
            .await
            .unwrap(); 
  ```

* Added new method `set_operator_set_param` in `avsregistry/writer` in [#327](https://github.com/Layr-Labs/eigensdk-rs/pull/327).

  ```rust
    let operator_set_params = OperatorSetParam {
        maxOperatorCount: 10,
        kickBIPsOfOperatorStake: 50,
        kickBIPsOfTotalStake: 50,
    };

    let tx_hash = avs_writer
        .set_operator_set_param(0, operator_set_params.clone())
        .await
        .unwrap();
  ```

* Added new method `create_operator_directed_avs_rewards_submission` in `avsregistry/writer` in [#352](https://github.com/Layr-Labs/eigensdk-rs/pull/352).

  ```rust
    let operator_rewards = OperatorReward {
        operator: SECOND_ADDRESS,
        amount: U256::from(100),
    };

    let strategies = StrategyAndMultiplier {
        strategy: strategy_address,
        multiplier: U96::from(1),
    };

    let operator_rewards_submission = OperatorDirectedRewardsSubmission {
        token: token_address,
        description: "test".to_string(),
        duration,
        startTimestamp,
        operatorRewards: vec![operator_rewards.clone()],
        strategiesAndMultipliers: vec![strategies],
    };

    let tx_hash = avs_writer
        .create_operator_directed_avs_rewards_submission(vec![operator_rewards_submission])
        .await
        .unwrap();
  ````

* Added new method `create_total_delegated_stake_quorum` in `avsregistry/writer` in [#342](https://github.com/Layr-Labs/eigensdk-rs/pull/342).

  ```rust
    let operator_set_params = OperatorSetParam {
        maxOperatorCount: 10,
        kickBIPsOfOperatorStake: 50,
        kickBIPsOfTotalStake: 50,
    };
    let minimum_stake = U96::from(10);
    let strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
    let strategy_params = StrategyParams {
        strategy,
        multiplier: U96::from(1),
    };
    let strategy_params = vec![strategy_params];

    avs_writer
        .create_total_delegated_stake_quorum(
            operator_set_params,
            minimum_stake,
            strategy_params,
        )
        .await
        .unwrap();
  ```

* Added new method `create_slashable_stake_quorum` in `avsregistry/writer` in [#340](https://github.com/Layr-Labs/eigensdk-rs/pull/340).

  ```rust
      let operator_set_param = OperatorSetParam {
          maxOperatorCount: 10,
          kickBIPsOfOperatorStake: 50,
          kickBIPsOfTotalStake: 50,
      };
      let minimum_stake = U96::from(100);
      let strategy_param = StrategyParams {
          strategy: get_erc20_mock_strategy(http_endpoint.clone()).await,
          multiplier: U96::from(1),
      };
      let look_ahead_period = 10;

      let tx_hash = avs_writer
          .create_slashable_stake_quorum(
              operator_set_param,
              minimum_stake,
              vec![strategy_param],
              look_ahead_period,
          )
          .await
          .unwrap();
  ```

* Added new method `set_ejector` in `avsregistry/writer` in [#330](https://github.com/Layr-Labs/eigensdk-rs/pull/330).
  
  ```rust
    let new_ejector_address = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    let tx_hash = avs_writer.set_ejector(new_ejector_address).await.unwrap();
  ````

* Added new method `set_ejection_cooldown` in `avsregistry/writer` in [#337](https://github.com/Layr-Labs/eigensdk-rs/pull/337).

  ```rust
      let new_cooldown = U256::from(120);
      let tx_hash = avs_writer
          .set_ejection_cooldown(new_cooldown)
          .await
          .unwrap();
  ```

* Added new method `eject_operator` in `avsregistry/writer` in [#328](https://github.com/Layr-Labs/eigensdk-rs/pull/328).

  ```rust
    let register_operator_address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let quorum_nums = Bytes::from([0]);

    let tx_hash = avs_writer
        .eject_operator(register_operator_address, quorum_nums)
        .await
        .unwrap();
  ```

* Added new method `is_operator_set_quorum` in `avsregistry/writer` in [#296](https://github.com/Layr-Labs/eigensdk-rs/pull/296).

  ```rust
    let operator_set_quourm = avs_reader.is_operator_set_quorum(0).await.unwrap();
  ```

* Added version explicitly in crates in [#322](https://github.com/Layr-Labs/eigensdk-rs/pull/322).
* Added new method `set_account_identifier` in `avsregistry/writer` in [#329](https://github.com/Layr-Labs/eigensdk-rs/pull/329).

  ```rust
    let tx_hash = avs_writer
        .set_account_identifier(new_identifier_address)
        .await
        .unwrap();
  ```

* Added missing StakeRegistry writer functions in [#343](https://github.com/Layr-Labs/eigensdk-rs/pull/343).

  * `set_minimum_stake_for_quorum`

    ```rust
    let tx_hash = avs_writer
      .set_minimum_stake_for_quorum(quorum_number, minimum_stake)
      .await
      .unwrap();
    ```

  * add_strategies

  ```rust
  let tx_hash = avs_writer
    .add_strategies(quorum_number, vec_of_strategy_params)
    .await
    .unwrap();
  ```

  * remove_strategies

  ```rust
  let tx_hash = avs_writer
    .remove_strategies(quorum_number, indices_to_remove)
    .await
    .unwrap();
  ```

  * modify_strategy_params

  ```rust
  let tx_hash = avs_writer
      .modify_strategy_params(
          quorum_numbers,
          vec_of_strategy_indices,
          vec_of_new_multipliers,
      )
      .await
      .unwrap();
  ```

* Added missing stake registry view methods in `avsregistry/reader` in [#347](https://github.com/Layr-Labs/eigensdk-rs/pull/347).

  * `weight_of_operator_for_quorum`

  ```rust
     let weight = avs_reader
            .weight_of_operator_for_quorum(quorum_number, operator_address)
            .await
            .unwrap();
  ```

  * `strategy_params_length`

  ```rust
     let len = avs_reader
            .strategy_params_length(quorum_number)
            .await
            .unwrap();
  ```

  * `strategy_params_by_index`

  ```rust
     let params = avs_reader
            .strategy_params_by_index(quorum_number, index)
            .await
            .unwrap();
  ```

  * `get_stake_history_length`

  ```rust
     let len = avs_reader
            .get_stake_history_length(operator_id, quorum_number)
            .await
            .unwrap();
  ```

  * `get_stake_history`

  ```rust
     let stake_update_vec = avs_reader
            .get_stake_history(operator_id, quorum_number)
            .await
            .unwrap();
  ```

  * `get_latest_stake_update`

  ```rust
     let latest_stake_update = avs_reader
            .get_latest_stake_update(operator_id, quorum_number)
            .await
            .unwrap();
  ```

  * `get_stake_update_at_index`

  ```rust
     let stake_update = avs_reader
            .get_stake_update_at_index(quorum_number, operator_id, index)
            .await
            .unwrap();
  ```

  * `get_stake_update_at_block_number`

  ```rust
    let stake_update_at_index = avs_reader
            .get_stake_update_at_block_number(operator_id, quorum_number, (block_number) as u32)
            .await
            .unwrap();
  ```

  * `get_stake_update_index_at_block_number`

  ```rust
      let stake_update_at_index_at_block_number = avs_reader
            .get_stake_update_index_at_block_number(operator_id, quorum_number, block_number as u32)
            .await
            .unwrap();
  ```

  * `get_stake_at_block_number_and_index`

  ```rust
     let stake_at_index_at_block_number = avs_reader
            .get_stake_at_block_number_and_index(
                quorum_number,
                block_number as u32,
                operator_id,
                index,
            )
            .await
            .unwrap();
  ```

  * `get_total_stake_history_length`

  ```rust
     let total_stake_history_length = avs_reader
            .get_total_stake_history_length(quorum_number)
            .await
            .unwrap();
  ```

  * `get_current_total_stake`

  ```rust
     let current_total_stake = avs_reader
            .get_current_total_stake(quorum_number)
            .await
            .unwrap();

  ```

  * `get_total_stake_update_at_index`

  ```rust
     let total_stake_update_at_index = avs_reader
            .get_total_stake_update_at_index(quorum_number, index)
            .await
            .unwrap();
  ```

  * `get_total_stake_at_block_number_from_index`

  ```rust
     let total_stake_at_block_number_from_index = avs_reader
            .get_total_stake_at_block_number_from_index(
                quorum_number,
                block_number as u32,
                index,
            )
            .await
            .unwrap();
  ```

  * `get_total_stake_indices_at_block_number`

  ```rust
      let total_stake_indices_at_block_number = avs_reader
            .get_total_stake_indices_at_block_number(block_number as u32, quorum_nums)
            .await
            .unwrap();
  ```

### Changed

### Breaking changes

* refactor: update interface on `bls aggregation` in [#254](https://github.com/Layr-Labs/eigensdk-rs/pull/254)
  * Introduces a new struct `TaskMetadata` with a constructor `TaskMetadata::new` to initialize a new task and a method `with_window_duration` to set the window duration.
  * Refactors `initialize_new_task` and `single_task_aggregator` to accept a `TaskMetadata` struct instead of multiple parameters.

    ```rust
    // BEFORE
    bls_agg_service
          .initialize_new_task(
              task_index,
              block_number as u32,
              quorum_numbers,
              quorum_threshold_percentages,
              time_to_expiry,
          )
          .await
          .unwrap();
    
    // AFTER
    let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
      )
    bls_agg_service.initialize_new_task(metadata).await.unwrap();
    ```

  * Removes `initialize_new_task_with_window` since `window_duration` can now be set in `TaskMetadata`.

    ```rust
    // BEFORE
    bls_agg_service
          .initialize_new_task_with_window(
              task_index,
              block_number as u32,
              quorum_numbers,
              quorum_threshold_percentages,
              time_to_expiry,
              window_duration,
          )
          .await
          .unwrap();

    // AFTER
    let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        ).with_window_duration(window_duration);
    bls_agg_service.initialize_new_task(metadata).await.unwrap();
* refactor: encapsulate parameters into `TaskSignature` in [#260](https://github.com/Layr-Labs/eigensdk-rs/pull/260)

  * Introduced `TaskSignature` struct to encapsulate parameters related to task signatures:
  * Updated `process_new_signature` to accept a `TaskSignature` struct instead of multiple parameters.

    ```rust
    // BEFORE
    bls_agg_service.process_new_signature(task_index, task_response_digest, bls_signature, operator_id).await.unwrap();

    // AFTER
    let task_signature = TaskSignature::new(
          task_index,
          task_response_digest,
          bls_signature,
          operator_id,
    );
    bls_agg_service.process_new_signature(task_signature).await.unwrap();
    ```

* Slashing UAM changes in [#248](https://github.com/Layr-Labs/eigensdk-rs/pull/248).

### Removed

## [0.2.0] - 2025-02-06

### Security 🔒

* chore(deps): bump openssl from 0.10.68 to 0.10.70 in the cargo group across 1 directory by @dependabot in <https://github.com/Layr-Labs/eigensdk-rs/pull/291>

### Added 🎉

* Added `eigen_common` dependency to the `eigensdk` crate when "full" feature is enabled in [#249](https://github.com/Layr-Labs/eigensdk-rs/pull/249).
  * Now when enabling the "full" feature:

    ```toml
    eigensdk = { version = "0.2", features = ["full"] }
    ```

    You can use access the `eigen-common` crate as a submodule of `eigensdk`:

    ```rust
    use eigensdk::common::*;
    ```

* Added bindings for `ECDSAStakeRegistry` and `ECDSAServiceManagerBase` in [#269](https://github.com/Layr-Labs/eigensdk-rs/pull/269).
  * These bindings can be accessed from:

    ```rust
    // From `eigensdk`
    use eigensdk::utils::middleware::ecdsaservicemanagerbase;
    use eigensdk::utils::middleware::ecdsastakeregistry;
    // From `eigen_utils`
    use eigen_utils::middleware::ecdsaservicemanagerbase;
    use eigen_utils::middleware::ecdsastakeregistry;
    ```

* Starting on this release, we're using [`release-plz`](https://github.com/release-plz/release-plz) to streamline our release process.
  * Added release-plz in ci in [#275](https://github.com/Layr-Labs/eigensdk-rs/pull/275).
  * Added custom configuration for release-plz in [#281](https://github.com/Layr-Labs/eigensdk-rs/pull/281).
  * Fixed typo in release-plz toml file in [#284](https://github.com/Layr-Labs/eigensdk-rs/pull/284).

### Breaking Changes 🛠

* fix: use rewards coordinator on get operator avs/pi split methods by @maximopalopoli in <https://github.com/Layr-Labs/eigensdk-rs/pull/250>

  * The parameters of `ChainReader::new` changed, and it now receives the address of the rewards coordinator.

    It was previously called this way:

    ```rust
    let el_chain_reader = ELChainReader::new(
        logger,
        SLASHER_ADDRESS,
        DELEGATION_MANAGER_ADDRESS,
        AVS_DIRECTORY_ADDRESS,
        provider_url,
    );
    ```

    Now, it's called this way:

    ```rust
    let el_chain_reader = ELChainReader::new(
        logger,
        SLASHER_ADDRESS,
        DELEGATION_MANAGER_ADDRESS,
        REWARDS_COORDINATOR,
        AVS_DIRECTORY_ADDRESS,
        provider_url,
    );
    ```

### Removed 🗑

* Removed homepage from testing-utils crate in [#266](https://github.com/Layr-Labs/eigensdk-rs/pull/266).
* Removed changelog generation by release-plz in [#281](https://github.com/Layr-Labs/eigensdk-rs/pull/281).
* Removed examples packages from workspace.dependencies in Cargo.toml in [#287](https://github.com/Layr-Labs/eigensdk-rs/pull/287).
* Removed release-plz-pr workflow in release-plz in [#292](https://github.com/Layr-Labs/eigensdk-rs/pull/292).

### Documentation 📚

* Fixed the rewardsv2 bindings version in readme to 0.5.4 in [#246](https://github.com/Layr-Labs/eigensdk-rs/pull/246).
* docs: improve changelog by adding examples by @maximopalopoli in <https://github.com/Layr-Labs/eigensdk-rs/pull/251>

### Other Changes

* Changes in the way bindings are generated in [#243](https://github.com/Layr-Labs/eigensdk-rs/pull/243).
  * The `bindings` target now generates the bindings using Docker with Foundry v0.3.0.
  * The previous `bindings` target was renamed to `bindings_host`, as it runs without Docker. However the `bindings_host` target is for CI use only. To generate the bindings, please use the `bindings` target.
* Fixed incorrect package name in Cargo.toml for examples in [#285](https://github.com/Layr-Labs/eigensdk-rs/pull/285).
* docs: add mention of updated bindings to changelog by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/233>
* chore: format contracts by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/235>
* ci: add foundry workflow by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/236>
* ci: add CI job to check whether anvil state is up to date by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/237>
* chore: remove existing bindings when generating new ones by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/242>
* chore: remove alloy reexported crates from dependencies by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/244>
* docs: sync root and `crates/eigensdk/` READMEs by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/245>
* ci: add workflow to enforce updates to the changelog by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/239>
* docs: add `RELEASE.md` by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/231>
* ci: fix check bindings job by @pablodeymo in <https://github.com/Layr-Labs/eigensdk-rs/pull/247>
* ci: fix job that checks anvil state is up-to-date by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/252>
* refactor: move bindings generation to script by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/271>
* fix: simplify Cargo.toml by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/282>
* ci: split tests and coverage by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/286>

## [0.1.3] - 2024-01-17

### Added 🎉

* feat: add rewards-v2 related functionality by @supernovahs in <https://github.com/Layr-Labs/eigensdk-rs/pull/221>
  * New methods in `ELChainReader`:
    * `get_operator_avs_split`

    ```rust
    // Given a chain_reader, an operator_address and an avs_address:

    let split = el_chain_reader
        .get_operator_avs_split(operator_address, avs_address)
        .await
        .unwrap();
    ```

    * `get_operator_pi_split`

    ```rust
    // Given a chain_reader and an operator_address:

    let split = el_chain_writer
        .el_chain_reader
        .get_operator_pi_split(operator_address)
        .await
        .unwrap();
    ```

  * New methods in `ELChainWriter`:
    * `set_operator_avs_split`

    ```rust
    // Given a chain_writer, an operator_address, an avs_address and a split:

    let tx_hash = el_chain_writer
        .set_operator_avs_split(operator_address, avs_address, split)
        .await
        .unwrap();
    let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
    ```

    * `set_operator_pi_split`

    ```rust
    // Given a chain_writer, an operator_address and a split:

    let tx_hash = el_chain_writer
        .set_operator_pi_split(operator, split)
        .await
        .unwrap();
    let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
    ```

  * Bindings updated for rewards-v2 core contracts release

### Breaking Changes 🛠

* feat!: remove delegation manager from `ELChainWriter` by @supernovahs in <https://github.com/Layr-Labs/eigensdk-rs/pull/214>
  * `ELChainWriter::new` no longer receives the delegation manager address as first parameter.
    Before, a chainWriter was created this way:

    ```rust
        let el_writer = ELChainWriter::new(
            DELEGATION_MANAGER_ADDRESS,
            STRATEGY_MANAGER_ADDRESS,
            REWARDS_COORDINATOR,
            el_chain_reader,
            "https://ethereum-holesky.blockpi.network/v1/rpc/public".to_string(),
            "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8".to_string(),
        );
    ```

    Now, the creation is done without the delegation manager address:

    ```rust
        let el_writer = ELChainWriter::new(
            STRATEGY_MANAGER_ADDRESS,
            REWARDS_COORDINATOR,
            el_chain_reader,
            "<https://ethereum-holesky.blockpi.network/v1/rpc/public>".to_string(),
            "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8".to_string(),
        );
    ```

* feat!: change way bindings are generated by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/204>
  * `eigen_utils::core` contains bindings related to core contracts
  * `eigen_utils::middleware` contains bindings related to middleware contracts
  * `eigen_utils::sdk` contains bindings related to the SDK (should only be used for testing)
  
  Now, to update the bindings, run `make bindings`. This command will generate the bindings files in the folder: `crates/utils`.

### Documentation 📚

* docs: add CHANGELOG.md by @lferrigno in <https://github.com/Layr-Labs/eigensdk-rs/pull/220>

### Other Changes

* ci: change docker setup action for official one by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/219>
* docs: add error message for `cargo test` on darwin by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/215>
* test: fix `test_register_and_update_operator` by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/223>
* chore: update way anvil state dump is generated by @ricomateo in <https://github.com/Layr-Labs/eigensdk-rs/pull/222>
* fix: disable doctests on `eigen-utils` by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/227>
* chore: bump version by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/228>
* docs: add GitHub release changelog configuration by @MegaRedHand in <https://github.com/Layr-Labs/eigensdk-rs/pull/229>

## [0.1.2] - 2025-01-09

### Added

* Added retries with exponential backoff to send transactions in [#158](https://github.com/Layr-Labs/eigensdk-rs/pull/158)
* Added `query_registration_detail` method in [#162](https://github.com/Layr-Labs/eigensdk-rs/pull/162)
* Added clippy lints in `Cargo.toml` in [#159](https://github.com/Layr-Labs/eigensdk-rs/pull/159)
* Added BLS aggregation logger in [#154](https://github.com/Layr-Labs/eigensdk-rs/pull/154)
* Added `common` crate to `eigensdk` crate in [#213](https://github.com/Layr-Labs/eigensdk-rs/pull/213)

### Changed

* Updated `eigenlayer-middleware` to v0.4.3 (rewards release) in [#177](https://github.com/Layr-Labs/eigensdk-rs/pull/177)
* Fixed Holesky RPC provider URL in [#184](https://github.com/Layr-Labs/eigensdk-rs/pull/184)
* Fixed BLS signature logic in [#174](https://github.com/Layr-Labs/eigensdk-rs/pull/174)

### Removed

* Deleted `TxManager` in [#151](https://github.com/Layr-Labs/eigensdk-rs/pull/151)
* Removed `TxManager` crate import in [#211](https://github.com/Layr-Labs/eigensdk-rs/pull/211)
* Removed logs in `operatorsinfo` test in [#185](https://github.com/Layr-Labs/eigensdk-rs/pull/185)

### Documentation

* Added notes for running tests in [#194](https://github.com/Layr-Labs/eigensdk-rs/pull/194)
* Added "Contract Bindings" section to the README in [#178](https://github.com/Layr-Labs/eigensdk-rs/pull/178)
* Added "Branches" section to the README in [#200](https://github.com/Layr-Labs/eigensdk-rs/pull/200)

## Previous versions

This changelog was introduced in-between v0.1.2 and v0.1.3.
For changes from previous releases, you can check on our GitHub repo's releases page: [github.com/Layr-Labs/eigensdk-rs/releases](https://github.com/Layr-Labs/eigensdk-rs/releases)
