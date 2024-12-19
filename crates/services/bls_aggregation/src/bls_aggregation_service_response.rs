use eigen_crypto_bls::{BlsG1Point, BlsG2Point, Signature};
use eigen_types::avs::{TaskIndex, TaskResponseDigest};
use serde::{Deserialize, Serialize};

/// The response from the BLS aggregation service
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlsAggregationServiceResponse {
    pub task_index: TaskIndex,
    pub task_response_digest: TaskResponseDigest,
    pub non_signers_pub_keys_g1: Vec<BlsG1Point>,
    pub quorum_apks_g1: Vec<BlsG1Point>,
    pub signers_apk_g2: BlsG2Point,
    pub signers_agg_sig_g1: Signature,
    pub non_signer_quorum_bitmap_indices: Vec<u32>,
    pub quorum_apk_indices: Vec<u32>,
    pub total_stake_indices: Vec<u32>,
    pub non_signer_stake_indices: Vec<Vec<u32>>,
}
