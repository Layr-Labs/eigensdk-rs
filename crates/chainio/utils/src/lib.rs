use ark_bn254::G2Projective;
use ark_bn254::{Fq, G1Projective};
use ark_ff::BigInteger256;
use eigensdk_contracts_bindings::BLSApkRegistry::bls_apk_registry::{
    G1Point as BlsG1Point, G2Point as BlsG2Point,
};
// use eigensdk_
use eigensdk_contracts_bindings::RegistryCoordinator;
use eigensdk_crypto_bls::attestation::{new_fp_element, G1Point, G2Point};
use eigensdk_crypto_bn254::utils::biginteger256_to_u256;
use eigensdk_crypto_bn254::utils::u256_to_bigint256;

pub fn convert_bn254_to_ark(g1_point: RegistryCoordinator::G1Point) -> G1Point {
    G1Point::new(u256_to_bigint256(g1_point.x), u256_to_bigint256(g1_point.y))
}

pub fn convert_to_bn254_g1_point(g1: G1Projective) -> BlsG1Point {
    let x: BigInteger256 = g1.x.into();
    let y: BigInteger256 = g1.y.into();

    BlsG1Point {
        x: biginteger256_to_u256(x),
        y: biginteger256_to_u256(y),
    }
}

pub fn convert_to_bn254_g2_point(g2: G2Projective) -> BlsG2Point {
    let x_0: BigInteger256 = g2.x.c0.into();
    let x_1: BigInteger256 = g2.x.c1.into();
    let y_0: BigInteger256 = g2.y.c0.into();
    let y_1: BigInteger256 = g2.y.c1.into();

    BlsG2Point {
        x: [biginteger256_to_u256(x_0), biginteger256_to_u256(x_1)],
        y: [biginteger256_to_u256(y_0), biginteger256_to_u256(y_1)],
    }
}
