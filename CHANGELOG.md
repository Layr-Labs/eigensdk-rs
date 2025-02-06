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

### Breaking Changes 🛠

### Deprecated ⚠️

### Removed 🗑

### Documentation 📚

### Other Changes

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

## [0.1.3] - 2025-01-17

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
