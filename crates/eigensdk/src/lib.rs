#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/* --------------------------------------- Core re-exports -------------------------------------- */

#[doc(inline)]
#[cfg(feature = "types")]
pub use eigen_types as types;

#[cfg(feature = "utils")]
#[doc(inline)]
pub use eigen_utils as utils;

#[cfg(feature = "crypto-bls")]
#[doc(inline)]
pub use eigen_crypto_bls as crypto_bls;

#[cfg(feature = "crypto-bn254")]
#[doc(inline)]
pub use eigen_crypto_bn254 as crypto_bn254;

#[cfg(feature = "signer")]
#[doc(inline)]
pub use eigen_signer as signer;

#[cfg(feature = "logging")]
#[doc(inline)]
pub use eigen_logging as logging;

#[cfg(feature = "metrics")]
#[doc(inline)]
pub use eigen_metrics as metrics;

#[cfg(feature = "contract-bindings")]
#[doc(inline)]
pub use eigen_contract_bindings as contract_bindings;

/* ------------------------------------- Client Re-exports ------------------------------------- */

#[cfg(feature = "client-avsregistry")]
#[doc(inline)]
pub use eigen_client_avsregistry as client_avsregistry;

#[cfg(feature = "client-elcontracts")]
#[doc(inline)]
pub use eigen_client_elcontracts as client_elcontracts;

#[cfg(feature = "client-eth")]
#[doc(inline)]
pub use eigen_client_eth as client_eth;

#[cfg(feature = "client-fireblocks")]
#[doc(inline)]
pub use eigen_client_fireblocks as client_fireblocks;

/* ------------------------------------- Services Re-exports ------------------------------------- */

#[cfg(feature = "services-avsregistry")]
#[doc(inline)]
pub use eigen_services_avsregistry as services_avsregistry;

#[cfg(feature = "services-blsaggregation")]
#[doc(inline)]
pub use eigen_services_blsaggregation as services_blsaggregation;

#[cfg(feature = "services-operatorsinfo")]
#[doc(inline)]
pub use eigen_services_operatorsinfo as services_operatorsinfo;

/* ------------------------------------ Node API Re-export ------------------------------------ */

#[cfg(feature = "nodeapi")]
#[doc(inline)]
pub use eigen_nodeapi as nodeapi;

/* ------------------------------------ Testing Utils Re-export -------------------------------- */

#[cfg(feature = "testing-utils")]
#[doc(inline)]
pub use eigen_testing_utils as testing_utils;

/* ------------------------------------ Chain IO Re-exports ------------------------------------ */

#[cfg(feature = "chainio-txmanager")]
#[doc(inline)]
pub use eigen_chainio_txmanager as chainio_txmanager;

/* ------------------------------------ Metrics Collectors Re-exports -------------------------- */

#[cfg(feature = "metrics-collectors-economic")]
#[doc(inline)]
pub use eigen_metrics_collectors_economic as metrics_collectors_economic;

#[cfg(feature = "metrics-collectors-rpc-calls")]
#[doc(inline)]
pub use eigen_metrics_collectors_rpc_calls as metrics_collectors_rpc_calls;
