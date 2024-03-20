use ark_bn254::{Fq, Fr, G1Affine, G1Projective};
use ark_ff::{BigInteger, BigInteger256};
use ethers::core::types::U256;
use std::str::FromStr;
/// Converts [U256] to [BigInteger256]
pub fn u256_to_bigint256(value: U256) -> BigInteger256 {
    // Convert U256 to a byte array
    let mut bytes = [0u8; 32];
    value.to_big_endian(&mut bytes);
    // Convert the byte array to a bit array
    let mut bits = [false; 256];
    for (byte_idx, byte) in bytes.iter().enumerate() {
        for bit_idx in 0..8 {
            let bit = byte & (1 << bit_idx) != 0;
            bits[byte_idx * 8 + bit_idx] = bit;
        }
    }
    // Create a BigInteger256 from the byte array
    BigInteger256::from_bits_be(&bits)
}

pub fn get_g1_generator() -> G1Affine {
    let x = Fq::from_str("1").unwrap();
    let y = Fq::from_str("2").unwrap();

    G1Affine::new(x, y)
}

pub fn mul_by_generator_g1(pvt_key: Fr) -> G1Projective {
    let g1_gen: G1Projective = get_g1_generator().into();

    g1_gen * pvt_key
}
