use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ff::{BigInteger, BigInteger256};
use ethers::core::types::U256;
use std::ops::Mul;
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

pub fn biginteger256_to_u256(bi: BigInteger256) -> U256 {
    let s = bi.to_bytes_be();
    U256::from_little_endian(&s)
}

pub fn get_g1_generator() -> G1Affine {
    let x = Fq::from_str("1").unwrap();
    let y = Fq::from_str("2").unwrap();

    G1Affine::new(x, y)
}

pub fn get_g2_generator() -> G2Affine {
    let x = Fq2::new(
        Fq::from_str(
            "10857046999023057135944570762232829481370756359578518086990519993285655852781",
        )
        .unwrap(),
        Fq::from_str(
            "11559732032986387107991004021392285783925812861821192530917403151452391805634",
        )
        .unwrap(),
    );
    let y = Fq2::new(
        Fq::from_str(
            "8495653923123431417604973247489272438418190587263600148770280649306958101930",
        )
        .unwrap(),
        Fq::from_str(
            "4082367875863433681332203403145435568316851327593401208105741076214120093531",
        )
        .unwrap(),
    );

    G2Affine::new(x, y)
}

pub fn mul_by_generator_g1(pvt_key: Fr) -> G1Projective {
    let g1_gen: G1Projective = get_g1_generator().into();

    g1_gen.mul(pvt_key)
}

pub fn mul_by_generator_g2(pvt_key: Fr) -> G2Projective {
    let g2_gen: G2Projective = get_g2_generator().into();

    g2_gen.mul(pvt_key)
}
