use ethers::types::U256;
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
