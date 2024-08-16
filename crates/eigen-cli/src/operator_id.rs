use alloy_primitives::keccak256;
use ark_ec::CurveGroup;
use eigen_crypto_bls::{error::BlsError, BlsKeyPair};

/// Derives an operator ID from a private key
///
/// # Arguments
///
/// * `private_key` - A private key used to derive the operator ID
///
/// # Returns
///
/// * The operator ID as `String`
///
/// # Errors
///
/// * If the private key is not valid
pub fn derive_operator_id(private_key: String) -> Result<String, BlsError> {
    let key_pair = BlsKeyPair::new(private_key)?;
    let pub_key = key_pair.public_key();
    let pub_key_affine = pub_key.g1();

    let x_int: num_bigint::BigUint = pub_key_affine.x.into();
    let y_int: num_bigint::BigUint = pub_key_affine.y.into();

    let x_bytes = x_int.to_bytes_be();
    let y_bytes = y_int.to_bytes_be();

    let hash = keccak256([x_bytes, y_bytes].concat());
    Ok(hex::encode(hash))
}
