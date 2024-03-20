use ark_bn254::Fq;
use eigensdk_contracts_bindings::RegistryCoordinator;
use eigensdk_crypto_bls::attestation::{new_fp_element, G1Point,G2Point};
use eigensdk_contracts_bindings::BLSApkRegistry::bls_apk_registry::{G1Point as BlsG1Point};
use eigensdk_crypto_bn254::utils::u256_to_bigint256;
use ethers::core::types::U256;

pub fn convert_bn254_to_ark(g1_point: RegistryCoordinator::G1Point) -> G1Point {
    G1Point::new(u256_to_bigint256(g1_point.x), u256_to_bigint256(g1_point.y))
}


pub fn convert_to_bn254_g2_point(input : G2Point) -> BlsG1Point{

    BlsG1Point{
        x : fq_to_u256(input.point.x),
        y : input.point.y
    }



}

// Function to convert an Fq element to U256
fn fq_to_u256(fq: &Fq) -> U256 {
    let mut bytes = [0u8; 32];
    fq.write(&mut bytes[..]).expect("Failed to write Fq to bytes");
    U256::from_big_endian(&bytes)
}