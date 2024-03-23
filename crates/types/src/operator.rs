use eigensdk_crypto_bls::attestation::{G1Point, G2Point};
use ethers::types::{Address, U256};
use num_bigint::BigUint;
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

pub struct OperatorPubKeys {
    pub g1_pub_key: G1Point,
    pub g2_pub_key: G2Point,
}

pub struct Operator {
    address: Address,
    earnings_receiver_address: Address,
    delegation_approver_address: Address,
    staker_opt_out_window_blocks: u32,
}

impl Operator {
    pub fn new(
        address: Address,
        earnings_receiver_address: Address,
        delegation_approver_address: Address,
        staker_opt_out_window_blocks: u32,
    ) -> Self {
        Operator {
            address,
            earnings_receiver_address,
            delegation_approver_address,
            staker_opt_out_window_blocks,
        }
    }

    fn address(&mut self, address: Address) {
        self.address = address;
    }

    fn earnings_receiver_address(&mut self, address: Address) {
        self.earnings_receiver_address = address;
    }

    fn delegation_approver_address(&mut self, address: Address) {
        self.delegation_approver_address = address;
    }

    fn staker_opt_out_window_blocks(&mut self, block: u32) {
        self.staker_opt_out_window_blocks = block;
    }
}
