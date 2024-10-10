#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use alloy_primitives::{B256, U256};
use ark_std::str::FromStr;
pub mod error;

use crate::error::BlsError;
use ark_bn254::{g1::G1Affine, Fq, Fr, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{fields::PrimeField, BigInt, BigInteger256, Fp2};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use eigen_crypto_bn254::utils::map_to_curve;
use eigen_utils::blsapkregistry::BN254::{G1Point as G1PointRegistry, G2Point as G2PointRegistry};
use eigen_utils::iblssignaturechecker::BN254::{
    G1Point as G1PointChecker, G2Point as G2PointChecker,
};
use eigen_utils::registrycoordinator::BN254::{G1Point, G2Point};
use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
pub type PrivateKey = Fr;
pub type PublicKey = G1Affine;
pub type BlsSignature = G1Affine;
pub type OperatorId = B256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlsG1Point {
    g1: G1Affine,
}

impl Serialize for BlsG1Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buffer = Vec::new();
        self.g1().serialize_uncompressed(&mut buffer).unwrap();
        serializer.serialize_bytes(&buffer)
    }
}

impl<'de> Deserialize<'de> for BlsG1Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BlsG1PointVisitor;

        impl<'de> Visitor<'de> for BlsG1PointVisitor {
            type Value = BlsG1Point;

            fn expecting(&self, formatter: &mut ark_std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a byte array representing a G1Affine point")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut buffer = Vec::new();

                while let Some(value) = seq.next_element()? {
                    buffer.push(value);
                }

                let g1 = G1Affine::deserialize_uncompressed(&*buffer).map_err(de::Error::custom)?;
                Ok(BlsG1Point { g1 })
            }
        }

        deserializer.deserialize_seq(BlsG1PointVisitor)
    }
}

impl BlsG1Point {
    pub fn new(g1: G1Affine) -> Self {
        Self { g1 }
    }

    pub fn g1(&self) -> G1Affine {
        self.g1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlsG2Point {
    g2: G2Affine,
}

impl BlsG2Point {
    pub fn new(g2: G2Affine) -> Self {
        Self { g2 }
    }

    pub fn g2(&self) -> G2Affine {
        self.g2
    }
}

impl Serialize for BlsG2Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buffer = Vec::new();
        self.g2().serialize_uncompressed(&mut buffer).unwrap();
        serializer.serialize_bytes(&buffer)
    }
}

impl<'de> Deserialize<'de> for BlsG2Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BlsG2PointVisitor;

        impl<'de> Visitor<'de> for BlsG2PointVisitor {
            type Value = BlsG2Point;

            fn expecting(&self, formatter: &mut ark_std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a byte array representing a G1Affine point")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut buffer = Vec::new();

                while let Some(value) = seq.next_element()? {
                    buffer.push(value);
                }

                let g2 = G2Affine::deserialize_uncompressed(&*buffer).map_err(de::Error::custom)?;
                Ok(BlsG2Point { g2 })
            }
        }

        deserializer.deserialize_seq(BlsG2PointVisitor)
    }
}

/// Bls key pair with public key on G1
#[derive(Debug, Clone)]
pub struct BlsKeyPair {
    /// Private Key
    priv_key: Fr,
    /// Public Key on G1
    pub_key: BlsG1Point,
}

impl BlsKeyPair {
    /// Input [`Fr`] as a [`String`]
    pub fn new(fr: String) -> Result<Self, BlsError> {
        let sk = Fr::from_str(&fr).map_err(|_| BlsError::InvalidBlsPrivateKey)?;
        let pk = G1Projective::from(G1Affine::generator()) * sk;
        Ok(Self {
            priv_key: sk,
            pub_key: BlsG1Point::new(pk.into_affine()),
        })
    }

    /// Get public key on G1
    pub fn public_key(&self) -> BlsG1Point {
        self.pub_key.clone()
    }

    pub fn sign_hashed_to_curve_message(&self, g1_hashed_msg: G1Affine) -> Signature {
        let sk_int: BigInteger256 = self.priv_key.into();
        let r = g1_hashed_msg.mul_bigint(sk_int);
        Signature::new(r.into_affine())
    }

    pub fn sign_message(&self, message: &[u8]) -> Signature {
        let g1 = map_to_curve(message);
        let sk_int: BigInteger256 = self.priv_key.into();
        let r = g1.mul_bigint(sk_int);
        Signature::new(r.into_affine())
    }

    /// Get public key on G2
    pub fn public_key_g2(&self) -> BlsG2Point {
        let pk = G2Projective::from(G2Affine::generator()) * self.priv_key;
        BlsG2Point::new(pk.into_affine())
    }
}

/// Convert [`G1Point`] to [`G1Affine`]
pub fn alloy_g1_point_to_g1_affine(g1_point: G1Point) -> G1Affine {
    let x_point = g1_point.X.into_limbs();
    let x = Fq::new(BigInteger256::new(x_point));
    let y_point = g1_point.Y.into_limbs();
    let y = Fq::new(BigInteger256::new(y_point));
    G1Affine::new(x, y)
}

/// Convert [`G1Affine`] to  Alloy [`G1Point`]
pub fn convert_to_g1_point(g1: G1Affine) -> Result<G1Point, BlsError> {
    let x_point_result = g1.x();
    let y_point_result = g1.y();

    let (Some(x_point), Some(y_point)) = (x_point_result, y_point_result) else {
        return Err(BlsError::InvalidG1Affine);
    };

    let x = BigInt::new(x_point.into_bigint().0);
    let y = BigInt::new(y_point.into_bigint().0);

    let x_u256 = U256::from_limbs(x.0);
    let y_u256 = U256::from_limbs(y.0);

    Ok(G1Point {
        X: x_u256,
        Y: y_u256,
    })
}

/// Convert [`G1Affine`] to  Alloy [`G1PointChecker`]
pub fn convert_to_bls_checker_g1_point(g1: G1Affine) -> Result<G1PointChecker, BlsError> {
    let x_point_result = g1.x();
    let y_point_result = g1.y();

    let (Some(x_point), Some(y_point)) = (x_point_result, y_point_result) else {
        return Err(BlsError::InvalidG1Affine);
    };
    let x = BigInt::new(x_point.into_bigint().0);
    let y = BigInt::new(y_point.into_bigint().0);

    let x_u256 = U256::from_limbs(x.0);
    let y_u256 = U256::from_limbs(y.0);

    Ok(G1PointChecker {
        X: x_u256,
        Y: y_u256,
    })
}

/// Convert [`G2Affine`] to  Alloy [`G2PointChecker`]
pub fn convert_to_bls_checker_g2_point(g2: G2Affine) -> Result<G2PointChecker, BlsError> {
    let x_point_result = g2.x();
    let y_point_result = g2.y();

    let (Some(x_point), Some(y_point)) = (x_point_result, y_point_result) else {
        return Err(BlsError::InvalidG2Affine);
    };
    let x_point_c0 = x_point.c0;
    let x_point_c1 = x_point.c1;
    let y_point_c0 = y_point.c0;
    let y_point_c1 = y_point.c1;

    let x_0 = BigInt::new(x_point_c0.into_bigint().0);
    let x_1 = BigInt::new(x_point_c1.into_bigint().0);
    let y_0 = BigInt::new(y_point_c0.into_bigint().0);
    let y_1 = BigInt::new(y_point_c1.into_bigint().0);

    let x_u256_0 = U256::from_limbs(x_0.0);
    let x_u256_1 = U256::from_limbs(x_1.0);
    let y_u256_0 = U256::from_limbs(y_0.0);
    let y_u256_1 = U256::from_limbs(y_1.0);

    Ok(G2PointChecker {
        X: [x_u256_1, x_u256_0],
        Y: [y_u256_1, y_u256_0],
    })
}

/// Convert [`G2Affine`] to [`G2Point`]
pub fn convert_to_g2_point(g2: G2Affine) -> Result<G2Point, BlsError> {
    let x_point_result = g2.x();
    // let x_point_c1 = g2.x().unwrap().c1;

    let y_point_result = g2.y();
    // let y_point_c1 = g2.y().unwrap().c1;

    let (Some(x_point), Some(y_point)) = (x_point_result, y_point_result) else {
        return Err(BlsError::InvalidG2Affine);
    };
    let x_point_c0 = x_point.c0;
    let x_point_c1 = x_point.c1;
    let y_point_c0 = y_point.c0;
    let y_point_c1 = y_point.c1;

    let x_0 = BigInt::new(x_point_c0.into_bigint().0);
    let x_1 = BigInt::new(x_point_c1.into_bigint().0);
    let y_0 = BigInt::new(y_point_c0.into_bigint().0);
    let y_1 = BigInt::new(y_point_c1.into_bigint().0);

    let x_u256_0 = U256::from_limbs(x_0.0);
    let x_u256_1 = U256::from_limbs(x_1.0);
    let y_u256_0 = U256::from_limbs(y_0.0);
    let y_u256_1 = U256::from_limbs(y_1.0);

    Ok(G2Point {
        X: [x_u256_1, x_u256_0],
        Y: [y_u256_1, y_u256_0],
    })
}

/// Convert [`G1PointRegistry`] to [`G1Affine`]
pub fn alloy_registry_g1_point_to_g1_affine(g1_point: G1PointRegistry) -> G1Affine {
    let x_point = g1_point.X.into_limbs();
    let x = Fq::new(BigInteger256::new(x_point));
    let y_point = g1_point.Y.into_limbs();
    let y = Fq::new(BigInteger256::new(y_point));
    G1Affine::new(x, y)
}

/// Convert [`G2PointRegistry`] to [`G2Affine`]
pub fn alloy_registry_g2_point_to_g2_affine(g2_point: G2PointRegistry) -> G2Affine {
    let x_fp2 = Fp2::new(
        BigInteger256::new(g2_point.X[1].into_limbs()).into(),
        BigInteger256::new(g2_point.X[0].into_limbs()).into(),
    );
    let y_fp2 = Fp2::new(
        BigInteger256::new(g2_point.Y[1].into_limbs()).into(),
        BigInteger256::new(g2_point.Y[0].into_limbs()).into(),
    );
    G2Affine::new(x_fp2, y_fp2)
}

/// Convert [`G2Affine`] to [`G2PointRegistry`]
pub fn convert_to_registry_g2_point(g2: G2Affine) -> Result<G2PointRegistry, BlsError> {
    let x_point_result = g2.x();
    let y_point_result = g2.y();

    if let (Some(x_point), Some(y_point)) = (x_point_result, y_point_result) {
        let x_point_c0 = x_point.c0;
        let x_point_c1 = x_point.c1;
        let y_point_c0 = y_point.c0;
        let y_point_c1 = y_point.c1;

        let x_0 = BigInt::new(x_point_c0.into_bigint().0);
        let x_1 = BigInt::new(x_point_c1.into_bigint().0);
        let y_0 = BigInt::new(y_point_c0.into_bigint().0);
        let y_1 = BigInt::new(y_point_c1.into_bigint().0);

        let x_u256_0 = U256::from_limbs(x_0.0);
        let x_u256_1 = U256::from_limbs(x_1.0);
        let y_u256_0 = U256::from_limbs(y_0.0);
        let y_u256_1 = U256::from_limbs(y_1.0);

        Ok(G2PointRegistry {
            X: [x_u256_1, x_u256_0],
            Y: [y_u256_1, y_u256_0],
        })
    } else {
        Err(BlsError::InvalidG2Affine)
    }
}

/// Signature instance on [`G1Affine`]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    g1_point: BlsG1Point,
}

impl Signature {
    pub fn new(g1: G1Affine) -> Self {
        Self {
            g1_point: BlsG1Point::new(g1),
        }
    }

    pub fn g1_point(&self) -> BlsG1Point {
        self.g1_point.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq2;
    use eigen_crypto_bn254::utils::verify_message;

    #[test]
    fn test_convert_to_g1_point() {
        let x_point = Fq::from_str(
            "17709620697113958145616918533531128159269167719799793368595970620022661612059",
        )
        .unwrap();
        let y_point = Fq::from_str(
            "9890439522434691655532127414660267222813910180198976870423582442696952349816",
        )
        .unwrap();
        let g1_affine = G1Affine::new(x_point, y_point);

        let alloy_g1_point = convert_to_g1_point(g1_affine).unwrap();
        assert_eq!(
            alloy_g1_point.X,
            U256::from_str(
                "17709620697113958145616918533531128159269167719799793368595970620022661612059"
            )
            .unwrap()
        );
        assert_eq!(
            alloy_g1_point.Y,
            U256::from_str(
                "9890439522434691655532127414660267222813910180198976870423582442696952349816"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_alloy_g1_point_to_g1_affine() {
        let alloy_g1_point = G1Point {
            X: U256::from_str(
                "17709620697113958145616918533531128159269167719799793368595970620022661612059",
            )
            .unwrap(),
            Y: U256::from_str(
                "9890439522434691655532127414660267222813910180198976870423582442696952349816",
            )
            .unwrap(),
        };

        let g1_affine = alloy_g1_point_to_g1_affine(alloy_g1_point);
        assert_eq!(
            U256::from_limbs(g1_affine.x().unwrap().into_bigint().0),
            U256::from_str(
                "17709620697113958145616918533531128159269167719799793368595970620022661612059"
            )
            .unwrap()
        );
        assert_eq!(
            U256::from_limbs(g1_affine.y().unwrap().into_bigint().0),
            U256::from_str(
                "9890439522434691655532127414660267222813910180198976870423582442696952349816"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_convert_to_g2_point() {
        let x_point_c0 = Fq::from_str(
            "6834287759893774453556191528501556195232162436167606874229072410417955767882",
        )
        .unwrap();
        let x_point_c1 = Fq::from_str(
            "15529400123788596166111036611862227541174221446291015207340396747864347375335",
        )
        .unwrap();

        let y_point_c0 = Fq::from_str(
            "7616309349481520605447660298084926776417001188005125143383153219707218450524",
        )
        .unwrap();
        let y_point_c1 = Fq::from_str(
            "19775028091101520702581412350510183088819198056772055625089714355379667714558",
        )
        .unwrap();

        let x_point = Fq2::new(x_point_c0, x_point_c1);
        let y_point = Fq2::new(y_point_c0, y_point_c1);

        let g2_affine = G2Affine::new(x_point, y_point);

        let alloy_g2_point = convert_to_g2_point(g2_affine).unwrap();
        assert_eq!(
            alloy_g2_point.X[0],
            U256::from_str(
                "15529400123788596166111036611862227541174221446291015207340396747864347375335"
            )
            .unwrap()
        );
        assert_eq!(
            alloy_g2_point.X[1],
            U256::from_str(
                "6834287759893774453556191528501556195232162436167606874229072410417955767882"
            )
            .unwrap()
        );
        assert_eq!(
            alloy_g2_point.Y[0],
            U256::from_str(
                "19775028091101520702581412350510183088819198056772055625089714355379667714558"
            )
            .unwrap()
        );
        assert_eq!(
            alloy_g2_point.Y[1],
            U256::from_str(
                "7616309349481520605447660298084926776417001188005125143383153219707218450524"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_bls_key_pair() {
        let bls_priv_key =
            "12248929636257230549931416853095037629726205319386239410403476017439825112537";
        let bls_key_pair = BlsKeyPair::new(bls_priv_key.to_string()).unwrap();

        assert_eq!(
            U256::from_limbs(bls_key_pair.public_key().g1().x().unwrap().into_bigint().0),
            U256::from_str(
                "277950648056014144722774518899051149098728246263316284984520891067822832300"
            )
            .unwrap()
        );
        assert_eq!(
            U256::from_limbs(bls_key_pair.public_key().g1().y().unwrap().into_bigint().0),
            U256::from_str(
                "16927236637669640540790285431111034664564710839671197540688155537113438534238"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_map_to_curve() {
        let message: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let g1 = map_to_curve(&message);

        assert_eq!(
            U256::from_limbs(g1.x().unwrap().into_bigint().0),
            U256::from_str(
                "455867356320691211509944977504407603390036387149619137164185182714736811811"
            )
            .unwrap()
        );
        assert_eq!(
            U256::from_limbs(g1.y().unwrap().into_bigint().0),
            U256::from_str(
                "9802125641729881429496664198939823213610051907104384160271670136040620850981"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_sign_message() {
        let message: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let bls_priv_key =
            "12248929636257230549931416853095037629726205319386239410403476017439825112537";
        let bls_key_pair = BlsKeyPair::new(bls_priv_key.to_string()).unwrap();

        let signature = bls_key_pair.sign_message(&message);
        assert_eq!(
            U256::from_limbs(signature.g1_point().g1().x().unwrap().into_bigint().0),
            U256::from_str(
                "6125087140203962697351933212367898471377426213402772883153680722977416765651"
            )
            .unwrap()
        );
        assert_eq!(
            U256::from_limbs(signature.g1_point().g1().y().unwrap().into_bigint().0),
            U256::from_str(
                "19120302240465611628345095276448175199636936878728446037184749040811421969742"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_verify_message() {
        let message: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ];
        let bls_priv_key =
            "12248929636257230549931416853095037629726205319386239410403476017439825112537";
        let bls_key_pair = BlsKeyPair::new(bls_priv_key.to_string()).unwrap();

        let signature = bls_key_pair.sign_message(&message);

        assert!(verify_message(
            bls_key_pair.public_key_g2().g2(),
            &message,
            signature.g1_point().g1()
        ));
    }

    #[test]
    fn test_alloy_registry_g2_point_to_g2_affine() {
        let registry_g2_point = G2PointRegistry {
            X: [
                U256::from_str(
                    "5757009320127825712028542414399286695979413866882055578475552905478799178978",
                )
                .unwrap(),
                U256::from_str(
                    "21244616545128868564944750577089226156588822099825362793595203506897139322148",
                )
                .unwrap(),
            ],
            Y: [
                U256::from_str(
                    "14151879035050941576498647371309462393327101480686968228451570672809612016186",
                )
                .unwrap(),
                U256::from_str(
                    "3459884663217117850014821742383597128426843416583591466170557027357262534805",
                )
                .unwrap(),
            ],
        };

        let g2_affine = alloy_registry_g2_point_to_g2_affine(registry_g2_point);
        let expected_g2_affine = G2Affine {
            x: Fq2::new(
                Fq::from_str(
                    "21244616545128868564944750577089226156588822099825362793595203506897139322148",
                )
                .unwrap(),
                Fq::from_str(
                    "5757009320127825712028542414399286695979413866882055578475552905478799178978",
                )
                .unwrap(),
            ),
            y: Fq2::new(
                Fq::from_str(
                    "3459884663217117850014821742383597128426843416583591466170557027357262534805",
                )
                .unwrap(),
                Fq::from_str(
                    "14151879035050941576498647371309462393327101480686968228451570672809612016186",
                )
                .unwrap(),
            ),
            infinity: false,
        };

        assert_eq!(g2_affine, expected_g2_affine);
    }

    #[test]
    fn test_serialize_deserialize_bls_g1_point() {
        let bls_priv_key =
            "12248929636257230549931416853095037629726205319386239410403476017439825112537";
        let bls_key_pair = BlsKeyPair::new(bls_priv_key.to_string()).unwrap();

        let original_point = bls_key_pair.public_key();
        // Serialize the BlsG1Point to a JSON string
        let serialized = serde_json::to_string(&original_point).expect("Failed to serialize");

        // Deserialize the JSON string back to a BlsG1Point
        let deserialized: BlsG1Point =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Check that the deserialized point matches the original
        assert_eq!(
            original_point, deserialized,
            "The deserialized point does not match the original"
        );
    }

    #[test]
    fn test_serialize_deserialize_bls_g2_point() {
        let bls_priv_key =
            "12248929636257230549931416853095037629726205319386239410403476017439825112537";
        let bls_key_pair = BlsKeyPair::new(bls_priv_key.to_string()).unwrap();

        let original_point = bls_key_pair.public_key_g2();
        // Serialize the BlsG2Point to a JSON string
        let serialized = serde_json::to_string(&original_point).expect("Failed to serialize");

        // Deserialize the JSON string back to a BlsG2Point
        let deserialized: BlsG2Point =
            serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Check that the deserialized point matches the original
        assert_eq!(
            original_point, deserialized,
            "The deserialized point does not match the original"
        );
    }
}
