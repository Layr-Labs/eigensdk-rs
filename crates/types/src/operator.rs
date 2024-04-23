use eigensdk_crypto_bls::attestation::{G1Point, G2Point};
use ethers::types::{Address, U256,U64};
use num_bigint::BigUint;
use std::collections::HashMap;
const MAX_NUMBER_OF_QUORUMS: usize = 192;
    

pub fn bitmap_to_quorum_ids(quorum_bitmaps: U256) -> Vec<u8> {
    let mut bytes: [u8; 32] = [0u8; 32];
    quorum_bitmaps.to_big_endian(&mut bytes);

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
    pub g1_pub_key: G1Point,
    pub g2_pub_key: G2Point,
}

pub struct Operator {
    address: Address,
    earnings_receiver_address: Address,
    delegation_approver_address: Address,
    staker_opt_out_window_blocks: u32,
    metadata_url: String,
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
            metadata_url: metadata_url.unwrap(),
        }
    }

    pub fn address(&mut self, address: Address) {
        self.address = address;
    }

    pub fn metadata_url(&mut self, metadata: String) {
        self.metadata_url = metadata;
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

    pub fn has_metadata_url(&self) -> String {
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

type Socket = String;

type QuorumNum = u8;

pub struct OperatorInfo{
    socket: Socket,
    pub_keys : OperatorPubKeys
}

pub struct OperatorAvsState{
    operator_id : [u8;32],
    operator_info: OperatorInfo,
    stake_per_quorum : HashMap<QuorumNum,U256>,
    block_num : U64    
}

