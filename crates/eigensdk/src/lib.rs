#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/* --------------------------------------- Core re-exports -------------------------------------- */

#[doc(inline)]
#[cfg(feature = "types")]
pub use eigen_types as types;

#[doc(inline)]
#[cfg(feature = "utils")]
pub use eigen_utils as utils;

#[doc(inline)]
#[cfg(feature = "crypto-bls")]
pub use eigen_crypto_bls as crypto_bls;

#[doc(inline)]
#[cfg(feature = "crypto-bn254")]
pub use eigen_crypto_bn254 as crypto_bn254;

#[doc(inline)]
#[cfg(feature = "signer")]
pub use eigen_signer as signer;

#[doc(inline)]
#[cfg(feature = "logging")]
pub use eigen_logging as logging;

#[doc(inline)]
#[cfg(feature = "metrics")]
pub use eigen_metrics as metrics;

/* ------------------------------------- Client Re-exports ------------------------------------- */

#[doc(inline)]
#[cfg(feature = "client-avsregistry")]
pub use eigen_client_avsregistry as client_avsregistry;

#[doc(inline)]
#[cfg(feature = "client-elcontracts")]
pub use eigen_client_elcontracts as client_elcontracts;

#[doc(inline)]
#[cfg(feature = "client-eth")]
pub use eigen_client_eth as client_eth;

#[doc(inline)]
#[cfg(feature = "client-fireblocks")]
pub use eigen_client_fireblocks as client_fireblocks;

/* ------------------------------------- Services Re-exports ------------------------------------- */

#[doc(inline)]
#[cfg(feature = "services-avsregistry")]
pub use eigen_services_avsregistry as services_avsregistry;

#[doc(inline)]
#[cfg(feature = "services-blsaggregation")]
pub use eigen_services_blsaggregation as services_blsaggregation;

#[doc(inline)]
#[cfg(feature = "services-operatorsinfo")]
pub use eigen_services_operatorsinfo as services_operatorsinfo;

/* ------------------------------------ Node API Re-export ------------------------------------ */

#[doc(inline)]
#[cfg(feature = "nodeapi")]
pub use eigen_nodeapi as nodeapi;

/* ------------------------------------ Testing Utils Re-export -------------------------------- */

#[doc(inline)]
#[cfg(feature = "testing-utils")]
pub use eigen_testing_utils as testing_utils;

/* ------------------------------------ Metrics Collectors Re-exports -------------------------- */

#[doc(inline)]
#[cfg(feature = "metrics-collectors-economic")]
pub use eigen_metrics_collectors_economic as metrics_collectors_economic;

#[doc(inline)]
#[cfg(feature = "metrics-collectors-rpc-calls")]
pub use eigen_metrics_collectors_rpc_calls as metrics_collectors_rpc_calls;

/* ------------------------------------ Common Utilities Re-exports -------------------------- */

#[doc(inline)]
#[cfg(feature = "common")]
pub use eigen_common as common;
