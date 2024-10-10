use alloy_primitives::{aliases::U192, Address, FixedBytes, U256};
use ark_serialize::{CanonicalSerialize, SerializationError};
use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair};
use ethers::{types::U64, utils::keccak256};
use num_bigint::BigUint;
use std::collections::HashMap;

const MAX_NUMBER_OF_QUORUMS: u8 = 192;

pub type OperatorId = FixedBytes<32>;

pub fn bitmap_to_quorum_ids(quorum_bitmaps: U256) -> Vec<u8> {
    let bytes = quorum_bitmaps.to_be_bytes::<32>();

    let mut quorum_ids: Vec<u8> = Vec::with_capacity(usize::from(MAX_NUMBER_OF_QUORUMS));

    for i in 0..MAX_NUMBER_OF_QUORUMS {
        let bitmap = BigUint::from_bytes_be(&bytes);
        if bitmap.bit(u64::from(i)) {
            quorum_ids.push(i);
        }
    }
    quorum_ids
}

pub fn bitmap_to_quorum_ids_from_u192(quorum_bitmaps: U192) -> Vec<u8> {
    let bytes = quorum_bitmaps.to_be_bytes::<32>();

    let mut quorum_ids: Vec<u8> = Vec::with_capacity(usize::from(MAX_NUMBER_OF_QUORUMS));

    for i in 0..MAX_NUMBER_OF_QUORUMS {
        let bitmap = BigUint::from_bytes_be(&bytes);
        if bitmap.bit(u64::from(i)) {
            quorum_ids.push(i);
        }
    }
    quorum_ids
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPubKeys {
    pub g1_pub_key: BlsG1Point,
    pub g2_pub_key: BlsG2Point,
}

impl From<BlsKeyPair> for OperatorPubKeys {
    fn from(keypair: BlsKeyPair) -> Self {
        Self {
            g1_pub_key: keypair.public_key(),
            g2_pub_key: keypair.public_key_g2(),
        }
    }
}

pub struct Operator {
    pub address: Address,
    pub earnings_receiver_address: Address,
    pub delegation_approver_address: Address,
    pub staker_opt_out_window_blocks: u32,
    pub metadata_url: Option<String>,
}

pub type Socket = String;

pub type QuorumNum = u8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorInfo {
    pub pub_keys: Option<OperatorPubKeys>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorAvsState {
    pub operator_id: [u8; 32],
    pub operator_info: OperatorInfo,
    pub stake_per_quorum: HashMap<QuorumNum, U256>,
    pub block_num: U64,
}

pub fn operator_id_from_g1_pub_key(pub_key: BlsG1Point) -> Result<[u8; 32], SerializationError> {
    let mut bytes = Vec::new();
    let g1 = pub_key.g1();
    g1.serialize_uncompressed(&mut bytes)?;
    Ok(keccak256(bytes))
}

#[derive(Debug, PartialEq, Eq)]
pub struct QuorumAvsState {
    pub quorum_num: u8,
    pub total_stake: U256,
    pub agg_pub_key_g1: BlsG1Point,
    pub block_num: u32,
}

pub type QuorumThresholdPercentage = u8;

pub type QuorumThresholdPercentages = Vec<QuorumThresholdPercentage>;
