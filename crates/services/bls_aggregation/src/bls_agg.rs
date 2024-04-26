use eigensdk_crypto_bls::attestation::{G1Point, G2Point, Signature};
use eigensdk_types::avs::{SignedTaskResponseDigest, TaskIndex, TaskResponseDigest};
use ethers::abi::Hash;
use ethers_core::types::U256;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BlsAggregationServiceResponse {
    task_index: TaskIndex,
    task_response_digest: TaskResponseDigest,
    non_signers_pub_keys_g1: Vec<G1Point>,
    quorum_apks_g1: Vec<G1Point>,
    signers_apk_g2: Vec<G2Point>,
    signers_agg_sig_g1: Vec<Signature>,
    non_signer_quorum_bitmap_indices: Vec<u32>,
    quorum_apk_indices: Vec<u32>,
    total_stake_indices: Vec<u32>,
    non_signer_stake_indices: Vec<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct AggregatedOperators {
    signers_apk_g2: G2Point,

    signers_agg_sig_g1: Signature,

    signers_total_stake_per_quorum: HashMap<u8, U256>,

    signers_operator_ids_set: HashMap<[u8; 32], bool>,
}

#[derive(Debug, Clone)]
pub struct BlsAggregatorService {
    aggregated_response: BlsAggregationServiceResponse,

    signed_task_resps: HashMap<TaskIndex, SignedTaskResponseDigest>,
}
