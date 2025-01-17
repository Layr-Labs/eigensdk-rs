# Changelog

All notable changes to this project will be documented in this file.

Changelog is present since v0.1.2

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Each version will have a separate `Breaking Changes` section as well. To describe in how to upgrade from one version to another if needed

## [Unreleased]
### Added
### Changed
### Breaking changes
### Removed



## [0.1.2] - 2024-11-19
### Added
- Added retries with exponential backoff to send transactions in [#158](https://github.com/Layr-Labs/eigensdk-rs/pull/158)
- Added `query_registration_detail` method in [#162](https://github.com/Layr-Labs/eigensdk-rs/pull/162)
- Added clippy lints in `Cargo.toml` in [#159](https://github.com/Layr-Labs/eigensdk-rs/pull/159)
- Added BLS aggregation logger in [#154](https://github.com/Layr-Labs/eigensdk-rs/pull/154)
- Added `common` crate to `eigensdk` crate in [#213](https://github.com/Layr-Labs/eigensdk-rs/pull/213)

### Changed
- Updated `eigenlayer-middleware` to v0.4.3 (rewards release) in [#177](https://github.com/Layr-Labs/eigensdk-rs/pull/177)
- Fixed Holesky RPC provider URL in [#184](https://github.com/Layr-Labs/eigensdk-rs/pull/184)
- Fixed BLS signature logic in [#174](https://github.com/Layr-Labs/eigensdk-rs/pull/174)

### Removed
- Deleted `TxManager` in [#151](https://github.com/Layr-Labs/eigensdk-rs/pull/151)
- Removed `TxManager` crate import in [#211](https://github.com/Layr-Labs/eigensdk-rs/pull/211)
- Removed logs in `operatorsinfo` test in [#185](https://github.com/Layr-Labs/eigensdk-rs/pull/185)

### Documentation
- Added notes for running tests in [#194](https://github.com/Layr-Labs/eigensdk-rs/pull/194)
- Added "Contract Bindings" section to the README in [#178](https://github.com/Layr-Labs/eigensdk-rs/pull/178)
- Added "Branches" section to the README in [#200](https://github.com/Layr-Labs/eigensdk-rs/pull/200)
