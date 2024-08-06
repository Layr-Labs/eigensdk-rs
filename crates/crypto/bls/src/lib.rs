#![doc(
    html_logo_url = "https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/supernovahs/eigen-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod attestation;

pub mod error;

use alloy_primitives::U64;
use ark_bn254::{Fq, Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{
    fields::{Field, PrimeField},
    BigInt, BigInteger, One,
};
use eigen_utils::binding::BLSApkRegistry::{G1Point, G2Point};
use ethers::{core::types::H256, types::Address, utils::keccak256};
pub type PrivateKey = Fr;
pub type PublicKey = G1Affine;
pub type BlsSignature = G1Affine;
pub type OperatorId = H256;

pub const Q_INV_VEG: u64 = 14042775128853446655;
pub const Q0: u64 = 4891460686036598785;
pub const Q1: u64 = 2896914383306846353;
pub const Q2: u64 = 13281191951274694749;
pub const Q3: u64 = 3486998266802970665;

pub fn cids_multiplication(x: Fr, y: Fr) -> Fr {
    let mut t0: u64;
    let mut t1: u64;
    let mut t2: u64;
    let mut t3: u64;
    let mut u0;
    let mut u1: u64;
    let mut u2: u64;
    let mut u3: u64;

    {
        let mut c0: u64;
        let mut c1: u64;
        let mut c2: u64;

        let v = x.0 .0[0];
        (u0, t0) = mul_64(v, y.0 .0[0]);
        (u1, t1) = mul_64(v, y.0 .0[1]);
        (u2, t2) = mul_64(v, y.0 .0[2]);
        (u3, t3) = mul_64(v, y.0 .0[3]);
        (t1, c0) = add_64(u0, t1, 0);
        (t2, c0) = add_64(u1, t2, c0);
        (t3, c0) = add_64(u2, t3, c0);
        (c2, _) = add_64(u3, 0, c0);

        let m = Q_INV_VEG * t0;

        (u0, c1) = mul_64(m, Q0);
        (_, c0) = add_64(t0, c1, 0);
        (u1, c1) = mul_64(m, Q1);
        (t0, c0) = add_64(t1, c1, c0);
        (u2, c1) = mul_64(m, Q2);
        (t1, c0) = add_64(t2, c1, c0);
        (u3, c1) = mul_64(m, Q3);

        (t2, c0) = add_64(0, c1, c0);
        (u3, _) = add_64(u3, 0, c0);
        (t0, c0) = add_64(u0, t0, 0);
        (t1, c0) = add_64(u1, t1, c0);
        (t2, c0) = add_64(u2, t2, c0);
        (c2, _) = add_64(c2, 0, c0);
        (t2, c0) = add_64(t3, t2, 0);
        (t3, _) = add_64(u3, c2, c0);
    }

    {
        let mut c0: u64;
        let mut c1: u64;
        let mut c2: u64;

        let v = x.0 .0[1];
        (u0, c1) = mul_64(v, y.0 .0[0]);
        (t0, c0) = add_64(c1, t0, 0);
        (u1, c1) = mul_64(v, y.0 .0[1]);
        (t1, c0) = add_64(c1, t1, c0);
        (u2, c1) = mul_64(v, y.0 .0[2]);
        (t2, c0) = add_64(c1, t2, c0);
        (u3, c1) = mul_64(v, y.0 .0[3]);
        (t3, c0) = add_64(c1, t3, c0);

        (c2, _) = add_64(0, 0, c0);
        (t1, c0) = add_64(u0, t1, 0);
        (t2, c0) = add_64(u1, t2, c0);
        (t3, c0) = add_64(u2, t3, c0);
        (c2, _) = add_64(u3, c2, c0);

        let m = Q_INV_VEG * t0;

        (u0, c1) = mul_64(m, Q0);
        (_, c0) = add_64(t0, c1, 0);
        (u1, c1) = mul_64(m, Q1);
        (t0, c0) = add_64(t1, c1, c0);
        (u2, c1) = mul_64(m, Q2);
        (t1, c0) = add_64(t2, c1, c0);
        (u3, c1) = mul_64(m, Q3);

        (t2, c0) = add_64(0, c1, c0);
        (u3, _) = add_64(u3, 0, c0);
        (t0, c0) = add_64(u0, t0, 0);
        (t1, c0) = add_64(u1, t1, c0);
        (t2, c0) = add_64(u2, t2, c0);
        (c2, _) = add_64(c2, 0, c0);
        (t2, c0) = add_64(t3, t2, 0);
        (t3, _) = add_64(u3, c2, c0);
    }

    {
        let mut c0: u64;
        let mut c1: u64;
        let mut c2: u64;

        let v = x.0 .0[2];

        (u0, c1) = mul_64(v, y.0 .0[0]);
        (t0, c0) = add_64(c1, t0, 0);
        (u1, c1) = mul_64(v, y.0 .0[1]);
        (t1, c0) = add_64(c1, t1, c0);
        (u2, c1) = mul_64(v, y.0 .0[2]);
        (t2, c0) = add_64(c1, t2, c0);
        (u3, c1) = mul_64(v, y.0 .0[3]);
        (t3, c0) = add_64(c1, t3, c0);

        (c2, _) = add_64(0, 0, c0);
        (t1, c0) = add_64(u0, t1, 0);
        (t2, c0) = add_64(u1, t2, c0);
        (t3, c0) = add_64(u2, t3, c0);
        (c2, _) = add_64(u3, c2, c0);

        let m = Q_INV_VEG * t0;

        (u0, c1) = mul_64(m, Q0);
        (_, c0) = add_64(t0, c1, 0);
        (u1, c1) = mul_64(m, Q1);
        (t0, c0) = add_64(t1, c1, c0);
        (u2, c1) = mul_64(m, Q2);
        (t1, c0) = add_64(t2, c1, c0);
        (u3, c1) = mul_64(m, Q3);

        (t2, c0) = add_64(0, c1, c0);
        (u3, _) = add_64(u3, 0, c0);
        (t0, c0) = add_64(u0, t0, 0);
        (t1, c0) = add_64(u1, t1, c0);
        (t2, c0) = add_64(u2, t2, c0);
        (c2, _) = add_64(c2, 0, c0);
        (t2, c0) = add_64(t3, t2, 0);
        (t3, _) = add_64(u3, c2, c0);
    }

    {
        let mut c0: u64;
        let mut c1: u64;
        let mut c2: u64;

        let v = x.0 .0[3];

        (u0, c1) = mul_64(v, y.0 .0[0]);
        (t0, c0) = add_64(c1, t0, 0);
        (u1, c1) = mul_64(v, y.0 .0[1]);
        (t1, c0) = add_64(c1, t1, c0);
        (u2, c1) = mul_64(v, y.0 .0[2]);
        (t2, c0) = add_64(c1, t2, c0);
        (u3, c1) = mul_64(v, y.0 .0[3]);
        (t3, c0) = add_64(c1, t3, c0);

        (c2, _) = add_64(0, 0, c0);
        (t1, c0) = add_64(u0, t1, 0);
        (t2, c0) = add_64(u1, t2, c0);
        (t3, c0) = add_64(u2, t3, c0);
        (c2, _) = add_64(u3, c2, c0);

        let m = Q_INV_VEG * t0;

        (u0, c1) = mul_64(m, Q0);
        (_, c0) = add_64(t0, c1, 0);
        (u1, c1) = mul_64(m, Q1);
        (t0, c0) = add_64(t1, c1, c0);
        (u2, c1) = mul_64(m, Q2);
        (t1, c0) = add_64(t2, c1, c0);
        (u3, c1) = mul_64(m, Q3);

        (t2, c0) = add_64(0, c1, c0);
        (u3, _) = add_64(u3, 0, c0);
        (t0, c0) = add_64(u0, t0, 0);
        (t1, c0) = add_64(u1, t1, c0);
        (t2, c0) = add_64(u2, t2, c0);
        (c2, _) = add_64(c2, 0, c0);
        (t2, c0) = add_64(t3, t2, 0);
        (t3, _) = add_64(u3, c2, c0);
    }

    let mut z: Fr = Fr::default();

    z.0 .0[0] = t0;
    z.0 .0[1] = t1;
    z.0 .0[2] = t2;
    z.0 .0[3] = t3;

    if !smaller_than_modulus(z) {
        let mut b: u64;
        (z.0 .0[0], b) = sub_64(z.0 .0[0], Q0, 0);
        (z.0 .0[1], b) = sub_64(z.0 .0[1], Q1, 0);
        (z.0 .0[2], b) = sub_64(z.0 .0[2], Q2, 0);
        (z.0 .0[3], b) = sub_64(z.0 .0[3], Q3, 0);
    }

    z
}

/// mul_64 https://pkg.go.dev/math/bits#Mul64
pub fn mul_64(x: u64, y: u64) -> (u64, u64) {
    let product: u128 = (x as u128) * (y as u128);
    let u1: u64 = (product >> 64) as u64;
    let t1: u64 = product as u64;
    (u1, t1)
}

/// add_64 https://pkg.go.dev/math/bits#Add64
pub fn add_64(x: u64, y: u64, carry: u64) -> (u64, u64) {
    // Perform the addition with carry
    let (sum1, carry1) = x.overflowing_add(y);
    let (sum2, carry2) = sum1.overflowing_add(carry);

    // Calculate the final carry
    let carry_out = (carry1 as u64) + (carry2 as u64);

    (sum2, carry_out)
}

pub fn sub_64(x: u64, y: u64, borrow: u64) -> (u64, u64) {
    let diff = x.wrapping_sub(y).wrapping_sub(borrow);
    let borrow_out = ((!x & y) | ((!x ^ y) & diff)) >> 63;
    (diff, borrow_out)
}

pub fn smaller_than_modulus(z: Fr) -> bool {
    return z.0 .0[3] < Q3
        || (z.0 .0[3] == Q3
            && (z.0 .0[2] < Q2
                || (z.0 .0[2] == Q2
                    && (z.0 .0[1] < Q1 || (z.0 .0[1] == Q1 && (z.0 .0[0] < Q0))))));
}

// Test vector from https://pkg.go.dev/math/bits#Mul64
#[test]
fn test_mul64() {
    let a: u64 = 9223372036854775808;
    let b: u64 = 2;

    let s = mul_64(a, b);
    assert_eq!(1, s.0);
    assert_eq!(0, s.1);

    let c: u64 = 12;
    let d: u64 = 12;

    let s = mul_64(c, d);
    assert_eq!(0, s.0);
    assert_eq!(144, s.1);
}

/// Test vector https://pkg.go.dev/math/bits#Sub64
#[test]
fn test_sub_64() {
    let a: u64 = 23;
    let b: u64 = 12;

    let s = sub_64(a, b, 0);

    assert_eq!(11, s.0);
}

/// Test vector from https://pkg.go.dev/math/bits#Add64
#[test]
fn test_add_64() {
    let x: u64 = 12;
    let y: u64 = 23;
    let z: u64 = 0;

    let s = add_64(x, y, z); // (d1,carry)

    assert_eq!(35, s.0); // d1 =35
    assert_eq!(0, s.1); // carry bit

    let a: u64 = 33;
    let b: u64 = 21;

    let ss = add_64(a, b, s.1); // (d0,_)

    assert_eq!(ss.0, 54);
}
