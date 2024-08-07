use alloy_primitives::keccak256;
use ark_ec::CurveGroup;
use eigen_crypto_bls::attestation::KeyPair;

pub fn derive_operator_id(private_key: String) {
    let key_pair = KeyPair::from_string(private_key).unwrap();
    let pub_key = key_pair.get_pub_key_g1();
    let pub_key_affine = pub_key.into_affine();

    let x_int: num_bigint::BigUint = pub_key_affine.x.into();
    let y_int: num_bigint::BigUint = pub_key_affine.y.into();

    let x_bytes = x_int.to_bytes_be();
    let y_bytes = y_int.to_bytes_be();

    let hash = keccak256([x_bytes, y_bytes].concat());
    println!("{}", hex::encode(hash));
}
