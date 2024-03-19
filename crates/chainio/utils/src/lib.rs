use eigensdk_contracts_bindings::RegistryCoordinator;
use eigensdk_crypto_bls::attestation::{new_fp_element, G1Point};

use eigensdk_crypto_bn254::utils::u256_to_bigint256;

pub fn convert_bn254_to_ark(g1_point: RegistryCoordinator::G1Point) -> G1Point {
    G1Point::new(u256_to_bigint256(g1_point.x), u256_to_bigint256(g1_point.y))
}
