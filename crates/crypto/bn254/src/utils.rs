use ark_bn254::{Fq, G1Affine, G1Projective, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{
    fields::{Field, PrimeField},
    One,
};
use rust_bls_bn254::pairing;

/// MapToCurve implements the simple hash-and-check (also sometimes try-and-increment) algorithm
/// see https://hackmd.io/@benjaminion/bls12-381#Hash-and-check
/// Note that this function needs to be the same as the one used in the contract:
/// https://github.com/Layr-Labs/eigenlayer-middleware/blob/1feb6ae7e12f33ce8eefb361edb69ee26c118b5d/src/libraries/BN254.sol#L292
/// we don't use the newer constant time hash-to-curve algorithms as they are gas-expensive to compute onchain
pub fn map_to_curve(digest: &[u8]) -> G1Affine {
    let one = Fq::one();
    let three = Fq::from(3u64);
    let mut bytes = [0u8; 32];

    // This adds padding to the digest to make it 32 bytes. Go SDK automatically does it.
    // This makes HashToCurve same for both Rust and Go SDK.
    bytes[..digest.len()].copy_from_slice(&digest[..]);

    let mut x = Fq::from_be_bytes_mod_order(&bytes);

    loop {
        // y = x^3 + 3
        let mut y = x;
        y.square_in_place();
        y *= x;
        y += three;

        // Check if y is a quadratic residue (i.e., has a square root in the field)
        if let Some(y) = y.sqrt() {
            return G1Projective::new(x, y, Fq::one()).into_affine();
        } else {
            // x = x + 1
            x += one;
        }
    }
}

/// Verifies message on G2
pub fn verify_message(public_key: G2Affine, message: &[u8], signature: G1Affine) -> bool {
    if !signature.is_in_correct_subgroup_assuming_on_curve() || !signature.is_on_curve() {
        return false;
    }

    let q = map_to_curve(message);
    let c1 = pairing(public_key, q);
    let c2 = pairing(G2Affine::generator(), signature);
    c1 == c2
}
