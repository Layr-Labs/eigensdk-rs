use alloy_primitives::{Address, FixedBytes, U256};
use eigen_crypto_bls::{BlsG1Point, BlsG2Point};
use eigen_utils::binding::BLSApkRegistry;
use ethers::{types::U64, utils::keccak256};
use num_bigint::BigUint;
use std::collections::HashMap;
const MAX_NUMBER_OF_QUORUMS: usize = 192;
use ark_serialize::CanonicalSerialize;

pub type OperatorId = FixedBytes<32>;

pub fn bitmap_to_quorum_ids(quorum_bitmaps: U256) -> Vec<u8> {
    let bytes = quorum_bitmaps.to_be_bytes::<32>();

    let mut quorum_ids: Vec<u8> = Vec::with_capacity(MAX_NUMBER_OF_QUORUMS);

    for i in 0..MAX_NUMBER_OF_QUORUMS {
        let bitmap = BigUint::from_bytes_be(&bytes);
        if bitmap.bit(i.try_into().unwrap()) {
            quorum_ids.push(i.try_into().unwrap());
        }
    }
    quorum_ids
}

#[derive(Debug, Clone)]
pub struct OperatorPubKeys {
    pub g1_pub_key: BlsG1Point,
    pub g2_pub_key: BlsG2Point,
}

pub struct Operator {
    address: Address,
    earnings_receiver_address: Address,
    delegation_approver_address: Address,
    staker_opt_out_window_blocks: u32,
    metadata_url: Option<String>,
}

impl Operator {
    pub fn new(
        address: Address,
        earnings_receiver_address: Address,
        delegation_approver_address: Address,
        staker_opt_out_window_blocks: u32,
        metadata_url: Option<String>,
    ) -> Self {
        Operator {
            address,
            earnings_receiver_address,
            delegation_approver_address,
            staker_opt_out_window_blocks,
            metadata_url,
        }
    }

    pub fn address(&mut self, address: Address) {
        self.address = address;
    }

    pub fn metadata_url(&mut self, metadata: String) {
        self.metadata_url = Some(metadata);
    }

    pub fn earnings_receiver_address(&mut self, address: Address) {
        self.earnings_receiver_address = address;
    }

    pub fn delegation_approver_address(&mut self, address: Address) {
        self.delegation_approver_address = address;
    }

    pub fn staker_opt_out_window_blocks(&mut self, block: u32) {
        self.staker_opt_out_window_blocks = block;
    }

    pub fn has_address(&self) -> Address {
        self.address
    }

    pub fn has_metadata_url(&self) -> Option<String> {
        self.metadata_url.clone()
    }

    pub fn has_earnings_receiver_address(&self) -> Address {
        self.earnings_receiver_address
    }

    pub fn has_delegation_approver_address(&self) -> Address {
        self.delegation_approver_address
    }

    pub fn has_staker_opt_out_window_blocks(&self) -> u32 {
        self.staker_opt_out_window_blocks
    }
}

pub type Socket = String;

pub type QuorumNum = u8;

pub struct OperatorInfo {
    pub pub_keys: Option<OperatorPubKeys>,
}

pub struct OperatorAvsState {
    pub operator_id: [u8; 32],
    pub operator_info: OperatorInfo,
    pub stake_per_quorum: HashMap<QuorumNum, U256>,
    pub block_num: U64,
}

pub fn operator_id_from_g1_pub_key(pub_key: BlsG1Point) -> [u8; 32] {
    let mut bytes = Vec::new();
    let g1 = pub_key.g1();
    g1.serialize_uncompressed(&mut bytes).unwrap(); // check unwrap
    keccak256(bytes)
}

pub struct QuorumAvsState {
    pub quorum_num: u8,
    pub total_stake: U256,
    pub agg_pub_key_g1: BlsG1Point,
    pub block_num: u32,
}

pub type QuorumThresholdPercentage = u8;

pub type QuorumThresholdPercentages = Vec<QuorumThresholdPercentage>;
