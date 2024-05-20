use crate::error::Bn254Err;
use alloy_primitives::U256;
use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ff::{BigInteger, BigInteger256};
use std::ops::Mul;
use std::str::FromStr;

/// Converts [U256] to [BigInteger256]
pub fn u256_to_bigint256(value: U256) -> BigInteger256 {
    // Convert U256 to a big-endian byte array
    let bytes: [u8; 32] = value.to_be_bytes();

    // BigInteger256 expects a 4-element array of 64-bit values in little-endian order
    let mut data = [0u64; 4];

    // Iterate over the bytes in chunks of 8 bytes and convert to u64
    for (i, chunk) in bytes.chunks(8).enumerate() {
        let mut chunk_array = [0u8; 8];
        chunk_array.copy_from_slice(chunk);
        data[3 - i] = u64::from_be_bytes(chunk_array);
    }

    BigInteger256::new(data)
}

pub fn biginteger256_to_u256(bi: BigInteger256) -> U256 {
    let s = bi.to_bytes_be();
    U256::from_be_slice(&s)
}

pub fn get_g1_generator() -> Result<G1Affine, Bn254Err> {
    let x_result = Fq::from_str("1");

    let y_result = Fq::from_str("2");

    match x_result {
        Ok(x) => match y_result {
            Ok(y) => {
                return Ok(G1Affine::new(x, y));
            }
            Err(_) => return Err(Bn254Err::Fq),
        },
        Err(_) => return Err(Bn254Err::Fq),
    }
}

pub fn get_g2_generator() -> Result<G2Affine, Bn254Err> {
    let x_0_result = Fq::from_str(
        "10857046999023057135944570762232829481370756359578518086990519993285655852781",
    );

    let x_1result = Fq::from_str(
        "11559732032986387107991004021392285783925812861821192530917403151452391805634",
    );

    match x_0_result {
        Ok(x_0) => {
            match x_1result {
                Ok(x_1) => {
                    let x = Fq2::new(x_0, x_1);

                    let y_0_result = Fq::from_str("8495653923123431417604973247489272438418190587263600148770280649306958101930");

                    match y_0_result {
                        Ok(y_0) => {
                            let y_1_result = Fq::from_str("4082367875863433681332203403145435568316851327593401208105741076214120093531");

                            match y_1_result {
                                Ok(y_1) => {
                                    let y = Fq2::new(y_0, y_1);
                                    return Ok(G2Affine::new(x, y));
                                }
                                Err(_) => return Err(Bn254Err::Fq),
                            }
                        }
                        Err(_) => {
                            return Err(Bn254Err::Fq);
                        }
                    }
                }
                Err(_) => return Err(Bn254Err::Fq),
            }
        }
        Err(_) => {
            return Err(Bn254Err::Fq);
        }
    }
}

pub fn mul_by_generator_g1(pvt_key: Fr) -> Result<G1Projective, Bn254Err> {
    let g1_gen_result = get_g1_generator();

    match g1_gen_result {
        Ok(g1_gen) => {
            let s: G1Projective = g1_gen.into();
            return Ok(s.mul(pvt_key));
        }
        Err(_) => {
            return Err(Bn254Err::Fq);
        }
    }
}

pub fn mul_by_generator_g2(pvt_key: Fr) -> Result<G2Projective, Bn254Err> {
    let g2_gen_result = get_g2_generator();

    match g2_gen_result {
        Ok(g2_gen) => {
            let s: G2Projective = g2_gen.into();
            return Ok(s.mul(pvt_key));
        }
        Err(_) => {
            return Err(Bn254Err::Fq);
        }
    }
}

pub fn verify_sig(sig: G1Affine, pub_key: G2Affine, msg: [u8; 32]) {
    let g2_gen = get_g1_generator().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_u256_to_bigint256() {
        let u256 = U256::from(123456789);
        let result = u256_to_bigint256(u256);
        assert_eq!(result, BigInteger256::from(123456789u32));
    }

    #[tokio::test]
    async fn test_bigint256_to_u256() {
        let bi = BigInteger256::from(123456789u32);
        let result = biginteger256_to_u256(bi);
        assert_eq!(result, U256::from(123456789));
    }
}
