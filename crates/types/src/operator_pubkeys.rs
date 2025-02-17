use alloy::primitives::U256;
use ark_ff::PrimeField;
use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair};
use eigen_utils::slashing::middleware::blsapkregistry::BN254::{G1Point, G2Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPubKeys {
    pub g1_pub_key: BlsG1Point,
    pub g2_pub_key: BlsG2Point,
}

impl From<BlsKeyPair> for OperatorPubKeys {
    fn from(keypair: BlsKeyPair) -> Self {
        Self {
            g1_pub_key: keypair.public_key(),
            g2_pub_key: keypair.public_key_g2(),
        }
    }
}

impl OperatorPubKeys {
    pub fn to_contract_public_keys(&self) -> (G1Point, G2Point) {
        let x = self.g1_pub_key.g1().x;
        let y = self.g1_pub_key.g1().y;
        let g1 = G1Point {
            X: U256::from_limbs(x.into_bigint().0),
            Y: U256::from_limbs(y.into_bigint().0),
        };

        let x2 = self.g2_pub_key.g2().x;
        let y2 = self.g2_pub_key.g2().y;
        let g2 = G2Point {
            X: [
                U256::from_limbs(x2.c0.into_bigint().0),
                U256::from_limbs(x2.c1.into_bigint().0),
            ],
            Y: [
                U256::from_limbs(y2.c0.into_bigint().0),
                U256::from_limbs(y2.c1.into_bigint().0),
            ],
        };
        (g1, g2)
    }
}
