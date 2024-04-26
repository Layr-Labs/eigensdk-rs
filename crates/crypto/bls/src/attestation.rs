use crate::error::BlsError;
use ark_bn254::{Fq, Fq2, Fr, G1Projective, G2Projective};
use ark_ff::{BigInteger256, One};
use eigensdk_crypto_bn254::utils::{mul_by_generator_g1, mul_by_generator_g2};
use std::ops::{Add, Mul};
pub fn new_fp_element(x: BigInteger256) -> Fq {
    Fq::from(x)
}

fn new_fp2_element(a: BigInteger256, b: BigInteger256) -> Fq2 {
    Fq2::new(Fq::from(a), Fq::from(b))
}

type PrivateKey = Fr;

pub struct Signature {
    sig: G1Projective,
}

impl Signature {
    pub fn sig(&self) -> G1Projective {
        self.sig
    }
}

pub struct KeyPair {
    priv_key: PrivateKey,
    pub_key: G1Projective,
}

impl KeyPair {
    pub fn new(key: PrivateKey) -> Result<Self, BlsError> {
        let priv_key_projective_cconfig_result = mul_by_generator_g1(key);

        match priv_key_projective_cconfig_result {
            Ok(priv_key_projective_cconfig) => {
                return Ok(Self {
                    priv_key: key,
                    pub_key: priv_key_projective_cconfig,
                });
            }
            Err(_) => return Err(BlsError::MulByG1Projective),
        }
    }

    pub fn sign_hashes_to_curve_message(&self, g1_hashes_msg: G1Projective) -> Signature {
        let sig = g1_hashes_msg.mul(self.priv_key);

        Signature { sig }
    }

    pub fn get_pub_key_g1(&self) -> G1Projective {
        self.pub_key
    }

    pub fn gt_pub_key_g2(&self) -> Result<G2Projective, BlsError> {
        let mul_result = mul_by_generator_g2(self.priv_key);

        match mul_result {
            Ok(mul) => Ok(mul),
            Err(_) => return Err(BlsError::MulByG2Projective),
        }
    }
}

#[derive(Debug, Clone)]
pub struct G1Point {
    pub point: G1Projective,
}

#[derive(Debug, Clone)]
pub struct G2Point {
    pub point: G2Projective,
}

impl G2Point {
    // Function to create a new G2Point from x and y coordinates, where each coordinate is a pair of BigIntegers
    pub fn new(x: (BigInteger256, BigInteger256), y: (BigInteger256, BigInteger256)) -> Self {
        // Convert x and y to Fq2 elements
        let x_elem = new_fp2_element(x.1, x.0);
        let y_elem = new_fp2_element(y.1, y.0);

        // Create a new G2 point in projective coordinates
        let point = G2Projective::new(x_elem, y_elem, Fq2::one()); // Z coordinate is set to 1

        G2Point { point }
    }
}

impl G1Point {
    // Function to create a new G1Point from x and y coordinates
    pub fn new(x: BigInteger256, y: BigInteger256) -> Self {
        // Convert x and y to field elements
        let x_elem = new_fp_element(x);
        let y_elem = new_fp_element(y);

        // Create a new G1 point in projective coordinates
        let point = G1Projective::new(x_elem, y_elem, Fq::one()); // Z coordinate is set to 1

        G1Point { point }
    }

    pub fn add(&mut self, p2: G1Point) -> G1Point {
        let added_point = self.point.add(p2.point);
        G1Point { point: added_point }
    }
}
